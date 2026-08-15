use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;
use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
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

pub fn list() -> Vec<ProcessInfo> {
    let system = System::new_all();
    let mut processes = system
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().into_owned(),
            executable_path: process
                .exe()
                .map(|path| path.to_string_lossy().into_owned())
                .unwrap_or_default(),
        })
        .collect::<Vec<_>>();
    processes.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
            .then(left.pid.cmp(&right.pid))
    });
    processes
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

fn stop_process(
    pid: Pid,
    process: &Process,
    rules: &[ProcessRule],
    is_allowed_once: &mut impl FnMut(&str) -> bool,
) -> Option<StoppedProcess> {
    if pid.as_u32() == std::process::id() {
        return None;
    }

    let process_name = process.name().to_string_lossy().into_owned();
    let matching_rules = rules
        .iter()
        .filter(|rule| matches(&process_name, process.exe(), rule))
        .collect::<Vec<_>>();
    if matching_rules.is_empty() {
        return None;
    }

    let process_key = allowance_key(matching_rules[0]);
    if is_allowed_once(&process_key) {
        return None;
    }

    let launch = process.exe().map(|executable_path| ProcessLaunch {
        executable_path: executable_path.to_owned(),
        arguments: process.cmd().iter().skip(1).cloned().collect(),
        working_directory: process.cwd().map(Path::to_owned),
    });
    if !process.kill() {
        return None;
    }

    Some(StoppedProcess {
        process_name,
        rule_name: matching_rules[0].name.clone(),
        rule_ids: matching_rules.iter().map(|rule| rule.id.clone()).collect(),
        allowance_key: process_key,
        launch,
    })
}

pub fn enforce(
    rules: &[ProcessRule],
    mut is_allowed_once: impl FnMut(&str) -> bool,
) -> Vec<StoppedProcess> {
    if rules.is_empty() {
        return Vec::new();
    }

    let system = System::new_all();
    system
        .processes()
        .iter()
        .filter_map(|(pid, process)| stop_process(*pid, process, rules, &mut is_allowed_once))
        .collect()
}

pub fn enforce_started(
    process_id: u32,
    process_name: &str,
    rules: &[ProcessRule],
    mut is_allowed_once: impl FnMut(&str) -> bool,
) -> Option<StoppedProcess> {
    let matching_names = rules
        .iter()
        .filter(|rule| rule.enabled && process_name.eq_ignore_ascii_case(&rule.executable_name))
        .collect::<Vec<_>>();
    if matching_names.is_empty() {
        return None;
    }
    let requires_path = matching_names
        .iter()
        .all(|rule| rule.executable_path.is_some());

    let pid = Pid::from_u32(process_id);
    let mut system = System::new();
    for attempt in 0..4 {
        system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[pid]),
            true,
            ProcessRefreshKind::nothing()
                .with_exe(UpdateKind::Always)
                .with_cmd(UpdateKind::Always)
                .with_cwd(UpdateKind::Always),
        );
        if let Some(process) = system.process(pid)
            && (!requires_path || process.exe().is_some())
        {
            return stop_process(pid, process, rules, &mut is_allowed_once);
        }
        if attempt < 3 {
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
    }
    None
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

        assert!(enforce(std::slice::from_ref(&process_rule), |_| true).is_empty());
        assert!(child.try_wait().unwrap().is_none());

        let stopped = enforce(std::slice::from_ref(&process_rule), |_| false);
        assert_eq!(stopped.len(), 1);
        child.wait().unwrap();
    }
}
