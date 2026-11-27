use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use uuid::Uuid;

use crate::catalog;
use crate::change_history::ChangeSection;
use crate::engine;
use crate::models::{
    AddProcessRuleRequest, AppConfig, AutostartEntry, ProcessInfo, ProcessRule, RuntimeSettings,
    Snapshot, SystemInfo,
};
use crate::process_enforcement;
use crate::snapshot;
use crate::state::AppState;
use crate::windows::{autostart, processes, startup_task};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CatalogSection {
    Policies,
    Packages,
    NetworkBlocks,
    ScheduledTasks,
}

impl CatalogSection {
    fn contains(self, id: &str) -> bool {
        match self {
            Self::Policies => {
                catalog::policies().iter().any(|item| item.id == id)
                    || catalog::system_settings().iter().any(|item| item.id == id)
            }
            Self::Packages => catalog::packages().iter().any(|item| item.id == id),
            Self::NetworkBlocks => catalog::network_blocks().iter().any(|item| item.id == id),
            Self::ScheduledTasks => catalog::scheduled_tasks().iter().any(|item| item.id == id),
        }
    }

    fn change_key(self, id: &str) -> String {
        match self {
            Self::Policies => ChangeSection::Setting.key(id),
            Self::Packages => catalog::packages()
                .into_iter()
                .find(|item| item.id == id)
                .map_or_else(
                    || ChangeSection::App.key(id),
                    |item| ChangeSection::App.key(item.pattern),
                ),
            Self::NetworkBlocks => ChangeSection::Network.key(id),
            Self::ScheduledTasks => ChangeSection::ScheduledTask.key(id),
        }
    }
}

async fn run_blocking<T: Send + 'static>(
    failure: &str,
    task: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(task)
        .await
        .map_err(|error| format!("{failure}: {error}"))?
}

fn update_with_startup_task(
    state: &AppState,
    start_with_windows: bool,
    update: impl FnOnce(&AppState) -> Result<(), String>,
) -> Result<(), String> {
    let previous_start_with_windows = state.config().start_with_windows;
    startup_task::configure(start_with_windows)?;
    let Err(error) = update(state) else {
        return Ok(());
    };
    if previous_start_with_windows != start_with_windows {
        startup_task::configure(previous_start_with_windows).map_err(|rollback_error| {
            format!("{error}; could not restore the startup task: {rollback_error}")
        })?;
    }
    Err(error)
}

