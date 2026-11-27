use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;
use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_INVALID_PARAMETER, WAIT_OBJECT_0};
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE, TerminateProcess, WaitForSingleObject,
};
use wmi::WMIConnection;

use crate::models::{ProcessInfo, ProcessRule};

#[derive(Clone, Debug)]
pub struct ProcessLaunch {
    executable_path: PathBuf,
    arguments: Vec<OsString>,
    working_directory: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct StoppedProcess {
    pub process_name: String,
    pub rule_name: String,
    pub rule_ids: Vec<String>,
    pub allowance_key: String,
    pub launch: Option<ProcessLaunch>,
}

#[derive(Deserialize)]
struct ProcessStartTrace {
    #[serde(rename = "ProcessID")]
    process_id: u32,
    #[serde(rename = "ProcessName")]
    process_name: String,
}

fn new_system() -> Result<System, String> {
    System::new().map_err(|error| format!("Could not read running processes: {error}"))
}

fn executable_refresh() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing().with_exe(UpdateKind::OnlyIfNotSet)
}

fn launch_refresh() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing()
        .with_exe(UpdateKind::Always)
        .with_cmd(UpdateKind::Always)
        .with_cwd(UpdateKind::Always)
}

pub fn list() -> Result<Vec<ProcessInfo>, String> {
    let mut system = new_system()?;
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, executable_refresh());
    let mut grouped = BTreeMap::<(String, String), ProcessInfo>::new();
    for process in system.processes().values() {
        let name = process.name().to_string_lossy().into_owned();
        let executable_path = process
            .exe()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_default();
        grouped
            .entry((name.to_lowercase(), executable_path.to_lowercase()))
            .and_modify(|existing| existing.instances += 1)
            .or_insert(ProcessInfo {
                name,
                executable_path,
                instances: 1,
            });
    }
    Ok(grouped.into_values().collect())
}

fn matches(process_name: &str, process_path: Option<&Path>, rule: &ProcessRule) -> bool {
    if !rule.enabled || !process_name.eq_ignore_ascii_case(&rule.executable_name) {
        return false;
    }
    match (&rule.executable_path, process_path) {
        (Some(wanted), Some(actual)) => {
            normalized_path(actual) == normalized_path(Path::new(wanted))
        }
        (Some(_), None) => false,
        (None, _) => true,
    }
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy()
        .trim_start_matches(r"\\?\")
        .replace('/', "\\")
        .to_ascii_lowercase()
}

fn allowance_key(rule: &ProcessRule) -> String {
    match &rule.executable_path {
        Some(path) => format!(
            "{}\0{}",
            rule.executable_name.to_ascii_lowercase(),
            normalized_path(Path::new(path))
        ),
        None => rule.executable_name.to_ascii_lowercase(),
    }
}

fn terminate(pid: Pid) -> io::Result<bool> {
    let handle = unsafe { OpenProcess(PROCESS_TERMINATE | PROCESS_SYNCHRONIZE, 0, pid.as_u32()) };
    if handle.is_null() {
        let error = io::Error::last_os_error();
        return if error.raw_os_error() == Some(ERROR_INVALID_PARAMETER as i32) {
            Ok(false)
        } else {
            Err(error)
        };
    }
    let result = if unsafe { TerminateProcess(handle, 1) } != 0 {
        Ok(true)
    } else if unsafe { WaitForSingleObject(handle, 0) } == WAIT_OBJECT_0 {
        Ok(false)
    } else {
        Err(io::Error::last_os_error())
    };
    unsafe { CloseHandle(handle) };
    result
}

fn stop_process(
    pid: Pid,
    process: &Process,
    rules: &[ProcessRule],
    is_allowed_once: &mut impl FnMut(&str) -> bool,
) -> Result<Option<StoppedProcess>, String> {
    if pid.as_u32() == std::process::id() {
        return Ok(None);
    }

    let process_name = process.name().to_string_lossy().into_owned();
    let matching_rules = rules
        .iter()
        .filter(|rule| matches(&process_name, process.exe(), rule))
        .collect::<Vec<_>>();
    let Some(primary_rule) = matching_rules.first() else {
        return Ok(None);
    };

    let process_key = allowance_key(primary_rule);
    if is_allowed_once(&process_key) {
        return Ok(None);
    }

    let launch = process.exe().map(|executable_path| ProcessLaunch {
        executable_path: executable_path.to_owned(),
        arguments: process.cmd().iter().skip(1).cloned().collect(),
        working_directory: process.cwd().map(Path::to_owned),
    });
    match terminate(pid) {
        Ok(true) => {}
        Ok(false) => return Ok(None),
        Err(error) => return Err(format!("Could not stop {process_name}: {error}")),
    }

    Ok(Some(StoppedProcess {
        process_name,
        rule_name: primary_rule.name.clone(),
        rule_ids: matching_rules.iter().map(|rule| rule.id.clone()).collect(),
        allowance_key: process_key,
        launch,
    }))
}

#[derive(Default)]
pub struct ProcessEnforcement {
    pub stopped: Vec<StoppedProcess>,
    pub errors: Vec<String>,
}

pub fn enforce(
    rules: &[ProcessRule],
    mut is_allowed_once: impl FnMut(&str) -> bool,
) -> ProcessEnforcement {
    let mut outcome = ProcessEnforcement::default();
    if rules.is_empty() {
        return outcome;
    }

    let mut system = match new_system() {
        Ok(system) => system,
        Err(error) => {
            outcome.errors.push(error);
            return outcome;
        }
    };
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, executable_refresh());
    let candidates = system
        .processes()
        .iter()
        .filter(|(_, process)| {
            let name = process.name().to_string_lossy();
            rules.iter().any(|rule| matches(&name, process.exe(), rule))
        })
        .map(|(pid, _)| *pid)
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return outcome;
    }

    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&candidates),
        true,
        launch_refresh(),
    );
    for pid in candidates {
        let Some(process) = system.process(pid) else {
            continue;
        };
        match stop_process(pid, process, rules, &mut is_allowed_once) {
            Ok(Some(stopped)) => outcome.stopped.push(stopped),
            Ok(None) => {}
            Err(error) => outcome.errors.push(error),
        }
    }
    outcome
}

