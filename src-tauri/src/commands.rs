use std::fs;
use std::path::PathBuf;

use uuid::Uuid;

use crate::catalog;
use crate::engine;
use crate::models::{
    AddProcessRuleRequest, AppConfig, AutostartEntry, NetworkView, PackageView, PolicyView,
    RuntimeSettings, ScheduledTaskView, Snapshot,
};
use crate::process_enforcement;
use crate::state::AppState;
use crate::windows::{
    autostart, hosts, packages, processes, registry, startup_task, system_settings, tasks,
};

#[tauri::command]
pub async fn get_snapshot(state: tauri::State<'_, AppState>) -> Result<Snapshot, String> {
    let state = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || build_snapshot(&state))
        .await
        .map_err(|error| format!("Could not load application state: {error}"))?
}

fn build_snapshot(state: &AppState) -> Result<Snapshot, String> {
    let config = state.config();
    let system = crate::windows::system_info()?;
    let support = crate::windows::support_info()?;
    let definitions = catalog::configured_policies(&config);
    let mut policies = definitions
        .iter()
        .map(|definition| {
            if let Some(reason) = definition.support.unavailable_reason(&support) {
                return Ok(PolicyView {
                    id: definition.id.to_owned(),
                    name: definition.name.to_owned(),
                    category: definition.category.to_owned(),
                    scope: definition.hive.label().to_owned(),
                    enabled: config.policies.get(definition.id).copied().unwrap_or(false),
                    compliant: false,
                    current: "Unavailable".to_owned(),
                    wanted: definition.desired.label(),
                    available: false,
                    unavailable_reason: Some(reason),
                });
            }
            let current = registry::read_policy(definition)?;
            let wanted = definition.desired.label();
            Ok(PolicyView {
                id: definition.id.to_owned(),
                name: definition.name.to_owned(),
                category: definition.category.to_owned(),
                scope: definition.hive.label().to_owned(),
                enabled: config.policies.get(definition.id).copied().unwrap_or(false),
                compliant: definition.desired.is_satisfied_by(current.as_deref()),
                current: current.unwrap_or_else(|| "Not set".to_owned()),
                wanted,
                available: true,
                unavailable_reason: None,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let system_policy_views = catalog::system_settings()
        .into_iter()
        .map(|definition| {
            if let Some(reason) = definition.support.unavailable_reason(&support) {
                return Ok(PolicyView {
                    id: definition.id.to_owned(),
                    name: definition.name.to_owned(),
                    category: definition.category.to_owned(),
                    scope: definition.scope.to_owned(),
                    enabled: config.policies.get(definition.id).copied().unwrap_or(false),
                    compliant: false,
                    current: "Unavailable".to_owned(),
                    wanted: "Unavailable".to_owned(),
                    available: false,
                    unavailable_reason: Some(reason),
                });
            }
            let current = system_settings::read(definition.kind)?;
            Ok(PolicyView {
                id: definition.id.to_owned(),
                name: definition.name.to_owned(),
                category: definition.category.to_owned(),
                scope: definition.scope.to_owned(),
                enabled: config.policies.get(definition.id).copied().unwrap_or(false),
                compliant: current.compliant,
                current: current.current,
                wanted: current.wanted,
                available: current.available,
                unavailable_reason: current.unavailable_reason,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    policies.extend(system_policy_views);

    let installed_packages = packages::list()?;
    let package_views = catalog::packages()
        .into_iter()
        .map(|definition| {
            let matches = installed_packages
                .iter()
                .filter(|package| package.name.eq_ignore_ascii_case(definition.pattern));
            let mut installed = false;
            let mut provisioned = false;
            for package in matches {
                installed = true;
                provisioned |= package.provisioned;
            }
            PackageView {
                id: definition.id.to_owned(),
                name: definition.name.to_owned(),
                package_name: definition.pattern.to_owned(),
                enabled: config.packages.get(definition.id).copied().unwrap_or(false),
                installed,
                provisioned,
            }
        })
        .collect();

    let network_definitions = catalog::network_blocks();
    let blocked = hosts::statuses(&network_definitions)?;
    let network_blocks = network_definitions
        .into_iter()
        .zip(blocked)
        .map(|(definition, blocked)| NetworkView {
            id: definition.id.to_owned(),
            domain: definition.domain.to_owned(),
            enabled: config
                .network_blocks
                .get(definition.id)
                .copied()
                .unwrap_or(false),
            blocked,
        })
        .collect();

    let task_definitions = catalog::scheduled_tasks();
    let task_states = tasks::states(&task_definitions)?;
    let scheduled_tasks = task_definitions
        .into_iter()
        .map(|definition| ScheduledTaskView {
            id: definition.id.to_owned(),
            name: definition.name.to_owned(),
            path: definition.path.to_owned(),
            enabled: config
                .scheduled_tasks
                .get(definition.id)
                .copied()
                .unwrap_or(false),
            disabled: task_states.get(definition.id).copied().unwrap_or(true),
        })
        .collect();

    Ok(Snapshot {
        system,
        status: state.status(),
        settings: config,
        policies,
        packages: package_views,
        installed_packages,
        network_blocks,
        scheduled_tasks,
        activity: state.activity(),
    })
}

fn replace_profile(state: &AppState, config: AppConfig) -> Result<(), String> {
    config.validate()?;
    let previous_start_with_windows = state.config().start_with_windows;
    let start_with_windows = config.start_with_windows;
    startup_task::configure(start_with_windows)?;
    if let Err(error) = state.replace_config(config) {
        if previous_start_with_windows != start_with_windows {
            startup_task::configure(previous_start_with_windows).map_err(|rollback_error| {
                format!(
                    "{error}; could not restore the startup task after the profile update failed: {rollback_error}"
                )
            })?;
        }
        return Err(error);
    }
    Ok(())
}

#[tauri::command]
pub async fn set_catalog_item(
    state: tauri::State<'_, AppState>,
    section: String,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    let valid = match section.as_str() {
        "policies" => {
            catalog::policies().iter().any(|item| item.id == id)
                || catalog::system_settings().iter().any(|item| item.id == id)
        }
        "packages" => catalog::packages().iter().any(|item| item.id == id),
        "networkBlocks" => catalog::network_blocks().iter().any(|item| item.id == id),
        "scheduledTasks" => catalog::scheduled_tasks().iter().any(|item| item.id == id),
        _ => return Err("Unknown catalog section".to_owned()),
    };
    if !valid {
        return Err("Unknown catalog item".to_owned());
    }
    state.update_config(|config| {
        let values = match section.as_str() {
            "policies" => &mut config.policies,
            "packages" => &mut config.packages,
            "networkBlocks" => &mut config.network_blocks,
            "scheduledTasks" => &mut config.scheduled_tasks,
            _ => unreachable!(),
        };
        values.insert(id, enabled);
    })?;

    let state = state.inner().clone();
    match section.as_str() {
        "policies" if enabled => {
            tauri::async_runtime::spawn_blocking(move || engine::enforce_runtime(&state))
                .await
                .map_err(|error| format!("Could not apply policy: {error}"))??
        }
        "networkBlocks" => {
            tauri::async_runtime::spawn_blocking(move || engine::enforce_runtime(&state))
                .await
                .map_err(|error| format!("Could not apply network rules: {error}"))??
        }
        "packages" | "scheduledTasks" if enabled => {
            tauri::async_runtime::spawn_blocking(move || engine::enforce_slow(&state))
                .await
                .map_err(|error| format!("Could not apply rule: {error}"))??
        }
        _ => 0,
    };
    Ok(())
}

#[tauri::command]
pub async fn set_process_rule_enabled(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    state.try_update_config(|config| {
        let rule = config
            .process_rules
            .iter_mut()
            .find(|rule| rule.id == id)
            .ok_or_else(|| "Unknown process rule".to_owned())?;
        rule.enabled = enabled;
        Ok(())
    })?;
    if enabled {
        let state = state.inner().clone();
        tauri::async_runtime::spawn_blocking(move || process_enforcement::enforce(&app, &state))
            .await
            .map_err(|error| format!("Could not apply process rule: {error}"))?;
    }
    Ok(())
}

#[tauri::command]
pub fn set_process_rule_notifications_muted(
    state: tauri::State<'_, AppState>,
    id: String,
    muted: bool,
) -> Result<(), String> {
    state.try_update_config(|config| {
        if !config.process_rules.iter().any(|rule| rule.id == id) {
            return Err("Unknown process rule".to_owned());
        }
        if muted {
            config.muted_process_notifications.insert(id);
        } else {
            config.muted_process_notifications.remove(&id);
        }
        Ok(())
    })
}

#[tauri::command]
pub async fn add_process_rule(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    request: AddProcessRuleRequest,
) -> Result<(), String> {
    let executable_name = request.executable_name.trim();
    if executable_name.is_empty() {
        return Err("Executable name is required".to_owned());
    }
    let name = if request.name.trim().is_empty() {
        executable_name.to_owned()
    } else {
        request.name.trim().to_owned()
    };
    state.update_config(|config| {
        config.process_rules.push(crate::models::ProcessRule {
            id: Uuid::new_v4().to_string(),
            name,
            executable_name: executable_name.to_owned(),
            executable_path: request
                .executable_path
                .filter(|path| !path.trim().is_empty()),
            enabled: true,
            built_in: false,
        });
    })?;
    let state = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || process_enforcement::enforce(&app, &state))
        .await
        .map_err(|error| format!("Could not apply process rule: {error}"))?;
    Ok(())
}

#[tauri::command]
pub fn remove_process_rule(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    state.update_config(|config| {
        config.process_rules.retain(|rule| rule.id != id);
        config.muted_process_notifications.remove(&id);
    })
}

#[tauri::command]
pub async fn list_processes() -> Result<Vec<crate::models::ProcessInfo>, String> {
    tauri::async_runtime::spawn_blocking(processes::list)
        .await
        .map_err(|error| format!("Could not list processes: {error}"))
}

#[tauri::command]
pub async fn list_autostarts() -> Result<Vec<AutostartEntry>, String> {
    tauri::async_runtime::spawn_blocking(autostart::list)
        .await
        .map_err(|error| format!("Could not list autostarts: {error}"))?
}

#[tauri::command]
pub async fn block_autostart(
    state: tauri::State<'_, AppState>,
    entry: AutostartEntry,
) -> Result<(), String> {
    let rule = autostart::rule_from_entry(&entry);
    let enforcement_rule = rule.clone();
    tauri::async_runtime::spawn_blocking(move || autostart::enforce_rule(&enforcement_rule))
        .await
        .map_err(|error| format!("Could not disable autostart: {error}"))??;
    state.update_config(|config| {
        config
            .blocked_autostarts
            .retain(|existing| existing.id != rule.id);
        config.blocked_autostarts.push(rule);
    })
}

#[tauri::command]
pub fn remove_autostart_rule(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    state.update_config(|config| {
        config
            .blocked_autostarts
            .retain(|existing| existing.id != id)
    })
}

#[tauri::command]
pub fn add_custom_package(
    state: tauri::State<'_, AppState>,
    package_name: String,
) -> Result<(), String> {
    let package_name = package_name.trim();
    if package_name.is_empty() {
        return Err("Package name is required".to_owned());
    }
    state.update_config(|config| {
        if !config
            .custom_packages
            .iter()
            .any(|name| name.eq_ignore_ascii_case(package_name))
        {
            config.custom_packages.push(package_name.to_owned());
            config.custom_packages.sort();
        }
    })
}

#[tauri::command]
pub fn remove_custom_package(
    state: tauri::State<'_, AppState>,
    package_name: String,
) -> Result<(), String> {
    state.update_config(|config| {
        config
            .custom_packages
            .retain(|name| !name.eq_ignore_ascii_case(&package_name))
    })
}

#[tauri::command]
pub async fn update_runtime_settings(
    state: tauri::State<'_, AppState>,
    settings: RuntimeSettings,
) -> Result<(), String> {
    settings.validate()?;
    let previous_start_with_windows = state.config().start_with_windows;
    let start_with_windows = settings.start_with_windows;
    startup_task::configure(start_with_windows)?;
    if let Err(error) = state.update_config(|config| {
        config.start_with_windows = start_with_windows;
        config.active_hours_start = settings.active_hours_start;
        config.active_hours_end = settings.active_hours_end;
    }) {
        if previous_start_with_windows != start_with_windows {
            startup_task::configure(previous_start_with_windows).map_err(|rollback_error| {
                format!(
                    "{error}; could not restore the startup task after the settings update failed: {rollback_error}"
                )
            })?;
        }
        return Err(error);
    }
    let state = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || engine::enforce_runtime(&state))
        .await
        .map_err(|error| format!("Could not apply settings: {error}"))??;
    Ok(())
}

#[tauri::command]
pub async fn enforce_now(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<u64, String> {
    let state = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || engine::enforce_all(&app, &state))
        .await
        .map_err(|error| format!("Could not enforce policy: {error}"))?
}

#[tauri::command]
pub async fn reset_profile(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    replace_profile(&state, AppConfig::default())?;
    let state = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || engine::enforce_all(&app, &state))
        .await
        .map_err(|error| format!("Could not apply profile: {error}"))??;
    Ok(())
}

#[tauri::command]
pub fn export_profile(state: tauri::State<'_, AppState>, path: String) -> Result<(), String> {
    let content = serde_json::to_string_pretty(&state.config())
        .map_err(|error| format!("Could not serialize profile: {error}"))?;
    fs::write(PathBuf::from(path), content)
        .map_err(|error| format!("Could not export profile: {error}"))
}

#[tauri::command]
pub async fn import_profile(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    let content = fs::read_to_string(PathBuf::from(path))
        .map_err(|error| format!("Could not read profile: {error}"))?;
    let config: AppConfig = serde_json::from_str(&content)
        .map_err(|error| format!("Could not parse profile: {error}"))?;
    replace_profile(&state, config)?;
    let state = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || engine::enforce_all(&app, &state))
        .await
        .map_err(|error| format!("Could not apply profile: {error}"))??;
    Ok(())
}

#[tauri::command]
pub fn clear_activity(state: tauri::State<'_, AppState>) {
    state.clear_activity();
}
