use std::ffi::c_void;
use std::fs;
use std::mem::size_of;
use std::path::PathBuf;

use windows_sys::Win32::System::Power::{GetPwrCapabilities, SYSTEM_POWER_CAPABILITIES};
use windows_sys::Win32::System::SystemServices::{HIBERFILE_TYPE_FULL, HIBERFILE_TYPE_REDUCED};
use windows_sys::Win32::UI::Accessibility::{
    FILTERKEYS, SKF_AVAILABLE, SKF_HOTKEYACTIVE, SKF_STICKYKEYSON, STICKYKEYS, TOGGLEKEYS,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    FKF_AVAILABLE, FKF_FILTERKEYSON, FKF_HOTKEYACTIVE, SPI_GETFILTERKEYS, SPI_GETSTICKYKEYS,
    SPI_GETTOGGLEKEYS, SPI_SETFILTERKEYS, SPI_SETSTICKYKEYS, SPI_SETTOGGLEKEYS, SPIF_SENDCHANGE,
    SPIF_UPDATEINIFILE, SystemParametersInfoW, TKF_AVAILABLE, TKF_HOTKEYACTIVE, TKF_TOGGLEKEYSON,
};

use crate::models::{
    PolicyDefinition, RegistryHive, RegistryValue, SupportRequirement, SystemSettingKind,
    SystemSettingState,
};

const WIDGETS_PACKAGE_PREFIX: &str = "MicrosoftWindows.Client.WebExperience_";

fn system_parameters(
    action: u32,
    size: u32,
    value: *mut c_void,
    update_flags: u32,
    name: &str,
) -> Result<(), String> {
    if unsafe { SystemParametersInfoW(action, size, value, update_flags) } == 0 {
        Err(format!(
            "Could not update {name}: {}",
            std::io::Error::last_os_error()
        ))
    } else {
        Ok(())
    }
}

fn shortcut_state(available: bool, enabled: bool, hotkey_active: bool) -> SystemSettingState {
    if !available {
        return SystemSettingState {
            available: false,
            unavailable_reason: Some("Not available on this system".to_owned()),
            compliant: false,
            current: "Unavailable".to_owned(),
            wanted: "Disabled when unused".to_owned(),
        };
    }
    if enabled {
        return SystemSettingState {
            available: true,
            unavailable_reason: None,
            compliant: true,
            current: "In use".to_owned(),
            wanted: "Preserved while in use".to_owned(),
        };
    }
    SystemSettingState {
        available: true,
        unavailable_reason: None,
        compliant: !hotkey_active,
        current: if hotkey_active { "Enabled" } else { "Disabled" }.to_owned(),
        wanted: "Disabled".to_owned(),
    }
}

fn disabled_shortcut_flags(flags: u32, enabled_flag: u32, hotkey_flag: u32) -> Option<u32> {
    (flags & enabled_flag == 0 && flags & hotkey_flag != 0).then_some(flags & !hotkey_flag)
}

fn sticky_keys() -> Result<STICKYKEYS, String> {
    let mut keys = STICKYKEYS {
        cbSize: size_of::<STICKYKEYS>() as u32,
        ..STICKYKEYS::default()
    };
    system_parameters(
        SPI_GETSTICKYKEYS,
        keys.cbSize,
        &mut keys as *mut STICKYKEYS as *mut c_void,
        0,
        "StickyKeys",
    )?;
    Ok(keys)
}

fn filter_keys() -> Result<FILTERKEYS, String> {
    let mut keys = FILTERKEYS {
        cbSize: size_of::<FILTERKEYS>() as u32,
        ..FILTERKEYS::default()
    };
    system_parameters(
        SPI_GETFILTERKEYS,
        keys.cbSize,
        &mut keys as *mut FILTERKEYS as *mut c_void,
        0,
        "FilterKeys",
    )?;
    Ok(keys)
}

fn toggle_keys() -> Result<TOGGLEKEYS, String> {
    let mut keys = TOGGLEKEYS {
        cbSize: size_of::<TOGGLEKEYS>() as u32,
        ..TOGGLEKEYS::default()
    };
    system_parameters(
        SPI_GETTOGGLEKEYS,
        keys.cbSize,
        &mut keys as *mut TOGGLEKEYS as *mut c_void,
        0,
        "ToggleKeys",
    )?;
    Ok(keys)
}

fn read_sticky_keys() -> Result<SystemSettingState, String> {
    let keys = sticky_keys()?;
    Ok(shortcut_state(
        keys.dwFlags & SKF_AVAILABLE != 0,
        keys.dwFlags & SKF_STICKYKEYSON != 0,
        keys.dwFlags & SKF_HOTKEYACTIVE != 0,
    ))
}