pub fn enforce_started(
    process_id: u32,
    process_name: &str,
    rules: &[ProcessRule],
    mut is_allowed_once: impl FnMut(&str) -> bool,
) -> Result<Option<StoppedProcess>, String> {
    let matching_names = rules
        .iter()
        .filter(|rule| rule.enabled && process_name.eq_ignore_ascii_case(&rule.executable_name))
        .collect::<Vec<_>>();
    if matching_names.is_empty() {
        return Ok(None);
    }
    let requires_path = matching_names
        .iter()
        .all(|rule| rule.executable_path.is_some());

    let pid = Pid::from_u32(process_id);
    let mut system = new_system()?;
    for attempt in 0..4 {
        system.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), true, launch_refresh());
        if let Some(process) = system.process(pid)
            && (!requires_path || process.exe().is_some())
        {
            return stop_process(pid, process, rules, &mut is_allowed_once);
        }
        if attempt < 3 {
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
    }
    Ok(None)
}

pub fn launch(process: &ProcessLaunch) -> Result<(), String> {
    let mut command = Command::new(&process.executable_path);
    command.args(&process.arguments);
    if let Some(working_directory) = &process.working_directory {
        command.current_dir(working_directory);
    }
    command.spawn().map(|_| ()).map_err(|error| {
        format!(
            "Could not reopen {}: {error}",
            process.executable_path.display()
        )
    })
}

pub fn watch_starts(
    on_ready: impl FnOnce(),
    mut on_process_started: impl FnMut(u32, &str),
) -> Result<(), String> {
    let connection = WMIConnection::new()
        .map_err(|error| format!("Could not connect to process events: {error}"))?;
    let events = connection
        .raw_notification::<ProcessStartTrace>(
            "SELECT ProcessID, ProcessName FROM Win32_ProcessStartTrace",
        )
        .map_err(|error| format!("Could not subscribe to process events: {error}"))?;
    on_ready();

    for event in events {
        let event = event.map_err(|error| format!("Process event monitoring failed: {error}"))?;
        on_process_started(event.process_id, &event.process_name);
    }
    Err("Process event monitoring stopped".to_owned())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::process::Command;
    use std::time::Duration;

    use crate::models::ProcessRule;

    use super::{allowance_key, enforce, matches};

    fn rule(path: Option<&str>) -> ProcessRule {
        ProcessRule {
            id: "test".to_owned(),
            name: "Test".to_owned(),
            executable_name: "APP.EXE".to_owned(),
            executable_path: path.map(str::to_owned),
            enabled: true,
            built_in: false,
        }
    }

    #[test]
    fn matches_name_and_optional_path_case_insensitively() {
        let path_rule = rule(Some(r"C:\Apps\app.exe"));
        assert!(matches(
            "app.exe",
            Some(Path::new(r"c:\apps\APP.exe")),
            &path_rule
        ));
        assert!(!matches(
            "app.exe",
            Some(Path::new(r"c:\other\app.exe")),
            &path_rule
        ));
        assert!(matches(
            "app.exe",
            Some(Path::new(r"C:\anywhere\app.exe")),
            &rule(None)
        ));
    }

    #[test]
    fn disabled_rules_do_not_match() {
        let mut disabled = rule(None);
        disabled.enabled = false;

        assert!(!matches("app.exe", None, &disabled));
    }

    #[test]
    fn one_time_allowance_keys_include_the_executable_path() {
        assert_eq!(
            allowance_key(&rule(Some(r"\\?\C:\Apps\App.exe"))),
            "app.exe\0c:\\apps\\app.exe"
        );
        assert_eq!(allowance_key(&rule(None)), "app.exe");
    }

    #[test]
    fn process_rule_test_child() {
        if std::env::var_os("PREP_MY_WINDOWS_PROCESS_TEST_CHILD").is_some() {
            std::thread::sleep(Duration::from_secs(30));
        }
    }

    #[test]
    fn one_time_allowance_skips_one_matching_process() {
        let executable = std::env::current_exe().unwrap();
        let executable_name = executable
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let mut child = Command::new(&executable)
            .args([
                "--exact",
                "windows::processes::tests::process_rule_test_child",
            ])
            .env("PREP_MY_WINDOWS_PROCESS_TEST_CHILD", "1")
            .spawn()
            .unwrap();
        std::thread::sleep(Duration::from_millis(150));
        let process_rule = ProcessRule {
            id: "child".to_owned(),
            name: "Child".to_owned(),
            executable_name,
            executable_path: Some(executable.to_string_lossy().into_owned()),
            enabled: true,
            built_in: false,
        };

        let allowed = enforce(std::slice::from_ref(&process_rule), |_| true);
        assert!(allowed.stopped.is_empty() && allowed.errors.is_empty());
        assert!(child.try_wait().unwrap().is_none());

        let stopped = enforce(std::slice::from_ref(&process_rule), |_| false);
        assert_eq!(stopped.stopped.len(), 1);
        assert!(stopped.errors.is_empty());
        child.wait().unwrap();
    }
}
