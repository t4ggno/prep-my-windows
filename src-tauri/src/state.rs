use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use parking_lot::{Mutex, MutexGuard, RwLock};
use uuid::Uuid;

use serde::Serialize;

use crate::catalog;
use crate::change_history::{self, AppliedChange, ChangeHistory, ReappliedChange};
use crate::models::{ActivityEvent, AppConfig, EngineStatus, InstalledPackage};

const PROBLEM_REPEAT_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

fn normalize_catalog_selections(config: &mut AppConfig) {
    fn sync_selections<'a>(
        selections: &mut BTreeMap<String, bool>,
        items: impl IntoIterator<Item = (&'a str, bool)>,
    ) {
        let defaults = items.into_iter().collect::<BTreeMap<_, _>>();
        selections.retain(|id, _| defaults.contains_key(id.as_str()));
        for (id, enabled) in defaults {
            selections.entry(id.to_owned()).or_insert(enabled);
        }
    }

    sync_selections(
        &mut config.policies,
        catalog::policies()
            .into_iter()
            .map(|item| (item.id, item.default_enabled))
            .chain(
                catalog::system_settings()
                    .into_iter()
                    .map(|item| (item.id, true)),
            ),
    );
    sync_selections(
        &mut config.packages,
        catalog::packages().into_iter().map(|item| (item.id, true)),
    );
    sync_selections(
        &mut config.network_blocks,
        catalog::network_blocks()
            .into_iter()
            .map(|item| (item.id, true)),
    );
    sync_selections(
        &mut config.scheduled_tasks,
        catalog::scheduled_tasks()
            .into_iter()
            .map(|item| (item.id, true)),
    );

    let policies = catalog::policies();
    config.policy_choices.retain(|id, choice| {
        policies.iter().any(|definition| {
            definition.id == id
                && definition
                    .choice(choice)
                    .is_some_and(|choice| choice.value != definition.desired)
        })
    });
}

fn write_json(path: &Path, value: &impl Serialize, label: &str) -> Result<(), String> {
    let content = serde_json::to_string_pretty(value)
        .map_err(|error| format!("Could not serialize {label}: {error}"))?;
    let temporary_path = path.with_extension("json.tmp");
    fs::write(&temporary_path, content)
        .and_then(|()| fs::rename(&temporary_path, path))
        .map_err(|error| format!("Could not save {label}: {error}"))
}

fn read_history(path: &Path) -> Result<ChangeHistory, String> {
    if !path.exists() {
        return Ok(ChangeHistory::default());
    }
    let content = fs::read_to_string(path)
        .map_err(|error| format!("Could not read change history: {error}"))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("Could not parse change history: {error}"))
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    config_path: PathBuf,
    history_path: PathBuf,
    config: RwLock<AppConfig>,
    history: Mutex<ChangeHistory>,
    status: RwLock<EngineStatus>,
    activity: RwLock<Vec<ActivityEvent>>,
    process_allowances: RwLock<HashSet<String>>,
    reported_problems: Mutex<HashMap<String, Instant>>,
    installed_packages: RwLock<Vec<InstalledPackage>>,
    enforcement: Mutex<()>,
}

