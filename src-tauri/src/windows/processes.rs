use std::path::Path;

use sysinfo::System;

use crate::models::{ProcessInfo, ProcessRule};

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
    if !process_name.eq_ignore_ascii_case(&rule.executable_name) {
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

pub fn enforce(rules: &[ProcessRule]) -> Vec<String> {
    let system = System::new_all();
    let current_pid = std::process::id();
    let mut killed = Vec::new();

    for (pid, process) in system.processes() {
        if pid.as_u32() == current_pid {
            continue;
        }
        let process_name = process.name().to_string_lossy();
        if rules
            .iter()
            .any(|rule| matches(&process_name, process.exe(), rule))
            && process.kill()
        {
            killed.push(format!("{} ({})", process_name, pid.as_u32()));
        }
    }

    killed
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::models::ProcessRule;

    use super::matches;

    #[test]
    fn matches_name_and_optional_path_case_insensitively() {
        let rule = ProcessRule {
            id: "test".to_owned(),
            name: "Test".to_owned(),
            executable_name: "APP.EXE".to_owned(),
            executable_path: Some(r"C:\Apps\app.exe".to_owned()),
            enabled: true,
            built_in: false,
        };
        assert!(matches(
            "app.exe",
            Some(Path::new(r"c:\apps\APP.exe")),
            &rule
        ));
        assert!(!matches(
            "app.exe",
            Some(Path::new(r"c:\other\app.exe")),
            &rule
        ));
        assert!(matches(
            "app.exe",
            Some(Path::new(r"\\?\C:\Apps\app.exe")),
            &rule
        ));
    }
}
