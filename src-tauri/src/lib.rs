mod app_update;
mod catalog;
mod commands;
mod engine;
mod events;
mod models;
mod process_enforcement;
mod snapshot;
mod state;
mod windows;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

use state::AppState;

const BACKGROUND_ARGUMENT: &str = "--background";

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn build_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let open = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let enforce = MenuItem::with_id(app, "enforce", "Enforce now", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &enforce, &quit])?;

    let icon = app
        .default_window_icon()
        .ok_or_else(|| std::io::Error::other("Application icon is missing"))?
        .clone();

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .tooltip("Prep My Windows")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => show_main_window(app),
            "enforce" => {
                let app = app.clone();
                let state = app.state::<AppState>().inner().clone();
                tauri::async_runtime::spawn_blocking(move || engine::enforce_all(&app, &state));
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}

fn initialize(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let system = windows::system_info().map_err(std::io::Error::other)?;
    if !system.is_windows_11 {
        return Err(std::io::Error::other("Windows 11 is required").into());
    }
    if !system.is_elevated {
        return Err(std::io::Error::other("Administrator access is required").into());
    }

    let config_directory = app.path().app_config_dir().map_err(std::io::Error::other)?;
    let state = AppState::load(&config_directory).map_err(std::io::Error::other)?;
    app.manage(system);
    app.manage(state.clone());
    build_tray(app)?;

    if let Some(window) = app.get_webview_window("main") {
        let window_to_hide = window.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window_to_hide.hide();
            }
        });
    }

    if !std::env::args().any(|argument| argument == BACKGROUND_ARGUMENT) {
        show_main_window(app.handle());
    }
    engine::start(app.handle().clone(), state).map_err(std::io::Error::other)?;
    Ok(())
}

fn report_startup_error(error: &dyn std::fmt::Display) {
    windows::show_error_dialog(
        "Prep My Windows",
        &format!(
            "Prep My Windows could not start.

{error}"
        ),
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _| {
            if !args.iter().any(|argument| argument == BACKGROUND_ARGUMENT) {
                show_main_window(app);
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_update::AppUpdateState::default())
        .setup(|app| initialize(app).inspect_err(|error| report_startup_error(error)))
        .invoke_handler(tauri::generate_handler![
            commands::get_snapshot,
            commands::set_catalog_items,
            commands::set_policy_choice,
            commands::set_process_rule_enabled,
            commands::set_process_rule_notifications_muted,
            commands::add_process_rule,
            commands::remove_process_rule,
            commands::list_processes,
            commands::list_autostarts,
            commands::block_autostart,
            commands::remove_autostart_rule,
            commands::add_custom_package,
            commands::remove_custom_package,
            commands::update_runtime_settings,
            commands::set_enforcement_enabled,
            commands::enforce_now,
            commands::reset_profile,
            commands::export_profile,
            commands::import_profile,
            commands::clear_activity,
            app_update::app_update_available,
            app_update::prepare_app_update,
            app_update::finish_app_update,
            app_update::download_app_update,
            app_update::cancel_app_update_download,
            app_update::install_app_update,
        ])
        .run(tauri::generate_context!());
    if let Err(error) = result {
        report_startup_error(&error);
        std::process::exit(1);
    }
}
