use std::time::Duration;

use serde::Deserialize;

use crate::models::InstalledPackage;

use super::{DEFAULT_TIMEOUT, powershell_string_array, run_powershell_json};

const PACKAGE_REMOVAL_TIMEOUT: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PowerShellPackage {
    name: String,
    full_name: String,
    publisher: String,
    version: String,
    provisioned: bool,
    removable: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PackageEnforcementResult {
    pub removed: Vec<String>,
    pub errors: Vec<String>,
}

pub fn list() -> Result<Vec<InstalledPackage>, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$provisioned = @(Get-AppxProvisionedPackage -Online)
$packages = @(Get-AppxPackage -AllUsers | Where-Object { -not $_.IsFramework } | Group-Object Name | ForEach-Object { $_.Group | Select-Object -First 1 })
$items = [System.Collections.Generic.List[object]]::new()
foreach ($package in $packages) {
    $items.Add([PSCustomObject]@{
        Name = $package.Name
        FullName = $package.PackageFullName
        Publisher = $package.Publisher
        Version = [string]$package.Version
        Provisioned = @($provisioned | Where-Object { $_.DisplayName -eq $package.Name }).Count -gt 0
        Removable = -not $package.NonRemovable
    })
}
foreach ($package in @($provisioned | Where-Object { $_.DisplayName -notin $packages.Name })) {
    $items.Add([PSCustomObject]@{
        Name = $package.DisplayName
        FullName = $package.PackageName
        Publisher = [string]$package.PublisherId
        Version = [string]$package.Version
        Provisioned = $true
        Removable = $true
    })
}
ConvertTo-Json -InputObject @($items) -Compress
"#;
    let packages: Vec<PowerShellPackage> = run_powershell_json(script, DEFAULT_TIMEOUT)?;
    Ok(packages
        .into_iter()
        .map(|package| InstalledPackage {
            name: package.name,
            full_name: package.full_name,
            publisher: package.publisher,
            version: package.version,
            provisioned: package.provisioned,
            removable: package.removable,
        })
        .collect())
}

pub fn enforce(package_names: &[String]) -> Result<PackageEnforcementResult, String> {
    if package_names.is_empty() {
        return Ok(PackageEnforcementResult {
            removed: Vec::new(),
            errors: Vec::new(),
        });
    }

    let targets = powershell_string_array(package_names);
    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$targets = @({targets})
$removed = [System.Collections.Generic.List[string]]::new()
$errors = [System.Collections.Generic.List[string]]::new()
$installed = @(Get-AppxPackage -AllUsers)
$provisioned = @(Get-AppxProvisionedPackage -Online)
foreach ($target in $targets) {{
    foreach ($package in @($installed | Where-Object {{ $_.Name -eq $target -and -not $_.NonRemovable }})) {{
        try {{
            Remove-AppxPackage -Package $package.PackageFullName -AllUsers -ErrorAction Stop
            $removed.Add($package.Name)
        }} catch {{
            $message = $_.Exception.Message
            if (@(Get-AppxPackage -AllUsers -Name $package.Name | Where-Object {{ $_.PackageFullName -eq $package.PackageFullName }}).Count -eq 0) {{
                $removed.Add($package.Name)
            }} else {{
                $errors.Add("$($package.Name): $message")
            }}
        }}
    }}
    foreach ($package in @($provisioned | Where-Object {{ $_.DisplayName -eq $target }})) {{
        try {{
            Remove-AppxProvisionedPackage -Online -PackageName $package.PackageName -AllUsers -ErrorAction Stop | Out-Null
            $removed.Add($package.DisplayName)
        }} catch {{
            $message = $_.Exception.Message
            if (@(Get-AppxProvisionedPackage -Online | Where-Object {{ $_.PackageName -eq $package.PackageName }}).Count -eq 0) {{
                $removed.Add($package.DisplayName)
            }} else {{
                $errors.Add("$($package.DisplayName): $message")
            }}
        }}
    }}
}}
[PSCustomObject]@{{ Removed = @($removed); Errors = @($errors) }} | ConvertTo-Json -Compress
"#
    );

    run_powershell_json(&script, PACKAGE_REMOVAL_TIMEOUT)
}
