use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use tauri::AppHandle;

use crate::events::{publish_config_changed, publish_live_state};
use crate::models::{AppConfig, ProcessRule};
use crate::state::AppState;
use crate::windows::{notifications, processes};

const MONITOR_RETRY_INTERVAL: Duration = Duration::from_secs(60);

fn should_notify(config: &AppConfig, stopped: &processes::StoppedProcess) -> bool {
    stopped
        .rule_ids
        .iter()
        .any(|id| !config.muted_process_notifications.contains(id))
}

fn apply_notification_action(
    state: &AppState,
    stopped: &processes::StoppedProcess,
    action: notifications::ProcessNotificationAction,
) -> Result<(), String> {
    match action {
        notifications::ProcessNotificationAction::AllowOnce => {
            state.allow_process_once(stopped.allowance_key.clone());
            Ok(())
        }
        notifications::ProcessNotificationAction::AlwaysAllow => state.update_config(|config| {
            for rule in &mut config.process_rules {
                if stopped.rule_ids.contains(&rule.id) {
                    rule.enabled = false;
                }
            }
        }),
    }
}

fn handle_notification_action(
    app: &AppHandle,
    state: &AppState,
    stopped: &processes::StoppedProcess,
    action: notifications::ProcessNotificationAction,
) {
    if let Err(error) = apply_notification_action(state, stopped, action) {
        state.add_activity("Error", error, false);
        publish_live_state(app, state);
        return;
    }

    let launch_result = stopped
        .launch
        .as_ref()
        .ok_or_else(|| {
            format!(
                "Could not reopen {}: executable path unavailable",
                stopped.rule_name
            )
        })
        .and_then(processes::launch);
    if let Err(error) = launch_result {
        if action == notifications::ProcessNotificationAction::AllowOnce {
            state.revoke_process_allowance(&stopped.allowance_key);
        }
        state.add_activity("Error", error, false);
    } else {
        let message = match action {
            notifications::ProcessNotificationAction::AllowOnce => {
                format!("Allowed {} once", stopped.rule_name)
            }
            notifications::ProcessNotificationAction::AlwaysAllow => {
                format!("Always allowed {}", stopped.rule_name)
            }
        };
        state.add_activity("Process", message, true);
    }
    if action == notifications::ProcessNotificationAction::AlwaysAllow {
        publish_config_changed(app);
    }
    publish_live_state(app, state);
}

fn notify_process_stopped(app: &AppHandle, state: &AppState, stopped: &processes::StoppedProcess) {
    if !should_notify(&state.config(), stopped) {
        return;
    }

    let handled = Arc::new(AtomicBool::new(false));
    let callback_handled = handled.clone();
    let callback_app = app.clone();
    let callback_state = state.clone();
    let callback_stopped = stopped.clone();
    if let Err(error) = notifications::show_process_stopped(
        &app.config().identifier,
        &stopped.rule_name,
        move |action| {
            if callback_handled.swap(true, Ordering::AcqRel) {
                return;
            }
            handle_notification_action(&callback_app, &callback_state, &callback_stopped, action);
        },
    ) {
        state.add_activity("Error", error, false);
    }
}