async fn replace_profile(
    app: tauri::AppHandle,
    state: AppState,
    mut config: AppConfig,
) -> Result<(), String> {
    config.validate()?;
    config.enforcement_enabled = state.config().enforcement_enabled;
    run_blocking("Could not apply profile", move || {
        update_with_startup_task(&state, config.start_with_windows, |state| {
            state.replace_config(config)
        })?;
        state.release_all_changes()?;
        engine::enforce_all(&app, &state)
    })
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn get_snapshot(
    state: tauri::State<'_, AppState>,
    system: tauri::State<'_, SystemInfo>,
) -> Result<Snapshot, String> {
    let state = state.inner().clone();
    let system = system.inner().clone();
    run_blocking("Could not load application state", move || {
        Ok(snapshot::build(&state, system))
    })
    .await
}

#[tauri::command]
pub async fn set_catalog_items(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    section: CatalogSection,
    ids: Vec<String>,
    enabled: bool,
) -> Result<(), String> {
    if ids.is_empty() || !ids.iter().all(|id| section.contains(id)) {
        return Err("Unknown catalog item".to_owned());
    }
    if !enabled {
        let keys = ids
            .iter()
            .map(|id| section.change_key(id))
            .collect::<Vec<_>>();
        state.release_changes(keys.iter().map(String::as_str))?;
    }
    state.update_config(|config| {
        let values = match section {
            CatalogSection::Policies => &mut config.policies,
            CatalogSection::Packages => &mut config.packages,
            CatalogSection::NetworkBlocks => &mut config.network_blocks,
            CatalogSection::ScheduledTasks => &mut config.scheduled_tasks,
        };
        values.extend(ids.into_iter().map(|id| (id, enabled)));
    })?;

    let enforce: fn(&tauri::AppHandle, &AppState) -> Result<u64, String> = match section {
        CatalogSection::Policies if enabled => engine::enforce_runtime,
        CatalogSection::NetworkBlocks => engine::enforce_runtime,
        CatalogSection::Packages | CatalogSection::ScheduledTasks if enabled => {
            engine::enforce_slow
        }
        _ => return Ok(()),
    };
    let state = state.inner().clone();
    run_blocking("Could not apply rule", move || enforce(&app, &state)).await?;
    Ok(())
}

#[tauri::command]
pub async fn set_policy_choice(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
    choice: String,
) -> Result<(), String> {
    let definition = catalog::policies()
        .into_iter()
        .find(|definition| definition.id == id)
        .ok_or_else(|| "Unknown setting".to_owned())?;
    let selected = definition
        .choice(&choice)
        .ok_or_else(|| "Unknown option".to_owned())?;
    let enabled = state.update_config(|config| {
        if selected.value == definition.desired {
            config.policy_choices.remove(&id);
        } else {
            config.policy_choices.insert(id.clone(), choice);
        }
        config.policies.get(&id).copied().unwrap_or(false)
    })?;
    if enabled {
        let state = state.inner().clone();
        run_blocking("Could not apply setting", move || {
            engine::enforce_runtime(&app, &state)
        })
        .await?;
    }
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
        run_blocking("Could not apply process rule", move || {
            Ok(process_enforcement::enforce(&app, &state))
        })
        .await?;
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
    let executable_path = request
        .executable_path
        .filter(|path| !path.trim().is_empty());
    state.try_update_config(|config| {
        let duplicate = config.process_rules.iter().any(|rule| {
            rule.executable_name.eq_ignore_ascii_case(executable_name)
                && match (&rule.executable_path, &executable_path) {
                    (Some(existing), Some(requested)) => existing.eq_ignore_ascii_case(requested),
                    (None, None) => true,
                    _ => false,
                }
        });
        if duplicate {
            return Err(format!("A rule for {executable_name} already exists"));
        }
        config.process_rules.push(ProcessRule {
            id: Uuid::new_v4().to_string(),
            name,
            executable_name: executable_name.to_owned(),
            executable_path,
            enabled: true,
            built_in: false,
        });
        Ok(())
    })?;
    let state = state.inner().clone();
    run_blocking("Could not apply process rule", move || {
        Ok(process_enforcement::enforce(&app, &state))
    })
    .await?;
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
pub async fn list_processes() -> Result<Vec<ProcessInfo>, String> {
    run_blocking("Could not list processes", processes::list).await
}

#[tauri::command]
pub async fn list_autostarts() -> Result<Vec<AutostartEntry>, String> {
    run_blocking("Could not list autostarts", autostart::list).await
}

#[tauri::command]
pub async fn block_autostart(
    state: tauri::State<'_, AppState>,
    entry: AutostartEntry,
) -> Result<(), String> {
    let rule = autostart::rule_from_entry(&entry);
    if state.config().enforcement_enabled {
        let enforcement_rule = rule.clone();
        run_blocking("Could not disable autostart", move || {
            autostart::enforce_rule(&enforcement_rule)
        })
        .await?;
    }
    state.update_config(|config| {
        config
            .blocked_autostarts
            .retain(|existing| existing.id != rule.id);
        config.blocked_autostarts.push(rule);
    })
}

#[tauri::command]
pub fn remove_autostart_rule(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    state.release_changes([ChangeSection::Autostart.key(&id).as_str()])?;
    state.update_config(|config| {
        config
            .blocked_autostarts
            .retain(|existing| existing.id != id)
    })
}

#[tauri::command]
pub async fn add_custom_package(
    app: tauri::AppHandle,
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
    })?;
    let state = state.inner().clone();
    run_blocking("Could not remove app", move || {
        engine::enforce_slow(&app, &state)
    })
    .await?;
    Ok(())
}

#[tauri::command]
pub fn remove_custom_package(
    state: tauri::State<'_, AppState>,
    package_name: String,
) -> Result<(), String> {
    state.release_changes([ChangeSection::App.key(&package_name).as_str()])?;
    state.update_config(|config| {
        config
            .custom_packages
            .retain(|name| !name.eq_ignore_ascii_case(&package_name))
    })
}

#[tauri::command]
pub async fn update_runtime_settings(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    settings: RuntimeSettings,
) -> Result<(), String> {
    settings.validate()?;
    let state = state.inner().clone();
    run_blocking("Could not apply settings", move || {
        update_with_startup_task(&state, settings.start_with_windows, |state| {
            state.update_config(|config| {
                config.start_with_windows = settings.start_with_windows;
                config.active_hours_start = settings.active_hours_start;
                config.active_hours_end = settings.active_hours_end;
            })
        })?;
        engine::enforce_runtime(&app, &state)
    })
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn set_enforcement_enabled(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let state = state.inner().clone();
    state.update_config(|config| config.enforcement_enabled = enabled)?;
    if !enabled {
        state.release_all_changes()?;
    }
    state.add_activity(
        "Enforcement",
        if enabled {
            "Enforcement enabled"
        } else {
            "Enforcement disabled"
        },
        true,
    );
    crate::events::publish_live_state(&app, &state);
    if enabled {
        run_blocking("Could not enforce policy", move || {
            engine::enforce_all(&app, &state)
        })
        .await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn enforce_now(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<u64, String> {
    let state = state.inner().clone();
    run_blocking("Could not enforce policy", move || {
        engine::enforce_all(&app, &state)
    })
    .await
}

#[tauri::command]
pub async fn reset_profile(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    replace_profile(app, state.inner().clone(), AppConfig::default()).await
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
    replace_profile(app, state.inner().clone(), config).await
}

#[tauri::command]
pub fn clear_activity(app: tauri::AppHandle, state: tauri::State<'_, AppState>) {
    state.clear_activity();
    crate::events::publish_live_state(&app, &state);
}