fn read_filter_keys() -> Result<SystemSettingState, String> {
    let keys = filter_keys()?;
    Ok(shortcut_state(
        keys.dwFlags & FKF_AVAILABLE != 0,
        keys.dwFlags & FKF_FILTERKEYSON != 0,
        keys.dwFlags & FKF_HOTKEYACTIVE != 0,
    ))
}

fn read_toggle_keys() -> Result<SystemSettingState, String> {
    let keys = toggle_keys()?;
    Ok(shortcut_state(
        keys.dwFlags & TKF_AVAILABLE != 0,
        keys.dwFlags & TKF_TOGGLEKEYSON != 0,
        keys.dwFlags & TKF_HOTKEYACTIVE != 0,
    ))
}

fn enforce_sticky_keys() -> Result<bool, String> {
    let mut keys = sticky_keys()?;
    let Some(flags) = disabled_shortcut_flags(keys.dwFlags, SKF_STICKYKEYSON, SKF_HOTKEYACTIVE)
    else {
        return Ok(false);
    };
    keys.dwFlags = flags;
    system_parameters(
        SPI_SETSTICKYKEYS,
        keys.cbSize,
        &mut keys as *mut STICKYKEYS as *mut c_void,
        SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        "StickyKeys",
    )?;
    Ok(true)
}

fn enforce_filter_keys() -> Result<bool, String> {
    let mut keys = filter_keys()?;
    let Some(flags) = disabled_shortcut_flags(keys.dwFlags, FKF_FILTERKEYSON, FKF_HOTKEYACTIVE)
    else {
        return Ok(false);
    };
    keys.dwFlags = flags;
    system_parameters(
        SPI_SETFILTERKEYS,
        keys.cbSize,
        &mut keys as *mut FILTERKEYS as *mut c_void,
        SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        "FilterKeys",
    )?;
    Ok(true)
}

fn enforce_toggle_keys() -> Result<bool, String> {
    let mut keys = toggle_keys()?;
    let Some(flags) = disabled_shortcut_flags(keys.dwFlags, TKF_TOGGLEKEYSON, TKF_HOTKEYACTIVE)
    else {
        return Ok(false);
    };
    keys.dwFlags = flags;
    system_parameters(
        SPI_SETTOGGLEKEYS,
        keys.cbSize,
        &mut keys as *mut TOGGLEKEYS as *mut c_void,
        SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        "ToggleKeys",
    )?;
    Ok(true)
}

fn power_capabilities() -> Result<SYSTEM_POWER_CAPABILITIES, String> {
    let mut capabilities = SYSTEM_POWER_CAPABILITIES::default();
    if unsafe { GetPwrCapabilities(&mut capabilities) } {
        Ok(capabilities)
    } else {
        Err(format!(
            "Could not read power capabilities: {}",
            std::io::Error::last_os_error()
        ))
    }
}

fn hibernation_state(supported: bool, file_present: bool, file_type: u8) -> SystemSettingState {
    if !supported {
        return SystemSettingState {
            available: false,
            unavailable_reason: Some("System firmware does not support hibernation".to_owned()),
            compliant: false,
            current: "Unavailable".to_owned(),
            wanted: "Enabled".to_owned(),
        };
    }
    let full_file = file_present && file_type == HIBERFILE_TYPE_FULL as u8;
    SystemSettingState {
        available: true,
        unavailable_reason: None,
        compliant: full_file,
        current: if full_file {
            "Enabled"
        } else if file_present && file_type == HIBERFILE_TYPE_REDUCED as u8 {
            "Fast startup only"
        } else {
            "Disabled"
        }
        .to_owned(),
        wanted: "Enabled".to_owned(),
    }
}

fn read_hibernation() -> Result<SystemSettingState, String> {
    let capabilities = power_capabilities()?;
    Ok(hibernation_state(
        capabilities.SystemS4,
        capabilities.HiberFilePresent,
        capabilities.HiberFileType,
    ))
}

fn run_powercfg(arguments: &[&str]) -> Result<(), String> {
    let output = super::run_hidden("powercfg.exe", arguments)?;
    if output.status.success() {
        return Ok(());
    }
    let error = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    Err(if error.is_empty() {
        format!(
            "Could not enable hibernation: powercfg exited with {}",
            output.status
        )
    } else {
        format!("Could not enable hibernation: {error}")
    })
}

fn enforce_hibernation() -> Result<bool, String> {
    let current = read_hibernation()?;
    if !current.available || current.compliant {
        return Ok(false);
    }
    run_powercfg(&["/hibernate", "on"])?;
    run_powercfg(&["/hibernate", "/type", "full"])?;
    if !read_hibernation()?.compliant {
        return Err(
            "Could not enable hibernation: Windows did not create the hibernation file".to_owned(),
        );
    }
    Ok(true)
}

