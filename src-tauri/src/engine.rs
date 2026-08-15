use std::time::Duration;

use crate::catalog;
use crate::models::{ProcessRule, SupportRequirement, SystemSupport};
use crate::state::AppState;
use crate::windows::{
    self, autostart, hosts, packages, processes, registry, startup_task, system_settings, tasks,
};

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

pub fn enforce_processes(state: &AppState) -> u64 {
    let rules = state
        .config()
        .process_rules
        .into_iter()
        .filter(|rule| rule.enabled)
        .collect::<Vec<ProcessRule>>();
    let killed = processes::enforce(&rules);
    if !killed.is_empty() {
        state.add_activity("Process", format!("Stopped {}", killed.join(", ")), true);
        state.record_kills(killed.len() as u64);
    }
    killed.len() as u64
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

pub fn enforce_all(state: &AppState) -> Result<u64, String> {
    let _guard = BusyGuard::acquire(state)?;
    let mut outcome = enforce_runtime_inner(state);
    let killed = enforce_processes(state);
    outcome.merge(enforce_slow_inner(state));
    outcome.complete(state).map(|repaired| repaired + killed)
}

pub fn start(state: AppState) {
    let initial_state = state.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _ = enforce_all(&initial_state);
    });

    let process_state = state.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            let state = process_state.clone();
            let _ = tauri::async_runtime::spawn_blocking(move || enforce_processes(&state)).await;
        }
    });

    let runtime_state = state.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            let interval = runtime_state.config().enforcement_interval_seconds;
            tokio::time::sleep(Duration::from_secs(interval)).await;
            let state = runtime_state.clone();
            let _ = tauri::async_runtime::spawn_blocking(move || enforce_runtime(&state)).await;
        }
    });

    tauri::async_runtime::spawn(async move {
        loop {
            let interval = state.config().package_interval_minutes;
            tokio::time::sleep(Duration::from_secs(interval * 60)).await;
            let state = state.clone();
            let _ = tauri::async_runtime::spawn_blocking(move || enforce_slow(&state)).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::EnforcementOutcome;

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
}
