use std::collections::HashMap;
use std::thread;

use crate::catalog;
use crate::models::{
    AppConfig, InstalledPackage, NetworkView, PackageView, PolicyChoiceView, PolicyView,
    ScheduledTaskView, Snapshot, SupportRequirement, SystemInfo, SystemSupport, TaskStatus,
    ValueEditor,
};
use crate::state::AppState;
use crate::windows::{self, hosts, packages, registry, services, system_settings, tasks};

struct PolicyViews {
    views: Vec<PolicyView>,
    problems: Vec<String>,
}

fn unavailable_reason(
    requirement: SupportRequirement,
    support: &Result<SystemSupport, String>,
) -> Option<String> {
    match support {
        Ok(system) => requirement.unavailable_reason(system),
        Err(error) => (requirement != SupportRequirement::any()).then(|| error.clone()),
    }
}

fn policy_views(config: &AppConfig) -> PolicyViews {
    let support = windows::support_info();
    let mut problems = Vec::new();
    if let Err(error) = &support {
        problems.push(error.clone());
    }
    let enabled = |id: &str| config.policies.get(id).copied().unwrap_or(false);
    let recommendations = catalog::policies()
        .into_iter()
        .map(|definition| (definition.id, definition.desired))
        .collect::<HashMap<_, _>>();

    let mut views = Vec::new();
    for definition in catalog::configured_policies(config) {
        let recommended = &recommendations[definition.id];
        let base = PolicyView {
            id: definition.id.to_owned(),
            name: definition.name.to_owned(),
            category: definition.category.to_owned(),
            scope: definition.hive.label().to_owned(),
            enabled: enabled(definition.id),
            compliant: false,
            current: "Unavailable".to_owned(),
            wanted: definition.wanted_label(),
            choices: definition
                .choices
                .iter()
                .map(|choice| PolicyChoiceView {
                    id: choice.id.to_owned(),
                    label: choice.label.to_owned(),
                    value: choice.value.label(),
                })
                .collect(),
            choice: definition
                .selected_choice()
                .map(|choice| choice.id.to_owned()),
            editor: definition.editor,
            recommended: definition.label_for(recommended),
            recommended_enabled: definition.default_enabled,
            registry: Some(definition.registry_location()),
            available: false,
            unavailable_reason: None,
        };
        let reason = unavailable_reason(definition.support, &support).or_else(|| {
            definition
                .service_name()
                .filter(|name| !services::is_installed(name))
                .map(|_| "Not installed on this PC".to_owned())
        });
        if let Some(reason) = reason {
            views.push(PolicyView {
                unavailable_reason: Some(reason),
                ..base
            });
            continue;
        }
        let view = match registry::read_policy(&definition) {
            Ok(current) => PolicyView {
                compliant: definition.desired.is_satisfied_by(current.as_deref()),
                current: definition.value_label(current.as_deref()),
                available: true,
                ..base
            },
            Err(error) => {
                problems.push(error);
                PolicyView {
                    current: "Unreadable".to_owned(),
                    available: true,
                    ..base
                }
            }
        };
        views.push(view);
    }

    for definition in catalog::system_settings() {
        let base = PolicyView {
            id: definition.id.to_owned(),
            name: definition.name.to_owned(),
            category: definition.category.to_owned(),
            scope: definition.scope.to_owned(),
            enabled: enabled(definition.id),
            compliant: false,
            current: "Unavailable".to_owned(),
            wanted: "Unavailable".to_owned(),
            choices: Vec::new(),
            choice: None,
            editor: ValueEditor::Choice,
            recommended: "Unavailable".to_owned(),
            recommended_enabled: true,
            registry: None,
            available: false,
            unavailable_reason: None,
        };
        if let Some(reason) = unavailable_reason(definition.support, &support) {
            views.push(PolicyView {
                unavailable_reason: Some(reason),
                ..base
            });
            continue;
        }
        let view = match system_settings::read(definition.kind) {
            Ok(current) => PolicyView {
                compliant: current.compliant,
                current: current.current,
                recommended: current.wanted.clone(),
                wanted: current.wanted,
                available: current.available,
                unavailable_reason: current.unavailable_reason,
                ..base
            },
            Err(error) => {
                problems.push(error);
                PolicyView {
                    current: "Unreadable".to_owned(),
                    available: true,
                    ..base
                }
            }
        };
        views.push(view);
    }

    PolicyViews { views, problems }
}

