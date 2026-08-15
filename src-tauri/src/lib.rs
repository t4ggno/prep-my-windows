mod catalog;
mod commands;
mod engine;
mod events;
mod models;
mod process_enforcement;
mod state;
mod windows;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

use state::AppState;

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
                tauri::async_runtime::spawn_blocking(move || {
                    let _ = engine::enforce_all(&app, &state);
                });
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _| {
            if !args.iter().any(|argument| argument == "--background") {
                show_main_window(app);
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let info = windows::system_info().map_err(std::io::Error::other)?;
            if !info.is_windows_11 {
                return Err(std::io::Error::other("Windows 11 is required").into());
            }
            if !info.is_elevated {
                return Err(std::io::Error::other("Administrator access is required").into());
            }

            let config_directory = app.path().app_config_dir().map_err(std::io::Error::other)?;
            let state = AppState::load(&config_directory).map_err(std::io::Error::other)?;
            windows::startup_task::configure(state.config().start_with_windows)
                .map_err(std::io::Error::other)?;
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

            let background = std::env::args().any(|argument| argument == "--background");
            if !background {
                show_main_window(app.handle());
            }
            engine::start(app.handle().clone(), state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_snapshot,
            commands::set_catalog_item,
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
            commands::enforce_now,
            commands::reset_profile,
            commands::export_profile,
            commands::import_profile,
            commands::clear_activity,
        ])
        .run(tauri::generate_context!())
}
