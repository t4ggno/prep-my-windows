use std::io;
use std::iter::once;
use std::ptr::{null, null_mut};

use windows_sys::Win32::System::Services::{
    ChangeServiceConfigW, CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceStatus,
    SC_HANDLE, SC_MANAGER_CONNECT, SERVICE_CHANGE_CONFIG, SERVICE_NO_CHANGE, SERVICE_QUERY_STATUS,
    SERVICE_RUNNING, SERVICE_STATUS,
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

pub fn is_installed(name: &str) -> bool {
    service_key(name)
        .and_then(|key| key.get_raw_value("ImagePath"))
        .is_ok()
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

struct ServiceHandle(SC_HANDLE);

impl Drop for ServiceHandle {
    fn drop(&mut self) {
        unsafe { CloseServiceHandle(self.0) };
    }
}

fn open_service(name: &str, access: u32) -> io::Result<ServiceHandle> {
    let manager = unsafe { OpenSCManagerW(null(), null(), SC_MANAGER_CONNECT) };
    if manager.is_null() {
        return Err(io::Error::last_os_error());
    }
    let manager = ServiceHandle(manager);
    let wide_name = name.encode_utf16().chain(once(0)).collect::<Vec<_>>();
    let service = unsafe { OpenServiceW(manager.0, wide_name.as_ptr(), access) };
    if service.is_null() {
        return Err(io::Error::last_os_error());
    }
    Ok(ServiceHandle(service))
}

pub fn set_start_type(name: &str, start_type: u32) -> Result<(), String> {
    let service = open_service(name, SERVICE_CHANGE_CONFIG)
        .map_err(|error| format!("Could not open service {name}: {error}"))?;
    let changed = unsafe {
        ChangeServiceConfigW(
            service.0,
            SERVICE_NO_CHANGE,
            start_type,
            SERVICE_NO_CHANGE,
            null(),
            null(),
            null_mut(),
            null(),
            null(),
            null(),
            null(),
        )
    };
    if changed == 0 {
        return Err(format!(
            "Could not change service {name}: {}",
            io::Error::last_os_error()
        ));
    }
    Ok(())
}

pub fn is_running(name: &str) -> Result<bool, String> {
    let service = match open_service(name, SERVICE_QUERY_STATUS) {
        Ok(service) => service,
        Err(error) if error.raw_os_error() == Some(ERROR_SERVICE_DOES_NOT_EXIST) => {
            return Ok(false);
        }
        Err(error) => return Err(format!("Could not read service {name}: {error}")),
    };
    let mut status = SERVICE_STATUS::default();
    if unsafe { QueryServiceStatus(service.0, &mut status) } == 0 {
        return Err(format!(
            "Could not read service {name}: {}",
            io::Error::last_os_error()
        ));
    }
    Ok(status.dwCurrentState == SERVICE_RUNNING)
}

#[cfg(test)]
mod tests {
    use super::{is_driver, is_installed, is_running, set_start_type};

    #[test]
    fn reports_missing_services_as_not_running() {
        assert_eq!(is_running("PrepMyWindowsMissingService"), Ok(false));
    }

    #[test]
    fn reports_missing_services_when_changing_start_type() {
        assert!(set_start_type("PrepMyWindowsMissingService", 4).is_err());
    }

    #[test]
    fn detects_installed_services() {
        assert!(is_installed("EventLog"));
        assert!(!is_installed("PrepMyWindowsMissingService"));
    }

    #[test]
    fn recognizes_kernel_drivers() {
        assert!(is_driver("Tcpip"));
        assert!(!is_driver("EventLog"));
    }
}