impl AppState {
    pub fn load(config_directory: &Path) -> Result<Self, String> {
        fs::create_dir_all(config_directory)
            .map_err(|error| format!("Could not create {}: {error}", config_directory.display()))?;
        let config_path = config_directory.join("policy.json");
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|error| format!("Could not read profile: {error}"))?;
            serde_json::from_str(&content)
                .map_err(|error| format!("Could not parse profile: {error}"))?
        } else {
            AppConfig::default()
        };
        normalize_catalog_selections(&mut config);
        config.validate()?;
        let history_path = config_directory.join("history.json");
        let history = read_history(&history_path)?;
        let state = Self {
            inner: Arc::new(Inner {
                config_path,
                history_path,
                config: RwLock::new(config),
                history: Mutex::new(history),
                status: RwLock::new(EngineStatus::default()),
                activity: RwLock::new(Vec::new()),
                process_allowances: RwLock::new(HashSet::new()),
                reported_problems: Mutex::new(HashMap::new()),
                installed_packages: RwLock::new(Vec::new()),
                enforcement: Mutex::new(()),
            }),
        };
        state.save()?;
        Ok(state)
    }

    pub fn config(&self) -> AppConfig {
        self.inner.config.read().clone()
    }

    pub fn replace_config(&self, mut config: AppConfig) -> Result<(), String> {
        normalize_catalog_selections(&mut config);
        config.validate()?;
        let mut current = self.inner.config.write();
        self.save_config(&config)?;
        *current = config;
        Ok(())
    }

    pub fn update_config<T>(&self, update: impl FnOnce(&mut AppConfig) -> T) -> Result<T, String> {
        self.try_update_config(|config| Ok(update(config)))
    }

    pub fn try_update_config<T>(
        &self,
        update: impl FnOnce(&mut AppConfig) -> Result<T, String>,
    ) -> Result<T, String> {
        let mut current = self.inner.config.write();
        let mut updated = current.clone();
        let result = update(&mut updated)?;
        updated.validate()?;
        self.save_config(&updated)?;
        *current = updated;
        Ok(result)
    }

    pub fn save(&self) -> Result<(), String> {
        self.save_config(&self.inner.config.read())
    }

    fn save_config(&self, config: &AppConfig) -> Result<(), String> {
        write_json(&self.inner.config_path, config, "profile")
    }

    pub fn status(&self) -> EngineStatus {
        let mut status = self.inner.status.read().clone();
        status.busy = self.inner.enforcement.is_locked();
        status.reapplied_total = self.inner.history.lock().total_reverts();
        status
    }

    pub fn reapplied(&self) -> Vec<ReappliedChange> {
        self.inner.history.lock().reapplied()
    }

    pub fn record_changes(&self, changes: Vec<AppliedChange>) -> Result<(), String> {
        if changes.is_empty() {
            return Ok(());
        }
        let records = {
            let mut history = self.inner.history.lock();
            let records = history.record(changes, &Utc::now().to_rfc3339());
            write_json(&self.inner.history_path, &*history, "change history")?;
            records
        };
        for summary in change_history::summaries(&records) {
            self.add_activity_with_details("Enforcement", summary.message, summary.details, true);
        }
        Ok(())
    }

    pub fn release_changes<'a>(
        &self,
        keys: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), String> {
        let mut history = self.inner.history.lock();
        if history.release(keys) {
            write_json(&self.inner.history_path, &*history, "change history")?;
        }
        Ok(())
    }

    pub fn release_all_changes(&self) -> Result<(), String> {
        let mut history = self.inner.history.lock();
        if history.release_all() {
            write_json(&self.inner.history_path, &*history, "change history")?;
        }
        Ok(())
    }

    pub fn lock_enforcement(&self) -> MutexGuard<'_, ()> {
        self.inner.enforcement.lock()
    }

    pub fn try_lock_enforcement(&self) -> Option<MutexGuard<'_, ()>> {
        self.inner.enforcement.try_lock()
    }

    pub fn complete_enforcement(&self) {
        self.inner.status.write().last_enforced_at = Some(Utc::now().to_rfc3339());
    }

    pub fn record_kills(&self, killed: u64) {
        self.inner.status.write().killed_total += killed;
    }

    pub fn activity(&self) -> Vec<ActivityEvent> {
        self.inner.activity.read().clone()
    }

    pub fn add_activity(&self, kind: &str, message: impl Into<String>, success: bool) {
        self.add_activity_with_details(kind, message, Vec::new(), success);
    }

    fn add_activity_with_details(
        &self,
        kind: &str,
        message: impl Into<String>,
        details: Vec<String>,
        success: bool,
    ) {
        let mut activity = self.inner.activity.write();
        activity.insert(
            0,
            ActivityEvent {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now().to_rfc3339(),
                kind: kind.to_owned(),
                message: message.into(),
                details,
                success,
            },
        );
        activity.truncate(250);
    }

    pub fn report_problem(&self, message: impl Into<String>) {
        let message = message.into();
        let now = Instant::now();
        {
            let mut reported = self.inner.reported_problems.lock();
            reported.retain(|_, at| now.duration_since(*at) < PROBLEM_REPEAT_INTERVAL);
            if reported.contains_key(&message) {
                return;
            }
            reported.insert(message.clone(), now);
        }
        self.add_activity("Error", message, false);
    }

    pub fn clear_activity(&self) {
        self.inner.activity.write().clear();
        self.inner.reported_problems.lock().clear();
    }

    pub fn installed_packages(&self) -> Vec<InstalledPackage> {
        self.inner.installed_packages.read().clone()
    }

    pub fn remember_installed_packages(&self, packages: Vec<InstalledPackage>) {
        *self.inner.installed_packages.write() = packages;
    }

    pub fn allow_process_once(&self, process_key: String) {
        self.inner.process_allowances.write().insert(process_key);
    }

    pub fn consume_process_allowance(&self, process_key: &str) -> bool {
        self.inner.process_allowances.write().remove(process_key)
    }

    pub fn revoke_process_allowance(&self, process_key: &str) {
        self.inner.process_allowances.write().remove(process_key);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use uuid::Uuid;

    use super::{AppState, normalize_catalog_selections};
    use crate::models::AppConfig;

    fn temporary_directory() -> std::path::PathBuf {
        std::env::temp_dir().join(format!("prep-my-windows-test-{}", Uuid::new_v4()))
    }

    #[test]
    fn syncs_catalog_selections_without_overwriting_preferences() {
        let mut config = AppConfig::default();
        config.policies.remove("location-sensor");
        config.policies.remove("hibernation");
        config.policies.remove("start-recent-apps");
        config.policies.remove("taskbar-alignment");
        config.policies.insert("hidden-files".to_owned(), true);
        config.policies.insert("location".to_owned(), false);
        config.policies.insert("removed-setting".to_owned(), true);
        config.packages.remove("candy-crush");
        config.packages.insert("clipchamp".to_owned(), false);
        config.scheduled_tasks.remove("device-census");
        config.network_blocks.remove("events");

        normalize_catalog_selections(&mut config);

        assert_eq!(config.policies.get("location-sensor"), Some(&true));
        assert_eq!(config.policies.get("hibernation"), Some(&true));
        assert_eq!(config.policies.get("start-recent-apps"), Some(&true));
        assert_eq!(config.policies.get("taskbar-alignment"), Some(&false));
        assert_eq!(config.policies.get("hidden-files"), Some(&true));
        assert_eq!(config.policies.get("location"), Some(&false));
        assert_eq!(config.policies.get("removed-setting"), None);
        assert_eq!(config.packages.get("candy-crush"), Some(&true));
        assert_eq!(config.packages.get("clipchamp"), Some(&false));
        assert_eq!(config.scheduled_tasks.get("device-census"), Some(&true));
        assert_eq!(config.network_blocks.get("events"), Some(&true));
    }

    #[test]
    fn keeps_only_valid_non_default_policy_choices() {
        let mut config = AppConfig::default();
        for (id, choice) in [
            ("diagnostic-data", "required"),
            ("defender-sample-submission", "never"),
            ("location", "unknown"),
            ("removed-setting", "off"),
        ] {
            config
                .policy_choices
                .insert(id.to_owned(), choice.to_owned());
        }

        normalize_catalog_selections(&mut config);

        assert_eq!(
            config.policy_choices.into_iter().collect::<Vec<_>>(),
            [("diagnostic-data".to_owned(), "required".to_owned())]
        );
    }

    #[test]
    fn loads_existing_profiles_with_new_defaults_enabled() {
        let directory = temporary_directory();
        fs::create_dir_all(&directory).unwrap();
        let mut profile = serde_json::to_value(AppConfig::default()).unwrap();
        let profile_object = profile.as_object_mut().unwrap();
        profile_object.remove("activeHoursStart");
        profile_object.remove("activeHoursEnd");
        profile_object
            .get_mut("policies")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove("hibernation");
        fs::write(
            directory.join("policy.json"),
            serde_json::to_string_pretty(&profile).unwrap(),
        )
        .unwrap();

        let state = AppState::load(&directory).unwrap();
        let config = state.config();

        assert_eq!(config.active_hours_start, 22);
        assert_eq!(config.active_hours_end, 7);
        assert_eq!(config.policies.get("hibernation"), Some(&true));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn process_notification_preferences_survive_reload() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();
        state
            .update_config(|config| {
                config
                    .muted_process_notifications
                    .insert("built-in-teams".to_owned());
                config.process_rules[0].enabled = false;
            })
            .unwrap();

        let reloaded = AppState::load(&directory).unwrap().config();
        assert!(
            reloaded
                .muted_process_notifications
                .contains("built-in-teams")
        );
        assert!(
            reloaded
                .process_rules
                .iter()
                .find(|rule| rule.id == "built-in-teams")
                .unwrap()
                .enabled
        );
        assert!(!reloaded.process_rules[0].enabled);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn new_profiles_start_with_enforcement_disabled() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();
        assert!(!state.config().enforcement_enabled);

        state
            .update_config(|config| config.enforcement_enabled = true)
            .unwrap();

        assert!(
            AppState::load(&directory)
                .unwrap()
                .config()
                .enforcement_enabled
        );
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn process_allowances_are_consumed_once() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();

        state.allow_process_once("app.exe".to_owned());

        assert!(state.consume_process_allowance("app.exe"));
        assert!(!state.consume_process_allowance("app.exe"));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn repeated_problems_are_logged_once() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();

        state.report_problem("A");
        state.report_problem("A");
        state.report_problem("B");
        assert_eq!(state.activity().len(), 2);
        assert!(state.activity().iter().all(|event| !event.success));

        state.clear_activity();
        state.report_problem("A");
        assert_eq!(state.activity().len(), 1);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rejected_updates_do_not_change_state() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();

        let result = state.try_update_config(|config| {
            config.start_with_windows = false;
            Err::<(), _>("Rejected update".to_owned())
        });

        assert_eq!(result, Err("Rejected update".to_owned()));
        assert!(state.config().start_with_windows);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rolls_back_memory_when_profile_cannot_be_saved() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();
        let profile_path = directory.join("policy.json");
        fs::remove_file(&profile_path).unwrap();
        fs::create_dir(&profile_path).unwrap();

        assert!(
            state
                .update_config(|config| config.start_with_windows = false)
                .is_err()
        );
        assert!(state.config().start_with_windows);
        fs::remove_dir_all(directory).unwrap();
    }
}
