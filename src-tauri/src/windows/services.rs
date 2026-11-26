use std::io;
use std::iter::once;
use std::ptr::null;

use windows_sys::Win32::System::Services::{
    CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceStatus, SC_MANAGER_CONNECT,
    SERVICE_QUERY_STATUS, SERVICE_RUNNING, SERVICE_STATUS,
};
use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_64KEY};

use super::{DEFAULT_TIMEOUT, run_hidden};

const ERROR_SERVICE_DOES_NOT_EXIST: i32 = 1060;
const ERROR_SERVICE_NOT_ACTIVE: i32 = 1062;
const SERVICE_DRIVER_TYPES: u32 = 0x0000_000B;

fn service_key(name: &str) -> io::Result<RegKey> {
    RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
        format!(r"SYSTEM\CurrentControlSet\Services\{name}"),
        KEY_READ | KEY_WOW64_64KEY,
    )
}

fn is_driver(name: &str) -> bool {
    service_key(name)
        .and_then(|key| key.get_value::<u32, _>("Type"))
        .is_ok_and(|service_type| service_type & SERVICE_DRIVER_TYPES != 0)
}

pub fn stop(name: &str) -> Result<(), String> {
    if is_driver(name) {
        return Ok(());
    }
    let output = run_hidden("sc.exe", &["stop", name], DEFAULT_TIMEOUT)?;
    match output.status.code() {
        Some(0 | ERROR_SERVICE_DOES_NOT_EXIST | ERROR_SERVICE_NOT_ACTIVE) => Ok(()),
        _ => Err(format!(
            "Could not stop service {name}: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        )),
    }
}

pub fn is_running(name: &str) -> Result<bool, String> {
    let manager = unsafe { OpenSCManagerW(null(), null(), SC_MANAGER_CONNECT) };
    if manager.is_null() {
        return Err(format!(
            "Could not open the service manager: {}",
            io::Error::last_os_error()
        ));
    }
    let wide_name = name.encode_utf16().chain(once(0)).collect::<Vec<_>>();
    let service = unsafe { OpenServiceW(manager, wide_name.as_ptr(), SERVICE_QUERY_STATUS) };
    let result = if service.is_null() {
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(ERROR_SERVICE_DOES_NOT_EXIST) {
            Ok(false)
        } else {
            Err(format!("Could not read service {name}: {error}"))
        }
    } else {
        let mut status = SERVICE_STATUS::default();
        let queried = unsafe { QueryServiceStatus(service, &mut status) };
        let error = io::Error::last_os_error();
        unsafe { CloseServiceHandle(service) };
        if queried == 0 {
            Err(format!("Could not read service {name}: {error}"))
        } else {
            Ok(status.dwCurrentState == SERVICE_RUNNING)
        }
    };
    unsafe { CloseServiceHandle(manager) };
    result
}

#[cfg(test)]
mod tests {
    use super::{is_driver, is_running};

    #[test]
    fn reports_missing_services_as_not_running() {
        assert_eq!(is_running("PrepMyWindowsMissingService"), Ok(false));
    }

    #[test]
    fn recognizes_kernel_drivers() {
        assert!(is_driver("Tcpip"));
        assert!(!is_driver("EventLog"));
    }
}
