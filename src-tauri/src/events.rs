use tauri::{AppHandle, Emitter};

use crate::models::LiveState;
use crate::state::AppState;

pub fn publish_live_state(app: &AppHandle, state: &AppState) {
    let _ = app.emit(
        "live-state-changed",
        LiveState {
            status: state.status(),
            activity: state.activity(),
        },
    );
}

pub fn publish_config_changed(app: &AppHandle) {
    let _ = app.emit("config-changed", ());
}