fn record_stopped_processes(
    app: &AppHandle,
    state: &AppState,
    stopped_processes: Vec<processes::StoppedProcess>,
) -> u64 {
    if stopped_processes.is_empty() {
        return 0;
    }

    state.add_activity(
        "Process",
        format!(
            "Stopped {}",
            stopped_processes
                .iter()
                .map(|process| process.process_name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        true,
    );
    state.record_kills(stopped_processes.len() as u64);
    for stopped in &stopped_processes {
        notify_process_stopped(app, state, stopped);
    }
    publish_live_state(app, state);
    stopped_processes.len() as u64
}

fn enabled_rules(state: &AppState) -> Vec<ProcessRule> {
    let config = state.config();
    if !config.enforcement_enabled {
        return Vec::new();
    }
    config
        .process_rules
        .into_iter()
        .filter(|rule| rule.enabled)
        .collect()
}

fn report_errors(app: &AppHandle, state: &AppState, errors: &[String]) {
    if errors.is_empty() {
        return;
    }
    for error in errors {
        state.report_problem(error.clone());
    }
    publish_live_state(app, state);
}

pub fn enforce(app: &AppHandle, state: &AppState) -> u64 {
    let rules = enabled_rules(state);
    let outcome = processes::enforce(&rules, |key| state.consume_process_allowance(key));
    report_errors(app, state, &outcome.errors);
    record_stopped_processes(app, state, outcome.stopped)
}

fn enforce_started(app: &AppHandle, state: &AppState, process_id: u32, process_name: &str) {
    let rules = enabled_rules(state);
    match processes::enforce_started(process_id, process_name, &rules, |key| {
        state.consume_process_allowance(key)
    }) {
        Ok(Some(stopped)) => {
            record_stopped_processes(app, state, vec![stopped]);
        }
        Ok(None) => {}
        Err(error) => report_errors(app, state, &[error]),
    }
}

fn watch_once(
    app: &AppHandle,
    state: &AppState,
    ready: &mut Option<tokio::sync::oneshot::Sender<()>>,
) -> Result<(), String> {
    let result = processes::watch_starts(
        || {
            if let Some(ready) = ready.take() {
                let _ = ready.send(());
            }
        },
        |process_id, process_name| enforce_started(app, state, process_id, process_name),
    );
    if let Some(ready) = ready.take() {
        let _ = ready.send(());
    }
    result
}

fn run_monitor(app: AppHandle, state: AppState, ready: tokio::sync::oneshot::Sender<()>) {
    let mut ready = Some(ready);
    loop {
        let error =
            match panic::catch_unwind(AssertUnwindSafe(|| watch_once(&app, &state, &mut ready))) {
                Ok(Ok(())) => None,
                Ok(Err(error)) => Some(error),
                Err(_) => Some("Process event monitoring stopped unexpectedly".to_owned()),
            };
        if let Some(error) = error {
            report_errors(&app, &state, &[error]);
        }
        std::thread::sleep(MONITOR_RETRY_INTERVAL);
    }
}

pub fn start_monitor(
    app: AppHandle,
    state: AppState,
) -> Result<tokio::sync::oneshot::Receiver<()>, String> {
    let (ready_sender, ready_receiver) = tokio::sync::oneshot::channel();
    std::thread::Builder::new()
        .name("process-monitor".to_owned())
        .spawn(move || run_monitor(app, state, ready_sender))
        .map_err(|error| format!("Could not start process monitoring: {error}"))?;
    Ok(ready_receiver)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use uuid::Uuid;

    use super::{apply_notification_action, enabled_rules, should_notify};
    use crate::state::AppState;
    use crate::windows::notifications::ProcessNotificationAction;
    use crate::windows::processes::StoppedProcess;

    fn temporary_directory() -> std::path::PathBuf {
        std::env::temp_dir().join(format!("prep-my-windows-process-test-{}", Uuid::new_v4()))
    }

    fn stopped_process(rule_id: String) -> StoppedProcess {
        StoppedProcess {
            process_name: "app.exe".to_owned(),
            rule_name: "App".to_owned(),
            rule_ids: vec![rule_id],
            allowance_key: "app.exe".to_owned(),
            launch: None,
        }
    }

    #[test]
    fn rules_apply_only_while_enforcement_is_enabled() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();
        assert!(enabled_rules(&state).is_empty());

        state
            .update_config(|config| config.enforcement_enabled = true)
            .unwrap();

        assert!(!enabled_rules(&state).is_empty());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn allow_once_is_consumed_without_changing_the_rule() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();
        let rule_id = state.config().process_rules[0].id.clone();
        let stopped = stopped_process(rule_id.clone());

        apply_notification_action(&state, &stopped, ProcessNotificationAction::AllowOnce).unwrap();

        assert!(
            state
                .config()
                .process_rules
                .iter()
                .find(|rule| rule.id == rule_id)
                .unwrap()
                .enabled
        );
        assert!(state.consume_process_allowance(&stopped.allowance_key));
        assert!(!state.consume_process_allowance(&stopped.allowance_key));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn always_allow_disables_the_retained_rule_persistently() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();
        let rule_id = state.config().process_rules[0].id.clone();
        let stopped = stopped_process(rule_id.clone());

        apply_notification_action(&state, &stopped, ProcessNotificationAction::AlwaysAllow)
            .unwrap();

        let reloaded = AppState::load(&directory).unwrap().config();
        let rule = reloaded
            .process_rules
            .iter()
            .find(|rule| rule.id == rule_id)
            .unwrap();
        assert!(!rule.enabled);
        assert!(rule.built_in);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn muting_a_notification_does_not_disable_the_rule() {
        let directory = temporary_directory();
        let state = AppState::load(&directory).unwrap();
        let rule_id = state.config().process_rules[0].id.clone();
        let stopped = stopped_process(rule_id.clone());
        assert!(should_notify(&state.config(), &stopped));

        state
            .update_config(|config| {
                config.muted_process_notifications.insert(rule_id.clone());
            })
            .unwrap();

        let config = state.config();
        assert!(!should_notify(&config, &stopped));
        assert!(
            config
                .process_rules
                .iter()
                .find(|rule| rule.id == rule_id)
                .unwrap()
                .enabled
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
