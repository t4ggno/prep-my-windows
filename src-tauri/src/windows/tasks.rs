use std::collections::HashMap;

use serde::Deserialize;

use crate::models::{ScheduledTaskDefinition, TaskStatus};

use super::{DEFAULT_TIMEOUT, run_hidden, run_powershell_json};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TaskState {
    path: String,
    state: String,
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

pub fn disable(path: &str) -> Result<bool, String> {
    let output = run_hidden(
        "schtasks.exe",
        &["/Change", "/TN", path, "/Disable"],
        DEFAULT_TIMEOUT,
    )?;
    if output.status.success() {
        return Ok(true);
    }

    let message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if message.contains("cannot find") || message.contains("does not exist") {
        return Ok(false);
    }
    Err(if message.is_empty() {
        format!("Could not disable scheduled task {path}")
    } else {
        message
    })
}
