use std::time::Duration;

use parking_lot::MutexGuard;
use tauri::AppHandle;

use crate::catalog;
use crate::change_history::{AppliedChange, ChangeSection};
use crate::events::publish_live_state;
use crate::models::{
    PolicyDefinition, RegistryValue, SupportRequirement, SystemSupport, TaskStatus,
};
use crate::process_enforcement;
use crate::state::AppState;
use crate::windows::{
    self, autostart, hosts, packages, registry, services, startup_task, system_settings, tasks,
};

const RUNTIME_RECONCILIATION_INTERVAL: Duration = Duration::from_secs(60 * 60);
const SLOW_RECONCILIATION_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);
const MONITOR_READY_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Default)]
struct EnforcementOutcome {
    changes: Vec<AppliedChange>,
    errors: Vec<String>,
}

impl EnforcementOutcome {
    fn merge(&mut self, other: Self) {
        self.changes.extend(other.changes);
        self.errors.extend(other.errors);
    }

    fn complete(self, state: &AppState) -> u64 {
        let applied = self.changes.len() as u64;
        if let Err(error) = state.record_changes(self.changes) {
            state.report_problem(error);
        }
        for error in self.errors {
            state.report_problem(error);
        }
        state.complete_enforcement();
        applied
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

fn disabled_service_name(definition: &PolicyDefinition) -> Option<&'static str> {
    definition
        .service_name()
        .filter(|_| definition.desired == RegistryValue::Dword(4))
}

fn enforce_policy(definition: &PolicyDefinition) -> Result<Option<String>, String> {
    let current = registry::read_policy(definition)?;
    if definition.desired.is_satisfied_by(current.as_deref()) {
        return Ok(None);
    }
    match (definition.service_name(), &definition.desired) {
        (Some(service), RegistryValue::Dword(start_type)) => {
            services::set_start_type(service, *start_type)
                .map_err(|error| format!("Could not enforce {}: {error}", definition.name))?;
        }
        _ => {
            registry::enforce_policy(definition)?;
        }
    }
    Ok(Some(definition.value_label(current.as_deref())))
}

fn enforce_runtime_inner(state: &AppState) -> EnforcementOutcome {
    let config = state.config();
    let mut changes = Vec::new();
    let mut errors = Vec::new();
    let support = windows::support_info();
    if let Err(error) = &support {
        errors.push(error.clone());
    }

    for definition in catalog::configured_policies(&config) {
        if !config.policies.get(definition.id).copied().unwrap_or(false) {
            continue;
        }
        if !is_supported(definition.support, &support)
            || definition
                .service_name()
                .is_some_and(|name| !services::is_installed(name))
        {
            continue;
        }
        match enforce_policy(&definition) {
            Ok(Some(found)) => {
                changes.push(AppliedChange::new(
                    ChangeSection::Setting,
                    definition.id,
                    definition.name,
                    found,
                    definition.wanted_label(),
                ));
                if let Some(service) = disabled_service_name(&definition)
                    && let Err(error) = services::stop(service)
                {
                    errors.push(error);
                }
            }
            Ok(None) => {}
            Err(error) => errors.push(error),
        }
    }

    for definition in catalog::system_settings() {
        if !config.policies.get(definition.id).copied().unwrap_or(false)
            || !is_supported(definition.support, &support)
        {
            continue;
        }
        let before = match system_settings::read(definition.kind) {
            Ok(before) => before,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        if !before.available || before.compliant {
            continue;
        }
        match system_settings::enforce(definition.kind) {
            Ok(true) => changes.push(AppliedChange::new(
                ChangeSection::Setting,
                definition.id,
                definition.name,
                before.current,
                before.wanted,
            )),
            Ok(false) => {}
            Err(error) => errors.push(error),
        }
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
    match hosts::statuses(&enabled_network) {
        Ok(blocked) => match hosts::enforce(&enabled_network) {
            Ok(true) => changes.extend(
                enabled_network
                    .iter()
                    .zip(blocked)
                    .filter(|(_, blocked)| !blocked)
                    .map(|(definition, _)| {
                        AppliedChange::new(
                            ChangeSection::Network,
                            definition.id,
                            definition.domain,
                            "Allowed",
                            "Blocked",
                        )
                    }),
            ),
            Ok(false) => {}
            Err(error) => errors.push(error),
        },
        Err(error) => errors.push(error),
    }

    for rule in &config.blocked_autostarts {
        match autostart::enforce_rule(rule) {
            Ok(true) => changes.push(AppliedChange::new(
                ChangeSection::Autostart,
                &rule.id,
                rule.name.as_str(),
                "Active",
                "Blocked",
            )),
            Ok(false) => {}
            Err(error) => errors.push(error),
        }
    }

    EnforcementOutcome { changes, errors }
}

fn run_enforcement(
    app: &AppHandle,
    state: &AppState,
    guard: MutexGuard<'_, ()>,
    run: impl FnOnce() -> Result<u64, String>,
) -> Result<u64, String> {
    if !state.config().enforcement_enabled {
        return Ok(0);
    }
    publish_live_state(app, state);
    let result = run();
    drop(guard);
    publish_live_state(app, state);
    result
}

pub fn enforce_runtime(app: &AppHandle, state: &AppState) -> Result<u64, String> {
    run_enforcement(app, state, state.lock_enforcement(), || {
        Ok(enforce_runtime_inner(state).complete(state))
    })
}

fn removed_package_changes(removed: Vec<String>) -> Vec<AppliedChange> {
    let definitions = catalog::packages();
    let mut changes = Vec::<AppliedChange>::new();
    for package_name in removed {
        let key = ChangeSection::App.key(&package_name);
        if changes.iter().any(|change| change.key == key) {
            continue;
        }
        let name = definitions
            .iter()
            .find(|definition| definition.pattern.eq_ignore_ascii_case(&package_name))
            .map_or_else(
                || package_name.clone(),
                |definition| definition.name.to_owned(),
            );
        changes.push(AppliedChange::new(
            ChangeSection::App,
            &package_name,
            name,
            "Installed",
            "Removed",
        ));
    }
    changes
}

fn enforce_slow_inner(state: &AppState) -> EnforcementOutcome {
    let config = state.config();
    let mut changes = Vec::new();
    let mut errors = Vec::new();

    let task_definitions = catalog::scheduled_tasks();
    let task_states = match tasks::states(&task_definitions) {
        Ok(states) => states,
        Err(error) => {
            errors.push(error);
            Default::default()
        }
    };
    let pending_tasks = task_definitions
        .into_iter()
        .filter(|definition| {
            config
                .scheduled_tasks
                .get(definition.id)
                .copied()
                .unwrap_or(false)
                && task_states.get(definition.id) == Some(&TaskStatus::Enabled)
        })
        .collect::<Vec<_>>();
    if !pending_tasks.is_empty() {
        let disabled = tasks::disable(&pending_tasks);
        changes.extend(
            pending_tasks
                .iter()
                .filter(|definition| disabled.disabled.contains(&definition.id))
                .map(|definition| {
                    AppliedChange::new(
                        ChangeSection::ScheduledTask,
                        definition.id,
                        definition.name,
                        "Enabled",
                        "Disabled",
                    )
                }),
        );
        errors.extend(disabled.errors);
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
            changes.extend(removed_package_changes(result.removed));
            errors.extend(result.errors);
        }
        Err(error) => errors.push(error),
    }

    if let Err(error) = startup_task::configure(config.start_with_windows) {
        errors.push(error);
    }

    EnforcementOutcome { changes, errors }
}

pub fn enforce_slow(app: &AppHandle, state: &AppState) -> Result<u64, String> {
    run_enforcement(app, state, state.lock_enforcement(), || {
        Ok(enforce_slow_inner(state).complete(state))
    })
}

pub fn enforce_all(app: &AppHandle, state: &AppState) -> Result<u64, String> {
    run_enforcement(app, state, state.lock_enforcement(), || {
        let mut outcome = enforce_runtime_inner(state);
        let killed = process_enforcement::enforce(app, state);
        outcome.merge(enforce_slow_inner(state));
        Ok(outcome.complete(state) + killed)
    })
}

fn spawn_reconciliation(
    app: AppHandle,
    state: AppState,
    interval: Duration,
    enforce: fn(&AppHandle, &AppState) -> Result<u64, String>,
) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            let task_app = app.clone();
            let task_state = state.clone();
            if let Err(error) =
                tauri::async_runtime::spawn_blocking(move || enforce(&task_app, &task_state)).await
            {
                state.report_problem(format!("Enforcement stopped unexpectedly: {error}"));
                publish_live_state(&app, &state);
            }
        }
    });
}

