pub mod autostart;
pub mod hosts;
pub mod packages;
pub mod processes;
pub mod registry;
pub mod startup_task;
pub mod system_settings;
pub mod tasks;

use std::fs;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Output};

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_64KEY};

use crate::models::{SystemInfo, SystemSupport};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn run_hidden(program: &str, args: &[&str]) -> Result<Output, String> {
    Command::new(program)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|error| format!("Could not run {program}: {error}"))
}

pub fn run_powershell(script: &str) -> Result<String, String> {
    let output = run_hidden(
        "powershell.exe",
        &[
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ],
    )?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        return Err(if message.is_empty() {
            format!("PowerShell exited with {}", output.status)
        } else {
            message
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub fn run_powershell_json<T: serde::de::DeserializeOwned>(script: &str) -> Result<T, String> {
    let output = run_powershell(script)?;
    serde_json::from_str(&output).map_err(|error| format!("Invalid PowerShell response: {error}"))
}

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
    })
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
    let elevated = run_powershell(
        "$identity=[Security.Principal.WindowsIdentity]::GetCurrent(); $principal=[Security.Principal.WindowsPrincipal]::new($identity); $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator).ToString().ToLowerInvariant()",
    )? == "true";

    Ok(SystemInfo {
        product_name,
        display_version,
        build_number,
        is_windows_11: build >= 22_000,
        is_elevated: elevated,
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