fn package_views(config: &AppConfig, installed_packages: &[InstalledPackage]) -> Vec<PackageView> {
    catalog::packages()
        .into_iter()
        .map(|definition| {
            let matches = installed_packages
                .iter()
                .filter(|package| package.name.eq_ignore_ascii_case(definition.pattern));
            let mut installed = false;
            let mut provisioned = false;
            let mut removable = true;
            for package in matches {
                installed = true;
                provisioned |= package.provisioned;
                removable &= package.removable;
            }
            PackageView {
                id: definition.id.to_owned(),
                name: definition.name.to_owned(),
                package_name: definition.pattern.to_owned(),
                enabled: config.packages.get(definition.id).copied().unwrap_or(false),
                installed,
                provisioned,
                removable,
            }
        })
        .collect()
}

fn network_views(config: &AppConfig, problems: &mut Vec<String>) -> Vec<NetworkView> {
    let definitions = catalog::network_blocks();
    let blocked = hosts::statuses(&definitions).unwrap_or_else(|error| {
        problems.push(error);
        vec![false; definitions.len()]
    });
    definitions
        .into_iter()
        .zip(blocked)
        .map(|(definition, blocked)| NetworkView {
            id: definition.id.to_owned(),
            domain: definition.domain.to_owned(),
            enabled: config
                .network_blocks
                .get(definition.id)
                .copied()
                .unwrap_or(false),
            blocked,
        })
        .collect()
}

fn task_views(
    config: &AppConfig,
    states: Result<HashMap<String, TaskStatus>, String>,
    problems: &mut Vec<String>,
) -> Vec<ScheduledTaskView> {
    let states = states.unwrap_or_else(|error| {
        problems.push(error);
        HashMap::new()
    });
    catalog::scheduled_tasks()
        .into_iter()
        .map(|definition| ScheduledTaskView {
            id: definition.id.to_owned(),
            name: definition.name.to_owned(),
            path: definition.path.to_owned(),
            enabled: config
                .scheduled_tasks
                .get(definition.id)
                .copied()
                .unwrap_or(false),
            status: states
                .get(definition.id)
                .copied()
                .unwrap_or(TaskStatus::Unknown),
        })
        .collect()
}

pub fn build(state: &AppState, system: SystemInfo) -> Snapshot {
    let config = state.config();
    let (policies, installed_packages, task_states) = thread::scope(|scope| {
        let installed_packages = scope.spawn(packages::list);
        let task_states = scope.spawn(|| tasks::states(&catalog::scheduled_tasks()));
        let policies = policy_views(&config);
        (
            policies,
            installed_packages
                .join()
                .unwrap_or_else(|_| Err("Could not list installed apps".to_owned())),
            task_states
                .join()
                .unwrap_or_else(|_| Err("Could not read scheduled tasks".to_owned())),
        )
    });

    let mut problems = policies.problems;
    let installed_packages = match installed_packages {
        Ok(packages) => {
            state.remember_installed_packages(packages.clone());
            packages
        }
        Err(error) => {
            problems.push(format!("Could not list installed apps: {error}"));
            state.installed_packages()
        }
    };
    let network_blocks = network_views(&config, &mut problems);
    let scheduled_tasks = task_views(&config, task_states, &mut problems);
    for problem in problems {
        state.report_problem(problem);
    }

    Snapshot {
        system,
        status: state.status(),
        packages: package_views(&config, &installed_packages),
        settings: config,
        policies: policies.views,
        installed_packages,
        network_blocks,
        scheduled_tasks,
        activity: state.activity(),
        reapplied: state.reapplied(),
    }
}
