use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use winreg::RegKey;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE, KEY_WOW64_64KEY,
};
use winreg::types::FromRegValue;

use crate::models::{AutostartEntry, AutostartKind, AutostartRule};

use super::{run_hidden, run_powershell, run_powershell_json};

#[derive(Clone, Copy)]
enum RootHive {
    CurrentUser,
    LocalMachine,
}

impl RootHive {
    fn key(self) -> RegKey {
        match self {
            Self::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
            Self::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::CurrentUser => "HKCU",
            Self::LocalMachine => "HKLM",
        }
    }
}

#[derive(Clone, Copy)]
struct RegistryLocation {
    hive: RootHive,
    path: &'static str,
    source: &'static str,
    view: u32,
}

fn registry_locations() -> Vec<RegistryLocation> {
    use RootHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        RegistryLocation {
            hive: Cu,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            source: "Current user Run",
            view: KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: Cu,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
            source: "Current user RunOnce",
            view: KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: Cu,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\Run",
            source: "Current user policy Run",
            view: KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: Lm,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            source: "All users Run",
            view: KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: Lm,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
            source: "All users RunOnce",
            view: KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: Lm,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\Run",
            source: "All users policy Run",
            view: KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: Lm,
            path: r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
            source: "32-bit all users Run",
            view: KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: Lm,
            path: r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\RunOnce",
            source: "32-bit all users RunOnce",
            view: KEY_WOW64_64KEY,
        },
    ]
}

fn registry_entries() -> Result<Vec<AutostartEntry>, String> {
    let mut entries = Vec::new();

    for location in registry_locations() {
        let key = match location
            .hive
            .key()
            .open_subkey_with_flags(location.path, KEY_READ | location.view)
        {
            Ok(key) => key,
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(format!("Could not read {}: {error}", location.source)),
        };

        for result in key.enum_values() {
            let (name, value) =
                result.map_err(|error| format!("Could not read {}: {error}", location.source))?;
            let command =
                String::from_reg_value(&value).unwrap_or_else(|_| "Binary value".to_owned());
            let path = format!("{}\\{}", location.hive.label(), location.path);
            entries.push(AutostartEntry {
                id: format!("registry:{path}:{name}"),
                name,
                command,
                kind: AutostartKind::Registry,
                source: location.source.to_owned(),
                location: path,
                state: "Enabled".to_owned(),
            });
        }
    }

    Ok(entries)
}

fn startup_folders() -> Vec<(PathBuf, &'static str)> {
    let current = std::env::var_os("APPDATA").map(PathBuf::from).map(|path| {
        (
            path.join(r"Microsoft\Windows\Start Menu\Programs\Startup"),
            "Current user Startup folder",
        )
    });
    let all_users = std::env::var_os("ProgramData")
        .map(PathBuf::from)
        .map(|path| {
            (
                path.join(r"Microsoft\Windows\Start Menu\Programs\Startup"),
                "All users Startup folder",
            )
        });
    current.into_iter().chain(all_users).collect()
}

fn folder_entries() -> Result<Vec<AutostartEntry>, String> {
    let mut entries = Vec::new();

    for (folder, source) in startup_folders() {
        if !folder.exists() {
            continue;
        }
        for result in fs::read_dir(&folder)
            .map_err(|error| format!("Could not read {}: {error}", folder.display()))?
        {
            let entry = result.map_err(|error| format!("Could not read startup item: {error}"))?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            let location = path.to_string_lossy().into_owned();
            entries.push(AutostartEntry {
                id: format!("folder:{location}"),
                name,
                command: location.clone(),
                kind: AutostartKind::StartupFolder,
                source: source.to_owned(),
                location,
                state: "Enabled".to_owned(),
            });
        }
    }

    Ok(entries)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PowerShellAutostart {
    name: String,
    command: String,
    source: String,
    location: String,
    state: String,
}

fn scheduled_task_entries() -> Result<Vec<AutostartEntry>, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$items = [System.Collections.Generic.List[object]]::new()
foreach ($task in @(Get-ScheduledTask)) {
    $startupTrigger = @($task.Triggers | Where-Object { $_.CimClass.CimClassName -in @('MSFT_TaskBootTrigger', 'MSFT_TaskLogonTrigger', 'MSFT_TaskStartupTrigger') }).Count -gt 0
    if (-not $startupTrigger) { continue }
    foreach ($action in @($task.Actions | Where-Object { $_.Execute })) {
        $items.Add([PSCustomObject]@{
            Name = $task.TaskName
            Command = "$($action.Execute) $($action.Arguments)".Trim()
            Source = 'Scheduled task'
            Location = "$($task.TaskPath)$($task.TaskName)"
            State = [string]$task.State
        })
    }
}
ConvertTo-Json -InputObject @($items) -Compress
"#;
    let entries: Vec<PowerShellAutostart> = run_powershell_json(script)?;
    Ok(entries
        .into_iter()
        .map(|entry| AutostartEntry {
            id: format!("task:{}", entry.location),
            name: entry.name,
            command: entry.command,
            kind: AutostartKind::ScheduledTask,
            source: entry.source,
            location: entry.location,
            state: entry.state,
        })
        .collect())
}

