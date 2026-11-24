use super::{DEFAULT_TIMEOUT, run_hidden};

const ERROR_SERVICE_DOES_NOT_EXIST: i32 = 1060;
const ERROR_SERVICE_NOT_ACTIVE: i32 = 1062;

pub fn stop(name: &str) -> Result<(), String> {
    let output = run_hidden("sc.exe", &["stop", name], DEFAULT_TIMEOUT)?;
    match output.status.code() {
        Some(0 | ERROR_SERVICE_DOES_NOT_EXIST | ERROR_SERVICE_NOT_ACTIVE) => Ok(()),
        _ => Err(format!(
            "Could not stop service {name}: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        )),
    }
}
