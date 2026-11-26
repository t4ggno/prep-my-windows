pub mod autostart;
mod command;
pub mod hosts;
pub mod notifications;
pub mod packages;
pub mod processes;
pub mod registry;
pub mod services;
pub mod startup_task;
pub mod system_settings;
pub mod tasks;

use std::ffi::c_void;
use std::fs;
use std::mem::size_of;
use std::path::PathBuf;

use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::Security::{
    GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    MB_ICONERROR, MB_OK, MB_SETFOREGROUND, MessageBoxW,
};
use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_64KEY};

pub use command::{
    DEFAULT_TIMEOUT, powershell_string_array, run_hidden, run_powershell, run_powershell_json,
};

const USER_CHOICE_PROTECTION_SERVICE: &str = "UCPD";

use crate::models::{SystemInfo, SystemSupport};

fn current_version_key() -> Result<RegKey, String> {
    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    local_machine
        .open_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .map_err(|error| format!("Could not read Windows version: {error}"))
}

fn edge_major_from_directory_name(name: &str) -> Option<u32> {
    let mut components = name.split('.');
    let major = components.next()?.parse().ok()?;
    (components.count() >= 2).then_some(major)
}

fn edge_major_version() -> Option<u32> {
    ["ProgramFiles(x86)", "ProgramFiles", "LOCALAPPDATA"]
        .into_iter()
        .filter_map(std::env::var_os)
        .map(PathBuf::from)
        .map(|root| root.join(r"Microsoft\Edge\Application"))
        .filter_map(|directory| fs::read_dir(directory).ok())
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|file_type| file_type.is_dir()))
        .filter_map(|entry| {
            edge_major_from_directory_name(entry.file_name().to_string_lossy().as_ref())
        })
        .max()
}

pub fn support_info() -> Result<SystemSupport, String> {
    let key = current_version_key()?;
    let build = key
        .get_value::<String, _>("CurrentBuildNumber")
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap_or_default();
    let edition_id = key.get_value::<String, _>("EditionID").unwrap_or_default();
    Ok(SystemSupport {
        build,
        edition_id,
        edge_major_version: edge_major_version(),
        user_choice_protection_active: services::is_running(USER_CHOICE_PROTECTION_SERVICE)?,
    })
}

fn is_elevated() -> Result<bool, String> {
    let mut token: HANDLE = std::ptr::null_mut();
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
        return Err(format!(
            "Could not read process access: {}",
            std::io::Error::last_os_error()
        ));
    }
    let mut elevation = TOKEN_ELEVATION::default();
    let mut returned = 0_u32;
    let queried = unsafe {
        GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut TOKEN_ELEVATION as *mut c_void,
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut returned,
        )
    };
    let query_error = std::io::Error::last_os_error();
    unsafe { CloseHandle(token) };
    if queried == 0 {
        return Err(format!("Could not read process access: {query_error}"));
    }
    Ok(elevation.TokenIsElevated != 0)
}

pub fn show_error_dialog(title: &str, message: &str) {
    let title = title.encode_utf16().chain([0]).collect::<Vec<_>>();
    let message = message.encode_utf16().chain([0]).collect::<Vec<_>>();
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            message.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONERROR | MB_SETFOREGROUND,
        );
    }
}

pub fn system_info() -> Result<SystemInfo, String> {
    let key = current_version_key()?;
    let mut product_name = key
        .get_value::<String, _>("ProductName")
        .unwrap_or_else(|_| "Windows".to_owned());
    let display_version = key
        .get_value::<String, _>("DisplayVersion")
        .unwrap_or_default();
    let build_number = key
        .get_value::<String, _>("CurrentBuildNumber")
        .unwrap_or_default();
    let build = build_number.parse::<u32>().unwrap_or_default();
    if build >= 22_000 && product_name.starts_with("Windows 10") {
        product_name = product_name.replacen("Windows 10", "Windows 11", 1);
    }
    Ok(SystemInfo {
        product_name,
        display_version,
        build_number,
        is_windows_11: build >= 22_000,
        is_elevated: is_elevated()?,
    })
}

#[cfg(test)]
mod tests {
    use super::edge_major_from_directory_name;

    #[test]
    fn parses_edge_version_directories() {
        assert_eq!(edge_major_from_directory_name("151.0.4129.78"), Some(151));
        assert_eq!(edge_major_from_directory_name("SetupMetrics"), None);
        assert_eq!(edge_major_from_directory_name("151.0"), None);
    }
}