fn service_entries() -> Result<Vec<AutostartEntry>, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$items = @((Get-CimInstance Win32_Service | Where-Object { $_.StartMode -eq 'Auto' } | ForEach-Object {
    [PSCustomObject]@{
        Name = $_.DisplayName
        Command = [string]$_.PathName
        Source = 'Automatic service'
        Location = $_.Name
        State = $_.State
    }
}))
ConvertTo-Json -InputObject @($items) -Compress
"#;
    let entries: Vec<PowerShellAutostart> = run_powershell_json(script)?;
    Ok(entries
        .into_iter()
        .map(|entry| AutostartEntry {
            id: format!("service:{}", entry.location),
            name: entry.name,
            command: entry.command,
            kind: AutostartKind::Service,
            source: entry.source,
            location: entry.location,
            state: entry.state,
        })
        .collect())
}

pub fn list() -> Result<Vec<AutostartEntry>, String> {
    let mut entries = registry_entries()?;
    entries.extend(folder_entries()?);
    entries.extend(scheduled_task_entries()?);
    entries.extend(service_entries()?);
    entries.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
            .then(left.source.cmp(&right.source))
    });
    Ok(entries)
}

fn matching_registry_location(location: &str) -> Option<RegistryLocation> {
    registry_locations().into_iter().find(|candidate| {
        format!("{}\\{}", candidate.hive.label(), candidate.path).eq_ignore_ascii_case(location)
    })
}

fn remove_registry_value(rule: &AutostartRule) -> Result<bool, String> {
    let Some(location) = matching_registry_location(&rule.location) else {
        return Err("Unknown autostart registry location".to_owned());
    };
    let key = match location
        .hive
        .key()
        .open_subkey_with_flags(location.path, KEY_READ | KEY_SET_VALUE | location.view)
    {
        Ok(key) => key,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(format!("Could not open {}: {error}", rule.location)),
    };
    match key.delete_value(&rule.name) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(format!("Could not remove {}: {error}", rule.name)),
    }
}

fn is_startup_folder_file(path: &Path) -> bool {
    let parent = path.parent();
    startup_folders().iter().any(|(folder, _)| {
        parent
            .map(|candidate| {
                candidate
                    .to_string_lossy()
                    .eq_ignore_ascii_case(&folder.to_string_lossy())
            })
            .unwrap_or(false)
    })
}

fn remove_startup_file(rule: &AutostartRule) -> Result<bool, String> {
    let path = PathBuf::from(&rule.location);
    if !is_startup_folder_file(&path) {
        return Err("Unknown startup folder location".to_owned());
    }
    if !path.exists() {
        return Ok(false);
    }
    fs::remove_file(&path)
        .map_err(|error| format!("Could not remove {}: {error}", path.display()))?;
    Ok(true)
}

fn disable_task(rule: &AutostartRule) -> Result<bool, String> {
    let task_path = rule.location.replace('\'', "''");
    let script = format!(
        "$ErrorActionPreference = 'Stop'; $task = Get-ScheduledTask -ErrorAction Stop | Where-Object {{ \"$($_.TaskPath)$($_.TaskName)\" -eq '{task_path}' }} | Select-Object -First 1; if ($null -eq $task) {{ 'missing' }} elseif ($task.State -eq 'Disabled') {{ 'disabled' }} else {{ Disable-ScheduledTask -InputObject $task -ErrorAction Stop | Out-Null; 'changed' }}"
    );
    match run_powershell(&script)?.as_str() {
        "changed" => Ok(true),
        "disabled" | "missing" => Ok(false),
        value => Err(format!("Unexpected scheduled task state: {value}")),
    }
}

fn disable_service(rule: &AutostartRule) -> Result<bool, String> {
    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!(r"SYSTEM\CurrentControlSet\Services\{}", rule.location);
    let key = match local_machine.open_subkey_with_flags(&path, KEY_READ | KEY_SET_VALUE) {
        Ok(key) => key,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(format!("Could not open service {}: {error}", rule.name)),
    };
    let start = key.get_value::<u32, _>("Start").unwrap_or_default();
    if start == 4 {
        return Ok(false);
    }
    key.set_value("Start", &4_u32)
        .map_err(|error| format!("Could not disable service {}: {error}", rule.name))?;
    let _ = run_hidden("sc.exe", &["stop", &rule.location]);
    Ok(true)
}

pub fn enforce_rule(rule: &AutostartRule) -> Result<bool, String> {
    match rule.kind {
        AutostartKind::Registry => remove_registry_value(rule),
        AutostartKind::StartupFolder => remove_startup_file(rule),
        AutostartKind::ScheduledTask => disable_task(rule),
        AutostartKind::Service => disable_service(rule),
    }
}

pub fn rule_from_entry(entry: &AutostartEntry) -> AutostartRule {
    AutostartRule {
        id: entry.id.clone(),
        name: entry.name.clone(),
        kind: entry.kind,
        location: entry.location.clone(),
    }
}
