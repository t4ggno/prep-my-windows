use super::run_powershell;

const TASK_NAME: &str = "Prep My Windows";

fn change_from_output(output: &str) -> Result<bool, String> {
    match output {
        "changed" => Ok(true),
        "unchanged" => Ok(false),
        value => Err(format!("Unexpected startup task state: {value}")),
    }
}

pub fn configure(enabled: bool) -> Result<bool, String> {
    if !enabled {
        let output = run_powershell(&format!(
            r#"
$ErrorActionPreference = 'Stop'
$task = Get-ScheduledTask -TaskPath '\' | Where-Object {{ $_.TaskName -ceq '{TASK_NAME}' }} | Select-Object -First 1
if ($null -eq $task) {{
    'unchanged'
}} else {{
    Unregister-ScheduledTask -InputObject $task -Confirm:$false -ErrorAction Stop
    'changed'
}}
"#
        ))?;
        return change_from_output(&output);
    }

    let executable = std::env::current_exe()
        .map_err(|error| format!("Could not resolve application path: {error}"))?;
    let working_directory = executable
        .parent()
        .ok_or_else(|| "Could not resolve application directory".to_owned())?;
    let executable = executable.display().to_string().replace('\'', "''");
    let working_directory = working_directory.display().to_string().replace('\'', "''");
    let output = run_powershell(&format!(
        r#"
$ErrorActionPreference = 'Stop'
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$task = Get-ScheduledTask -TaskPath '\' | Where-Object {{ $_.TaskName -ceq '{TASK_NAME}' }} | Select-Object -First 1
$needsRegistration = $null -eq $task
if (-not $needsRegistration) {{
    $actions = @($task.Actions)
    $triggers = @($task.Triggers)
    $principalSid = try {{ ([Security.Principal.NTAccount]::new($task.Principal.UserId)).Translate([Security.Principal.SecurityIdentifier]).Value }} catch {{ '' }}
    $triggerSid = try {{ ([Security.Principal.NTAccount]::new($triggers[0].UserId)).Translate([Security.Principal.SecurityIdentifier]).Value }} catch {{ '' }}
    $needsRegistration =
        $actions.Count -ne 1 -or
        $actions[0].Execute -ine '{executable}' -or
        $actions[0].Arguments -cne '--background' -or
        $actions[0].WorkingDirectory -ine '{working_directory}' -or
        $triggers.Count -ne 1 -or
        $triggers[0].CimClass.CimClassName -ne 'MSFT_TaskLogonTrigger' -or
        -not $triggers[0].Enabled -or
        $triggerSid -ne $identity.User.Value -or
        $principalSid -ne $identity.User.Value -or
        [string]$task.Principal.LogonType -ne 'Interactive' -or
        [string]$task.Principal.RunLevel -ne 'Highest' -or
        [string]$task.Settings.MultipleInstances -ne 'IgnoreNew' -or
        $task.Settings.DisallowStartIfOnBatteries -or
        $task.Settings.StopIfGoingOnBatteries -or
        -not $task.Settings.StartWhenAvailable -or
        $task.Settings.ExecutionTimeLimit -ne 'PT0S'
}}
if (-not $needsRegistration) {{
    'unchanged'
    exit 0
}}
$action = New-ScheduledTaskAction -Execute '{executable}' -Argument '--background' -WorkingDirectory '{working_directory}'
$trigger = New-ScheduledTaskTrigger -AtLogOn -User $identity.Name
$principal = New-ScheduledTaskPrincipal -UserId $identity.Name -LogonType Interactive -RunLevel Highest
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit ([TimeSpan]::Zero) -MultipleInstances IgnoreNew -StartWhenAvailable
Register-ScheduledTask -TaskName '{TASK_NAME}' -TaskPath '\' -Action $action -Trigger $trigger -Principal $principal -Settings $settings -Force | Out-Null
'changed'
"#
    ))?;
    change_from_output(&output)
}

#[cfg(test)]
mod tests {
    use super::change_from_output;

    #[test]
    fn parses_startup_task_change_state() {
        assert_eq!(change_from_output("changed"), Ok(true));
        assert_eq!(change_from_output("unchanged"), Ok(false));
        assert!(change_from_output("unknown").is_err());
    }
}
