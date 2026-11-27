use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::change_history::ReappliedChange;

pub const DEFAULT_ACTIVE_HOURS_START: u8 = 22;
pub const DEFAULT_ACTIVE_HOURS_END: u8 = 7;

fn default_active_hours_start() -> u8 {
    DEFAULT_ACTIVE_HOURS_START
}

fn default_active_hours_end() -> u8 {
    DEFAULT_ACTIVE_HOURS_END
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RegistryHive {
    CurrentUser,
    LocalMachine,
}

impl RegistryHive {
    pub fn label(self) -> &'static str {
        match self {
            Self::CurrentUser => "Current user",
            Self::LocalMachine => "All users",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistryValue {
    Dword(u32),
    Text(&'static str),
    Absent,
}

impl RegistryValue {
    pub fn label(&self) -> String {
        match self {
            Self::Dword(value) => value.to_string(),
            Self::Text("") => "Empty".to_owned(),
            Self::Text(value) => (*value).to_owned(),
            Self::Absent => "Not set".to_owned(),
        }
    }

    pub fn is_satisfied_by(&self, current: Option<&str>) -> bool {
        match self {
            Self::Dword(value) => current.and_then(|value| value.parse().ok()) == Some(*value),
            Self::Text(value) => current == Some(*value),
            Self::Absent => current.is_none(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PolicyChoice {
    pub id: &'static str,
    pub label: &'static str,
    pub value: RegistryValue,
}

#[derive(Clone, Debug)]
pub struct PolicyDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub hive: RegistryHive,
    pub path: &'static str,
    pub value_name: &'static str,
    pub desired: RegistryValue,
    pub choices: &'static [PolicyChoice],
    pub support: SupportRequirement,
    pub default_enabled: bool,
}

const SERVICES_PATH: &str = r"SYSTEM\CurrentControlSet\Services\";

impl PolicyDefinition {
    pub fn off_by_default(mut self) -> Self {
        self.default_enabled = false;
        self
    }

    pub fn service_name(&self) -> Option<&'static str> {
        if self.value_name != "Start" {
            return None;
        }
        self.path
            .strip_prefix(SERVICES_PATH)
            .filter(|name| !name.is_empty() && !name.contains('\\'))
    }

    pub fn with_support(mut self, support: SupportRequirement) -> Self {
        self.support = support;
        self
    }

    pub fn with_choices(mut self, choices: &'static [PolicyChoice]) -> Self {
        self.choices = choices;
        self
    }

    pub fn choice(&self, id: &str) -> Option<&'static PolicyChoice> {
        self.choices.iter().find(|choice| choice.id == id)
    }

    pub fn selected_choice(&self) -> Option<&'static PolicyChoice> {
        self.choices
            .iter()
            .find(|choice| choice.value == self.desired)
    }

    pub fn wanted_label(&self) -> String {
        self.selected_choice()
            .map_or_else(|| self.desired.label(), |choice| choice.label.to_owned())
    }

    pub fn value_label(&self, current: Option<&str>) -> String {
        self.choices
            .iter()
            .find(|choice| choice.value.is_satisfied_by(current))
            .map_or_else(
                || current.unwrap_or("Not set").to_owned(),
                |choice| choice.label.to_owned(),
            )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowsEditionRequirement {
    Any,
    Professional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportRequirement {
    pub minimum_build: u32,
    pub edition: WindowsEditionRequirement,
    pub minimum_edge_major: u32,
    pub blocked_by_user_choice_protection: bool,
}

impl SupportRequirement {
    pub const fn any() -> Self {
        Self {
            minimum_build: 0,
            edition: WindowsEditionRequirement::Any,
            minimum_edge_major: 0,
            blocked_by_user_choice_protection: false,
        }
    }

    pub const fn blocked_by_user_choice_protection(self) -> Self {
        Self {
            blocked_by_user_choice_protection: true,
            ..self
        }
    }

    pub const fn windows_build(minimum_build: u32) -> Self {
        Self {
            minimum_build,
            ..Self::any()
        }
    }

    pub const fn edge(minimum_edge_major: u32) -> Self {
        Self {
            minimum_edge_major,
            ..Self::any()
        }
    }

    pub const fn professional(minimum_build: u32) -> Self {
        Self {
            minimum_build,
            edition: WindowsEditionRequirement::Professional,
            ..Self::any()
        }
    }

    pub fn unavailable_reason(self, system: &SystemSupport) -> Option<String> {
        if system.build < self.minimum_build {
            return Some(format!(
                "Requires Windows build {} or newer",
                self.minimum_build
            ));
        }
        if self.edition == WindowsEditionRequirement::Professional
            && !system.supports_professional_policies()
        {
            return Some(
                "Requires Windows Pro, Enterprise, Education, or IoT Enterprise".to_owned(),
            );
        }
        if self.minimum_edge_major > 0
            && system
                .edge_major_version
                .is_none_or(|version| version < self.minimum_edge_major)
        {
            return Some(format!(
                "Requires Microsoft Edge {} or newer",
                self.minimum_edge_major
            ));
        }
        if self.blocked_by_user_choice_protection && system.user_choice_protection_active {
            return Some(
                "Requires User Choice Protection to be off and Windows to be restarted".to_owned(),
            );
        }
        None
    }
}

impl Default for SupportRequirement {
    fn default() -> Self {
        Self::any()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemSupport {
    pub build: u32,
    pub edition_id: String,
    pub edge_major_version: Option<u32>,
    pub user_choice_protection_active: bool,
}

impl SystemSupport {
    fn supports_professional_policies(&self) -> bool {
        let edition = self.edition_id.to_ascii_lowercase();
        ["professional", "enterprise", "education", "iotenterprise"]
            .iter()
            .any(|supported| edition.starts_with(supported))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemSettingKind {
    Hibernation,
    StickyKeysShortcut,
    FilterKeysShortcut,
    ToggleKeysShortcut,
    WidgetsTaskbarButton,
}

#[derive(Clone, Debug)]
pub struct SystemSettingDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub scope: &'static str,
    pub kind: SystemSettingKind,
    pub support: SupportRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemSettingState {
    pub available: bool,
    pub unavailable_reason: Option<String>,
    pub compliant: bool,
    pub current: String,
    pub wanted: String,
}

#[derive(Clone, Debug)]
pub struct PackageDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub pattern: &'static str,
}

#[derive(Clone, Debug)]
pub struct NetworkDefinition {
    pub id: &'static str,
    pub domain: &'static str,
}

#[derive(Clone, Debug)]
pub struct ScheduledTaskDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub path: &'static str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessRule {
    pub id: String,
    pub name: String,
    pub executable_name: String,
    pub executable_path: Option<String>,
    pub enabled: bool,
    pub built_in: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutostartRule {
    pub id: String,
    pub name: String,
    pub kind: AutostartKind,
    pub location: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AutostartKind {
    Registry,
    StartupFolder,
    ScheduledTask,
    Service,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub enforcement_enabled: bool,
    pub policies: BTreeMap<String, bool>,
    #[serde(default)]
    pub policy_choices: BTreeMap<String, String>,
    pub packages: BTreeMap<String, bool>,
    pub network_blocks: BTreeMap<String, bool>,
    pub scheduled_tasks: BTreeMap<String, bool>,
    pub process_rules: Vec<ProcessRule>,
    #[serde(default)]
    pub muted_process_notifications: BTreeSet<String>,
    pub custom_packages: Vec<String>,
    pub blocked_autostarts: Vec<AutostartRule>,
    pub start_with_windows: bool,
    #[serde(default = "default_active_hours_start")]
    pub active_hours_start: u8,
    #[serde(default = "default_active_hours_end")]
    pub active_hours_end: u8,
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), String> {
        validate_active_hours(self.active_hours_start, self.active_hours_end)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyChoiceView {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyView {
    pub id: String,
    pub name: String,
    pub category: String,
    pub scope: String,
    pub enabled: bool,
    pub compliant: bool,
    pub current: String,
    pub wanted: String,
    pub choices: Vec<PolicyChoiceView>,
    pub choice: Option<String>,
    pub available: bool,
    pub unavailable_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageView {
    pub id: String,
    pub name: String,
    pub package_name: String,
    pub enabled: bool,
    pub installed: bool,
    pub provisioned: bool,
    pub removable: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledPackage {
    pub name: String,
    pub full_name: String,
    pub publisher: String,
    pub version: String,
    pub provisioned: bool,
    pub removable: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkView {
    pub id: String,
    pub domain: String,
    pub enabled: bool,
    pub blocked: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledTaskView {
    pub id: String,
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub status: TaskStatus,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Enabled,
    Disabled,
    Missing,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    pub name: String,
    pub executable_path: String,
    pub instances: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutostartEntry {
    pub id: String,
    pub name: String,
    pub command: String,
    pub kind: AutostartKind,
    pub source: String,
    pub location: String,
    pub state: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvent {
    pub id: String,
    pub timestamp: String,
    pub kind: String,
    pub message: String,
    pub details: Vec<String>,
    pub success: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineStatus {
    pub busy: bool,
    pub last_enforced_at: Option<String>,
    pub reapplied_total: u64,
    pub killed_total: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub product_name: String,
    pub display_version: String,
    pub build_number: String,
    pub is_windows_11: bool,
    pub is_elevated: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub system: SystemInfo,
    pub status: EngineStatus,
    pub settings: AppConfig,
    pub policies: Vec<PolicyView>,
    pub packages: Vec<PackageView>,
    pub installed_packages: Vec<InstalledPackage>,
    pub network_blocks: Vec<NetworkView>,
    pub scheduled_tasks: Vec<ScheduledTaskView>,
    pub activity: Vec<ActivityEvent>,
    pub reapplied: Vec<ReappliedChange>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveState {
    pub status: EngineStatus,
    pub activity: Vec<ActivityEvent>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeSettings {
    pub start_with_windows: bool,
    pub active_hours_start: u8,
    pub active_hours_end: u8,
}

impl RuntimeSettings {
    pub fn validate(&self) -> Result<(), String> {
        validate_active_hours(self.active_hours_start, self.active_hours_end)
    }
}

fn validate_active_hours(active_hours_start: u8, active_hours_end: u8) -> Result<(), String> {
    if active_hours_start > 23 || active_hours_end > 23 {
        return Err("Active hours must be between 0 and 23".to_owned());
    }
    let duration = (u16::from(active_hours_end) + 24 - u16::from(active_hours_start)) % 24;
    if duration == 0 || duration > 18 {
        return Err("Active hours must span between 1 and 18 hours".to_owned());
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddProcessRuleRequest {
    pub name: String,
    pub executable_name: String,
    pub executable_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{AppConfig, SupportRequirement, SystemSupport};

    #[test]
    fn validates_active_hours_ranges_across_midnight() {
        let mut config = AppConfig {
            active_hours_start: 22,
            active_hours_end: 7,
            ..AppConfig::default()
        };
        assert!(config.validate().is_ok());

        config.active_hours_end = 22;
        assert_eq!(
            config.validate(),
            Err("Active hours must span between 1 and 18 hours".to_owned())
        );

        config.active_hours_end = 21;
        assert_eq!(
            config.validate(),
            Err("Active hours must span between 1 and 18 hours".to_owned())
        );
    }

    #[test]
    fn reports_build_edition_and_edge_requirements() {
        let home = SystemSupport {
            build: 26_200,
            edition_id: "Core".to_owned(),
            edge_major_version: Some(151),
            user_choice_protection_active: true,
        };
        assert_eq!(
            SupportRequirement::professional(22_621).unavailable_reason(&home),
            Some("Requires Windows Pro, Enterprise, Education, or IoT Enterprise".to_owned())
        );
        assert_eq!(
            SupportRequirement::windows_build(30_000).unavailable_reason(&home),
            Some("Requires Windows build 30000 or newer".to_owned())
        );
        assert_eq!(
            SupportRequirement::any()
                .blocked_by_user_choice_protection()
                .unavailable_reason(&home),
            Some(
                "Requires User Choice Protection to be off and Windows to be restarted".to_owned()
            )
        );
        assert_eq!(
            SupportRequirement::any()
                .blocked_by_user_choice_protection()
                .unavailable_reason(&SystemSupport {
                    user_choice_protection_active: false,
                    ..home.clone()
                }),
            None
        );

        let without_edge = SystemSupport {
            edge_major_version: None,
            ..home
        };
        assert_eq!(
            SupportRequirement::edge(113).unavailable_reason(&without_edge),
            Some("Requires Microsoft Edge 113 or newer".to_owned())
        );
    }
}
