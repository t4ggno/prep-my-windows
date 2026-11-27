use std::collections::BTreeMap;

use crate::models::{
    AppConfig, DEFAULT_ACTIVE_HOURS_END, DEFAULT_ACTIVE_HOURS_START, PolicyDefinition,
    ScheduledTaskDefinition,
};

macro_rules! policy {
    ($id:expr, $name:expr, $category:expr, $hive:expr, $path:expr, $value_name:expr, $value:expr) => {
        $crate::models::PolicyDefinition {
            id: $id,
            name: $name,
            category: $category,
            hive: $hive,
            path: $path,
            value_name: $value_name,
            desired: $crate::models::RegistryValue::Dword($value),
            choices: &[],
            editor: $crate::models::ValueEditor::Choice,
            support: $crate::models::SupportRequirement::any(),
            default_enabled: true,
        }
    };
}

macro_rules! text_policy {
    ($id:expr, $name:expr, $category:expr, $hive:expr, $path:expr, $value_name:expr, $value:expr) => {
        $crate::models::PolicyDefinition {
            id: $id,
            name: $name,
            category: $category,
            hive: $hive,
            path: $path,
            value_name: $value_name,
            desired: $crate::models::RegistryValue::Text($value),
            choices: &[],
            editor: $crate::models::ValueEditor::Choice,
            support: $crate::models::SupportRequirement::any(),
            default_enabled: true,
        }
    };
}

macro_rules! absent_policy {
    ($id:expr, $name:expr, $category:expr, $hive:expr, $path:expr, $value_name:expr) => {
        $crate::models::PolicyDefinition {
            id: $id,
            name: $name,
            category: $category,
            hive: $hive,
            path: $path,
            value_name: $value_name,
            desired: $crate::models::RegistryValue::Absent,
            choices: &[],
            editor: $crate::models::ValueEditor::Choice,
            support: $crate::models::SupportRequirement::any(),
            default_enabled: true,
        }
    };
}

mod ai;
mod apps;
mod choices;
mod cloud;
mod diagnostics;
mod edge;
mod gaming;
mod network;
mod personalization;
mod privacy;
mod software;
mod system;
mod tasks;
#[cfg(test)]
mod tests;
mod update;
mod usability;

pub use apps::{default_process_rules, packages};
pub use network::network_blocks;
pub use software::software;
pub use system::system_settings;

pub fn scheduled_tasks() -> Vec<ScheduledTaskDefinition> {
    tasks::windows_tasks()
        .into_iter()
        .chain(software().into_iter().flat_map(|item| item.tasks))
        .collect()
}

pub fn policies() -> Vec<PolicyDefinition> {
    policies_with_active_hours(DEFAULT_ACTIVE_HOURS_START, DEFAULT_ACTIVE_HOURS_END)
}

pub fn configured_policies(config: &AppConfig) -> Vec<PolicyDefinition> {
    policies_with_active_hours(config.active_hours_start, config.active_hours_end)
        .into_iter()
        .map(|mut definition| {
            if let Some(choice) = config
                .policy_choices
                .get(definition.id)
                .and_then(|choice| definition.choice(choice))
            {
                definition.desired = choice.value.clone();
            }
            definition
        })
        .collect()
}

fn policies_with_active_hours(
    active_hours_start: u8,
    active_hours_end: u8,
) -> Vec<PolicyDefinition> {
    [
        diagnostics::policies(),
        system::policies(),
        usability::policies(),
        gaming::policies(),
        personalization::policies(),
        ai::policies(),
        cloud::policies(),
        update::policies(active_hours_start, active_hours_end),
        privacy::policies(),
        edge::policies(),
        software()
            .into_iter()
            .flat_map(|item| item.policies)
            .collect(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enforcement_enabled: false,
            policies: policies()
                .into_iter()
                .map(|item| (item.id.to_owned(), item.default_enabled))
                .chain(
                    system_settings()
                        .into_iter()
                        .map(|item| (item.id.to_owned(), true)),
                )
                .collect::<BTreeMap<_, _>>(),
            policy_choices: BTreeMap::new(),
            packages: packages()
                .into_iter()
                .map(|item| (item.id.to_owned(), true))
                .collect::<BTreeMap<_, _>>(),
            network_blocks: network_blocks()
                .into_iter()
                .map(|item| (item.id.to_owned(), true))
                .collect::<BTreeMap<_, _>>(),
            scheduled_tasks: scheduled_tasks()
                .into_iter()
                .map(|item| (item.id.to_owned(), item.default_enabled))
                .collect::<BTreeMap<_, _>>(),
            process_rules: default_process_rules(),
            muted_process_notifications: Default::default(),
            custom_packages: Vec::new(),
            blocked_autostarts: Vec::new(),
            start_with_windows: true,
            active_hours_start: DEFAULT_ACTIVE_HOURS_START,
            active_hours_end: DEFAULT_ACTIVE_HOURS_END,
        }
    }
}