pub fn start(app: AppHandle, state: AppState) -> Result<(), String> {
    let ready_receiver = process_enforcement::start_monitor(app.clone(), state.clone())?;

    let initial_app = app.clone();
    let initial_state = state.clone();
    tauri::async_runtime::spawn(async move {
        let _ = tokio::time::timeout(MONITOR_READY_TIMEOUT, ready_receiver).await;
        let _ =
            tauri::async_runtime::spawn_blocking(move || enforce_all(&initial_app, &initial_state))
                .await;
    });

    spawn_reconciliation(
        app.clone(),
        state.clone(),
        RUNTIME_RECONCILIATION_INTERVAL,
        enforce_runtime,
    );
    spawn_reconciliation(app, state, SLOW_RECONCILIATION_INTERVAL, enforce_slow);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        EnforcementOutcome, RUNTIME_RECONCILIATION_INTERVAL, SLOW_RECONCILIATION_INTERVAL,
        disabled_service_name, removed_package_changes,
    };
    use crate::catalog;

    #[test]
    fn stops_services_disabled_by_policies() {
        let services = catalog::policies()
            .iter()
            .filter_map(disabled_service_name)
            .collect::<Vec<_>>();

        assert!(services.contains(&"DiagTrack"));
        assert!(services.contains(&"dmwappushservice"));
        assert!(services.contains(&"WerSvc"));
        assert!(services.contains(&"MapsBroker"));
        assert!(services.contains(&"RetailDemo"));
    }

    #[test]
    fn names_removed_packages_once_with_their_catalog_name() {
        let changes = removed_package_changes(vec![
            "Microsoft.BingNews".to_owned(),
            "Microsoft.BingNews".to_owned(),
            "Contoso.Custom".to_owned(),
        ]);

        assert_eq!(changes.len(), 2);
        assert_ne!(changes[0].name, "Microsoft.BingNews");
        assert_eq!(changes[1].name, "Contoso.Custom");
        assert_eq!(changes[1].key, "app:contoso.custom");
    }

    #[test]
    fn combined_enforcement_keeps_earlier_errors() {
        let mut combined = EnforcementOutcome {
            changes: removed_package_changes(vec!["A".to_owned()]),
            errors: vec!["runtime error".to_owned()],
        };
        combined.merge(EnforcementOutcome {
            changes: removed_package_changes(vec!["B".to_owned(), "C".to_owned()]),
            errors: Vec::new(),
        });

        assert_eq!(combined.changes.len(), 3);
        assert_eq!(combined.errors, ["runtime error"]);
    }

    #[test]
    fn recurring_integrity_checks_are_low_frequency() {
        assert_eq!(RUNTIME_RECONCILIATION_INTERVAL.as_secs(), 60 * 60);
        assert_eq!(SLOW_RECONCILIATION_INTERVAL.as_secs(), 24 * 60 * 60);
    }
}
