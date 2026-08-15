use std::time::Duration;

use tauri::AppHandle;

use crate::catalog;
use crate::events::publish_live_state;
use crate::models::{SupportRequirement, SystemSupport};
use crate::process_enforcement;
use crate::state::AppState;
use crate::windows::{
    self, autostart, hosts, packages, registry, startup_task, system_settings, tasks,
};

const RUNTIME_RECONCILIATION_INTERVAL: Duration = Duration::from_secs(60 * 60);
const SLOW_RECONCILIATION_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

struct BusyGuard(AppState);

impl BusyGuard {
    fn acquire(state: &AppState) -> Result<Self, String> {
        if state.begin_enforcement() {
            Ok(Self(state.clone()))
        } else {
            Err("Enforcement is already running".to_owned())
        }
    }
}

impl Drop for BusyGuard {
    fn drop(&mut self) {
        self.0.finish_enforcement();
    }
}

fn report_errors(state: &AppState, errors: &[String]) {
    for error in errors {
        state.add_activity("Error", error, false);
    }
}

#[derive(Default)]
struct EnforcementOutcome {
    repaired: u64,
    errors: Vec<String>,
}

impl EnforcementOutcome {
    fn merge(&mut self, other: Self) {
        self.repaired += other.repaired;
        self.errors.extend(other.errors);
    }

    fn complete(self, state: &AppState) -> Result<u64, String> {
        report_errors(state, &self.errors);
        state.complete_enforcement(self.repaired, 0, self.errors.last().cloned());
        if self.errors.is_empty() {
            Ok(self.repaired)
        } else {
            Err(self.errors.join("\n"))
        }
    }
}

fn is_supported(requirement: SupportRequirement, support: &Result<SystemSupport, String>) -> bool {
    if requirement == SupportRequirement::any() {
        return true;
    }
    support
        .as_ref()
        .is_ok_and(|system| requirement.unavailable_reason(system).is_none())
}

