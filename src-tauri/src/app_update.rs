use std::time::Duration;

use parking_lot::Mutex;
use serde::Serialize;
use tauri::ipc::Channel;
use tauri::utils::platform::bundle_type;
use tauri::{Manager, ResourceId, Runtime, Webview};
use tauri_plugin_updater::Update;
use tokio_util::sync::CancellationToken;

use crate::state::AppState;

const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(30 * 60);

struct AppUpdateSession {
    cancellation: CancellationToken,
    downloading: bool,
    downloaded: Option<(ResourceId, Vec<u8>)>,
}

#[derive(Default)]
pub struct AppUpdateState(Mutex<Option<AppUpdateSession>>);

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    Started { content_length: Option<u64> },
    #[serde(rename_all = "camelCase")]
    Progress { chunk_length: usize },
}

#[tauri::command]
pub fn app_update_available() -> bool {
    !cfg!(debug_assertions) && bundle_type().is_some()
}

#[tauri::command]
pub fn prepare_app_update(state: tauri::State<'_, AppUpdateState>) -> Result<(), String> {
    if !app_update_available() {
        return Err("Updates require an installed release build".to_owned());
    }
    let mut session = state.0.lock();
    if session.is_none() {
        *session = Some(AppUpdateSession {
            cancellation: CancellationToken::new(),
            downloading: false,
            downloaded: None,
        });
    }
    Ok(())
}

#[tauri::command]
pub fn finish_app_update(state: tauri::State<'_, AppUpdateState>) {
    if let Some(session) = state.0.lock().take() {
        session.cancellation.cancel();
    }
}

#[tauri::command]
pub async fn download_app_update<R: Runtime>(
    webview: Webview<R>,
    rid: ResourceId,
    on_event: Channel<DownloadEvent>,
) -> Result<bool, String> {
    let update = webview
        .resources_table()
        .get::<Update>(rid)
        .map_err(|error| error.to_string())?;
    let mut update = (*update).clone();
    update.timeout = Some(DOWNLOAD_TIMEOUT);
    let state = webview.state::<AppUpdateState>();
    let cancellation = {
        let mut guard = state.0.lock();
        let session = guard
            .as_mut()
            .ok_or("Prepare the update before downloading")?;
        if session.downloading || session.downloaded.is_some() {
            return Err("An update download is already running".to_owned());
        }
        session.downloading = true;
        session.cancellation.clone()
    };

    let mut started = false;
    let download = update.download(
        |chunk_length, content_length| {
            if !started {
                started = true;
                let _ = on_event.send(DownloadEvent::Started { content_length });
            }
            let _ = on_event.send(DownloadEvent::Progress { chunk_length });
        },
        || {},
    );
    let result = tokio::select! {
        biased;
        _ = cancellation.cancelled() => None,
        result = download => Some(result),
    };

    let mut guard = state.0.lock();
    if cancellation.is_cancelled() {
        return Ok(false);
    }
    let session = guard.as_mut().ok_or("The update session has ended")?;
    session.downloading = false;
    match result {
        Some(Ok(bytes)) => {
            session.downloaded = Some((rid, bytes));
            Ok(true)
        }
        Some(Err(error)) => Err(error.to_string()),
        None => Ok(false),
    }
}

#[tauri::command]
pub fn cancel_app_update_download(state: tauri::State<'_, AppUpdateState>) {
    if let Some(session) = state.0.lock().as_mut() {
        session.cancellation.cancel();
        session.downloaded = None;
    }
}

#[tauri::command]
pub async fn install_app_update<R: Runtime>(
    webview: Webview<R>,
    rid: ResourceId,
) -> Result<(), String> {
    let update = webview
        .resources_table()
        .get::<Update>(rid)
        .map_err(|error| error.to_string())?;
    let bytes = {
        let state = webview.state::<AppUpdateState>();
        let mut guard = state.0.lock();
        let session = guard.as_mut().ok_or("The update session has ended")?;
        if session.cancellation.is_cancelled() {
            return Err("The update download was canceled".to_owned());
        }
        let (download_rid, bytes) = session
            .downloaded
            .take()
            .ok_or("Download the update before installing")?;
        if download_rid != rid {
            return Err("The downloaded package belongs to another update".to_owned());
        }
        bytes
    };
    let app_state = webview.state::<AppState>().inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _enforcement = app_state.lock_enforcement();
        update.install(bytes)
    })
    .await
    .map_err(|error| error.to_string())?
    .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn bundled_updater_configuration_requires_signed_versions() {
        let configuration: serde_json::Value =
            serde_json::from_str(include_str!("../tauri.conf.json")).unwrap();
        let updater: tauri_plugin_updater::Config =
            serde_json::from_value(configuration["plugins"]["updater"].clone()).unwrap();

        assert!(updater.require_signed_version);
        assert!(!updater.pubkey.is_empty());
        assert!(
            updater
                .endpoints
                .iter()
                .all(|endpoint| endpoint.scheme() == "https")
        );
    }
}