fn widgets_package_present() -> Result<bool, String> {
    let program_files = std::env::var_os("ProgramFiles")
        .ok_or_else(|| "Could not resolve the Program Files directory".to_owned())?;
    let packages_directory = PathBuf::from(program_files).join("WindowsApps");
    let entries = fs::read_dir(&packages_directory).map_err(|error| {
        format!(
            "Could not inspect Widgets package state in {}: {error}",
            packages_directory.display()
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            format!(
                "Could not inspect Widgets package state in {}: {error}",
                packages_directory.display()
            )
        })?;
        if entry
            .file_name()
            .to_string_lossy()
            .to_ascii_lowercase()
            .starts_with(&WIDGETS_PACKAGE_PREFIX.to_ascii_lowercase())
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn widgets_taskbar_policy() -> PolicyDefinition {
    PolicyDefinition {
        id: "widgets-taskbar-button",
        name: "Widgets taskbar button",
        category: "Windows behavior",
        hive: RegistryHive::CurrentUser,
        path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        value_name: "TaskbarDa",
        desired: RegistryValue::Dword(0),
        support: SupportRequirement::windows_build(22_000),
    }
}

fn widgets_taskbar_state(
    widgets_package_present: bool,
    taskbar_value: Option<&str>,
) -> SystemSettingState {
    let hidden = !widgets_package_present || taskbar_value == Some("0");
    SystemSettingState {
        available: true,
        unavailable_reason: None,
        compliant: hidden,
        current: if hidden { "Hidden" } else { "Visible" }.to_owned(),
        wanted: "Hidden".to_owned(),
    }
}

fn read_widgets_taskbar_button() -> Result<SystemSettingState, String> {
    let package_present = widgets_package_present()?;
    let taskbar_value = if package_present {
        super::registry::read_policy(&widgets_taskbar_policy())?
    } else {
        None
    };
    Ok(widgets_taskbar_state(
        package_present,
        taskbar_value.as_deref(),
    ))
}

fn enforce_widgets_taskbar_button() -> Result<bool, String> {
    if !widgets_package_present()? {
        return Ok(false);
    }
    super::registry::enforce_policy(&widgets_taskbar_policy())
}

pub fn read(kind: SystemSettingKind) -> Result<SystemSettingState, String> {
    match kind {
        SystemSettingKind::Hibernation => read_hibernation(),
        SystemSettingKind::StickyKeysShortcut => read_sticky_keys(),
        SystemSettingKind::FilterKeysShortcut => read_filter_keys(),
        SystemSettingKind::ToggleKeysShortcut => read_toggle_keys(),
        SystemSettingKind::WidgetsTaskbarButton => read_widgets_taskbar_button(),
    }
}

pub fn enforce(kind: SystemSettingKind) -> Result<bool, String> {
    match kind {
        SystemSettingKind::Hibernation => enforce_hibernation(),
        SystemSettingKind::StickyKeysShortcut => enforce_sticky_keys(),
        SystemSettingKind::FilterKeysShortcut => enforce_filter_keys(),
        SystemSettingKind::ToggleKeysShortcut => enforce_toggle_keys(),
        SystemSettingKind::WidgetsTaskbarButton => enforce_widgets_taskbar_button(),
    }
}

#[cfg(test)]
mod tests {
    use windows_sys::Win32::System::SystemServices::{HIBERFILE_TYPE_FULL, HIBERFILE_TYPE_REDUCED};

    use super::{
        disabled_shortcut_flags, hibernation_state, shortcut_state, widgets_taskbar_state,
    };

    #[test]
    fn disables_only_an_unused_active_shortcut() {
        assert_eq!(disabled_shortcut_flags(0b1110, 1, 0b100), Some(0b1010));
        assert_eq!(disabled_shortcut_flags(0b1111, 1, 0b100), None);
        assert_eq!(disabled_shortcut_flags(0b1010, 1, 0b100), None);
    }

    #[test]
    fn treats_an_enabled_accessibility_feature_as_compliant() {
        let state = shortcut_state(true, true, true);

        assert!(state.available);
        assert!(state.compliant);
        assert_eq!(state.current, "In use");
        assert_eq!(state.wanted, "Preserved while in use");
    }

    #[test]
    fn requires_a_full_hibernation_file() {
        let reduced = hibernation_state(true, true, HIBERFILE_TYPE_REDUCED as u8);
        let full = hibernation_state(true, true, HIBERFILE_TYPE_FULL as u8);

        assert!(!reduced.compliant);
        assert_eq!(reduced.current, "Fast startup only");
        assert!(full.compliant);
    }

    #[test]
    fn treats_a_removed_widgets_package_as_a_hidden_taskbar_button() {
        let removed = widgets_taskbar_state(false, None);
        let visible = widgets_taskbar_state(true, None);
        let hidden = widgets_taskbar_state(true, Some("0"));

        assert!(removed.compliant);
        assert_eq!(removed.current, "Hidden");
        assert!(!visible.compliant);
        assert!(hidden.compliant);
    }
}
