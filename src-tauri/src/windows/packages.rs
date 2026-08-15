use serde::Deserialize;

use crate::models::InstalledPackage;

use super::run_powershell_json;

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
    let packages: Vec<PowerShellPackage> = run_powershell_json(script)?;
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

fn powershell_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("'{}'", value.replace('\'', "''")))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn enforce(package_names: &[String]) -> Result<PackageEnforcementResult, String> {
    if package_names.is_empty() {
        return Ok(PackageEnforcementResult {
            removed: Vec::new(),
            errors: Vec::new(),
        });
    }

    let targets = powershell_array(package_names);
    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$targets = @({targets})
$removed = [System.Collections.Generic.List[string]]::new()
$errors = [System.Collections.Generic.List[string]]::new()
$installed = @(Get-AppxPackage -AllUsers)
$provisioned = @(Get-AppxProvisionedPackage -Online)
foreach ($target in $targets) {{
    foreach ($package in @($installed | Where-Object {{ $_.Name -eq $target }})) {{
        try {{
            Remove-AppxPackage -Package $package.PackageFullName -AllUsers -ErrorAction Stop
            $removed.Add($package.Name)
        }} catch {{
            $errors.Add("$($package.Name): $($_.Exception.Message)")
        }}
    }}
    foreach ($package in @($provisioned | Where-Object {{ $_.DisplayName -eq $target }})) {{
        try {{
            Remove-AppxProvisionedPackage -Online -PackageName $package.PackageName -AllUsers -ErrorAction Stop | Out-Null
            $removed.Add($package.DisplayName)
        }} catch {{
            $errors.Add("$($package.DisplayName): $($_.Exception.Message)")
        }}
    }}
}}
[PSCustomObject]@{{ Removed = @($removed); Errors = @($errors) }} | ConvertTo-Json -Compress
"#
    );

    run_powershell_json(&script)
}