fn enforce_runtime_inner(state: &AppState) -> EnforcementOutcome {
    let config = state.config();
    let mut repaired = 0_u64;
    let mut errors = Vec::new();
    let mut repaired_policy_names = Vec::new();
    let support = windows::support_info();
    if let Err(error) = &support {
        errors.push(error.clone());
    }

    for definition in catalog::configured_policies(&config) {
        if !config.policies.get(definition.id).copied().unwrap_or(false) {
            continue;
        }
        if !is_supported(definition.support, &support) {
            continue;
        }
        match registry::enforce_policy(&definition) {
            Ok(true) => {
                repaired += 1;
                repaired_policy_names.push(definition.name);
            }
            Ok(false) => {}
            Err(error) => errors.push(error),
        }
    }

    for definition in catalog::system_settings() {
        if !config.policies.get(definition.id).copied().unwrap_or(false) {
            continue;
        }
        if !is_supported(definition.support, &support) {
            continue;
        }
        match system_settings::enforce(definition.kind) {
            Ok(true) => {
                repaired += 1;
                repaired_policy_names.push(definition.name);
            }
            Ok(false) => {}
            Err(error) => errors.push(error),
        }
    }

    if repaired_policy_names
        .iter()
        .any(|name| name.contains("Connected User Experiences"))
    {
        let _ = crate::windows::run_hidden("sc.exe", &["stop", "DiagTrack"]);
    }
    if repaired_policy_names
        .iter()
        .any(|name| name.contains("push service"))
    {
        let _ = crate::windows::run_hidden("sc.exe", &["stop", "dmwappushservice"]);
    }

    let enabled_network = catalog::network_blocks()
        .into_iter()
        .filter(|definition| {
            config
                .network_blocks
                .get(definition.id)
                .copied()
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    match hosts::enforce(&enabled_network) {
        Ok(true) => repaired += 1,
        Ok(false) => {}
        Err(error) => errors.push(error),
    }

    let mut autostart_repairs = 0;
    for rule in &config.blocked_autostarts {
        match autostart::enforce_rule(rule) {
            Ok(true) => {
                repaired += 1;
                autostart_repairs += 1;
            }
            Ok(false) => {}
            Err(error) => errors.push(error),
        }
    }

    if !repaired_policy_names.is_empty() {
        state.add_activity(
            "Policy",
            format!("Reapplied {} settings", repaired_policy_names.len()),
            true,
        );
    }
    if autostart_repairs > 0 {
        state.add_activity(
            "Autostart",
            format!("Removed {autostart_repairs} autostart entries"),
            true,
        );
    }
    EnforcementOutcome { repaired, errors }
}

pub fn enforce_runtime(state: &AppState) -> Result<u64, String> {
    let _guard = BusyGuard::acquire(state)?;
    enforce_runtime_inner(state).complete(state)
}

fn enforce_slow_inner(state: &AppState) -> EnforcementOutcome {
    let config = state.config();
    let mut repaired = 0_u64;
    let mut errors = Vec::new();

    let task_definitions = catalog::scheduled_tasks();
    let task_states = match tasks::states(&task_definitions) {
        Ok(states) => states,
        Err(error) => {
            errors.push(error);
            Default::default()
        }
    };
    for definition in task_definitions {
        if !config
            .scheduled_tasks
            .get(definition.id)
            .copied()
            .unwrap_or(false)
        {
            continue;
        }
        if task_states.get(definition.id).copied().unwrap_or(true) {
            continue;
        }
        match tasks::disable(definition.path) {
            Ok(true) => repaired += 1,
            Ok(false) => {}
            Err(error) => errors.push(error),
        }
    }

    let mut package_names = catalog::packages()
        .into_iter()
        .filter(|definition| config.packages.get(definition.id).copied().unwrap_or(false))
        .map(|definition| definition.pattern.to_owned())
        .collect::<Vec<_>>();
    package_names.extend(config.custom_packages.iter().cloned());
    package_names.sort();
    package_names.dedup();

    match packages::enforce(&package_names) {
        Ok(result) => {
            repaired += result.removed.len() as u64;
            if !result.removed.is_empty() {
                state.add_activity(
                    "Apps",
                    format!("Removed {} app packages", result.removed.len()),
                    true,
                );
            }
            errors.extend(result.errors);
        }
        Err(error) => errors.push(error),
    }

    match startup_task::configure(config.start_with_windows) {
        Ok(true) => repaired += 1,
        Ok(false) => {}
        Err(error) => errors.push(error),
    }

    if repaired > 0 {
        state.add_activity("Enforcement", format!("Applied {repaired} changes"), true);
    }
    EnforcementOutcome { repaired, errors }
}

pub fn enforce_slow(state: &AppState) -> Result<u64, String> {
    let _guard = BusyGuard::acquire(state)?;
    enforce_slow_inner(state).complete(state)
}

pub fn enforce_all(app: &AppHandle, state: &AppState) -> Result<u64, String> {
    let _guard = BusyGuard::acquire(state)?;
    let mut outcome = enforce_runtime_inner(state);
    let killed = process_enforcement::enforce(app, state);
    outcome.merge(enforce_slow_inner(state));
    let result = outcome.complete(state).map(|repaired| repaired + killed);
    publish_live_state(app, state);
    result
}

pub fn start(app: AppHandle, state: AppState) {
    let ready_receiver = process_enforcement::start_monitor(app.clone(), state.clone());

    let initial_app = app.clone();
    let initial_state = state.clone();
    tauri::async_runtime::spawn(async move {
        let _ = tokio::time::timeout(Duration::from_secs(10), ready_receiver).await;
        let _ = tauri::async_runtime::spawn_blocking(move || {
            let _ = enforce_all(&initial_app, &initial_state);
        })
        .await;
    });

    let runtime_app = app.clone();
    let runtime_state = state.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(RUNTIME_RECONCILIATION_INTERVAL).await;
            let app = runtime_app.clone();
            let state = runtime_state.clone();
            let _ = tauri::async_runtime::spawn_blocking(move || {
                let result = enforce_runtime(&state);
                publish_live_state(&app, &state);
                result
            })
            .await;
        }
    });

    let slow_app = app;
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(SLOW_RECONCILIATION_INTERVAL).await;
            let app = slow_app.clone();
            let state = state.clone();
            let _ = tauri::async_runtime::spawn_blocking(move || {
                let result = enforce_slow(&state);
                publish_live_state(&app, &state);
                result
            })
            .await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{
        EnforcementOutcome, RUNTIME_RECONCILIATION_INTERVAL, SLOW_RECONCILIATION_INTERVAL,
    };

    #[test]
    fn combined_enforcement_keeps_earlier_errors() {
        let mut combined = EnforcementOutcome {
            repaired: 2,
            errors: vec!["runtime error".to_owned()],
        };
        combined.merge(EnforcementOutcome {
            repaired: 3,
            errors: Vec::new(),
        });

        assert_eq!(combined.repaired, 5);
        assert_eq!(combined.errors, ["runtime error"]);
    }

    #[test]
    fn recurring_integrity_checks_are_low_frequency() {
        assert_eq!(RUNTIME_RECONCILIATION_INTERVAL.as_secs(), 60 * 60);
        assert_eq!(SLOW_RECONCILIATION_INTERVAL.as_secs(), 24 * 60 * 60);
    }
}
