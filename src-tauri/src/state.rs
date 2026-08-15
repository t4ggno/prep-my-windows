use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;

use crate::catalog;
use crate::models::{ActivityEvent, AppConfig, EngineStatus};

fn add_missing_policy_defaults(config: &mut AppConfig) {
    for definition in catalog::policies() {
        config
            .policies
            .entry(definition.id.to_owned())
            .or_insert(true);
    }
    for definition in catalog::system_settings() {
        config
            .policies
            .entry(definition.id.to_owned())
            .or_insert(true);
    }
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    config_path: PathBuf,
    config: RwLock<AppConfig>,
    status: RwLock<EngineStatus>,
    activity: RwLock<Vec<ActivityEvent>>,
    process_allowances: RwLock<HashSet<String>>,
    busy: AtomicBool,
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
        add_missing_policy_defaults(&mut config);
        config.validate()?;
        let state = Self {
            inner: Arc::new(Inner {
                config_path,
                config: RwLock::new(config),
                status: RwLock::new(EngineStatus {
                    running: true,
                    ..EngineStatus::default()
                }),
                activity: RwLock::new(Vec::new()),
                process_allowances: RwLock::new(HashSet::new()),
                busy: AtomicBool::new(false),
            }),
        };
        state.save()?;
        Ok(state)
    }

    pub fn config(&self) -> AppConfig {
        self.inner.config.read().clone()
    }

    pub fn replace_config(&self, mut config: AppConfig) -> Result<(), String> {
        add_missing_policy_defaults(&mut config);
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
        let content = serde_json::to_string_pretty(config)
            .map_err(|error| format!("Could not serialize profile: {error}"))?;
        fs::write(&self.inner.config_path, content)
            .map_err(|error| format!("Could not save profile: {error}"))
    }

    pub fn status(&self) -> EngineStatus {
        let mut status = self.inner.status.read().clone();
        status.busy = self.inner.busy.load(Ordering::Relaxed);
        status
    }

    pub fn begin_enforcement(&self) -> bool {
        self.inner
            .busy
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    pub fn finish_enforcement(&self) {
        self.inner.busy.store(false, Ordering::Release);
    }

    pub fn complete_enforcement(&self, repaired: u64, killed: u64, last_error: Option<String>) {
        let mut status = self.inner.status.write();
        status.last_enforced_at = Some(Utc::now().to_rfc3339());
        status.repaired_total += repaired;
        status.killed_total += killed;
        status.last_error = last_error;
    }

    pub fn record_kills(&self, killed: u64) {
        self.inner.status.write().killed_total += killed;
    }

    pub fn activity(&self) -> Vec<ActivityEvent> {
        self.inner.activity.read().clone()
    }

    pub fn add_activity(&self, kind: &str, message: impl Into<String>, success: bool) {
        let mut activity = self.inner.activity.write();
        activity.insert(
            0,
            ActivityEvent {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now().to_rfc3339(),
                kind: kind.to_owned(),
                message: message.into(),
                success,
            },
        );
        activity.truncate(250);
    }

    pub fn clear_activity(&self) {
        self.inner.activity.write().clear();
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

    use super::{AppState, add_missing_policy_defaults};
    use crate::models::AppConfig;

    fn temporary_directory() -> std::path::PathBuf {
        std::env::temp_dir().join(format!("prep-my-windows-test-{}", Uuid::new_v4()))
    }

    #[test]
    fn adds_new_policy_defaults_without_overwriting_preferences() {
        let mut config = AppConfig::default();
        config.policies.remove("location-sensor");
        config.policies.remove("hibernation");
        config.policies.remove("start-recent-apps");
        config.policies.insert("location".to_owned(), false);

        add_missing_policy_defaults(&mut config);

        assert_eq!(config.policies.get("location-sensor"), Some(&true));
        assert_eq!(config.policies.get("hibernation"), Some(&true));
        assert_eq!(config.policies.get("start-recent-apps"), Some(&true));
        assert_eq!(config.policies.get("location"), Some(&false));
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
    fn process_allowances_are_consumed_once() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();

        state.allow_process_once("app.exe".to_owned());

        assert!(state.consume_process_allowance("app.exe"));
        assert!(!state.consume_process_allowance("app.exe"));
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
