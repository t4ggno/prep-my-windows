use std::collections::HashMap;
use std::time::Duration;

use serde::Deserialize;

use crate::models::{ScheduledTaskDefinition, TaskStatus};

use super::{
    DEFAULT_TIMEOUT, powershell_string_array, run_hidden, run_powershell, run_powershell_json,
};

const SYSTEM_HELPER_TIMEOUT: Duration = Duration::from_secs(16 * 60);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TaskState {
    path: String,
    state: String,
}

#[derive(Debug, Default)]
pub struct DisableOutcome {
    pub disabled: u64,
    pub errors: Vec<String>,
}

pub fn states(
    definitions: &[ScheduledTaskDefinition],
) -> Result<HashMap<String, TaskStatus>, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$items = @((Get-ScheduledTask | ForEach-Object {
    [PSCustomObject]@{
        Path = "$($_.TaskPath)$($_.TaskName)"
        State = [string]$_.State
    }
}))
ConvertTo-Json -InputObject @($items) -Compress
"#;
    let states: Vec<TaskState> = run_powershell_json(script, DEFAULT_TIMEOUT)?;
    let by_path = states
        .into_iter()
        .map(|state| {
            let status = if state.state == "Disabled" {
                TaskStatus::Disabled
            } else {
                TaskStatus::Enabled
            };
            (state.path.to_ascii_lowercase(), status)
        })
        .collect::<HashMap<_, _>>();
    Ok(definitions
        .iter()
        .map(|definition| {
            (
                definition.id.to_owned(),
                by_path
                    .get(&definition.path.to_ascii_lowercase())
                    .copied()
                    .unwrap_or(TaskStatus::Missing),
            )
        })
        .collect())
}

fn disable_directly(path: &str) -> Result<(), String> {
    let output = run_hidden(
        "schtasks.exe",
        &["/Change", "/TN", path, "/Disable"],
        DEFAULT_TIMEOUT,
    )?;
    if output.status.success() {
        return Ok(());
    }
    let message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let reason = message.trim_start_matches("ERROR:").trim();
    Err(if reason.is_empty() {
        format!("schtasks exited with {}", output.status)
    } else {
        reason.to_owned()
    })
}

fn disable_as_system(paths: &[&str]) -> Result<(), String> {
    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$paths = @({paths})
$name = "Prep My Windows helper $([guid]::NewGuid())"
$actions = @($paths | ForEach-Object {{ New-ScheduledTaskAction -Execute 'schtasks.exe' -Argument ('/Change /TN "{{0}}" /Disable' -f $_) }})
$principal = New-ScheduledTaskPrincipal -UserId 'S-1-5-18' -LogonType ServiceAccount -RunLevel Highest
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Minutes 15)
Register-ScheduledTask -TaskName $name -TaskPath '\' -Action $actions -Principal $principal -Settings $settings | Out-Null
try {{
    Start-ScheduledTask -TaskName $name -TaskPath '\'
    $deadline = (Get-Date).AddMinutes(15)
    do {{
        Start-Sleep -Seconds 1
        $task = Get-ScheduledTask -TaskName $name -TaskPath '\'
        $info = $task | Get-ScheduledTaskInfo
    }} while (($task.State -eq 'Running' -or $info.LastTaskResult -eq 0x41303) -and (Get-Date) -lt $deadline)
}} finally {{
    Unregister-ScheduledTask -TaskName $name -TaskPath '\' -Confirm:$false
}}
"#,
        paths = powershell_string_array(paths)
    );
    run_powershell(&script, SYSTEM_HELPER_TIMEOUT).map(|_| ())
}

pub fn disable(definitions: &[ScheduledTaskDefinition]) -> DisableOutcome {
    let mut outcome = DisableOutcome::default();
    let mut refused = Vec::new();
    for definition in definitions {
        match disable_directly(definition.path) {
            Ok(()) => outcome.disabled += 1,
            Err(reason) => refused.push((definition.clone(), reason)),
        }
    }
    if refused.is_empty() {
        return outcome;
    }

    let paths = refused
        .iter()
        .map(|(definition, _)| definition.path)
        .collect::<Vec<_>>();
    let helper_error = disable_as_system(&paths).err();
    let refused_definitions = refused
        .iter()
        .map(|(definition, _)| definition.clone())
        .collect::<Vec<_>>();
    let current = match states(&refused_definitions) {
        Ok(current) => current,
        Err(error) => {
            outcome.errors.push(error);
            return outcome;
        }
    };
    for (definition, reason) in refused {
        if current.get(definition.id) == Some(&TaskStatus::Enabled) {
            let reason = helper_error.as_deref().unwrap_or(&reason);
            outcome.errors.push(format!(
                "Could not disable scheduled task {}: {reason}",
                definition.path
            ));
        } else {
            outcome.disabled += 1;
        }
    }
    outcome
}
