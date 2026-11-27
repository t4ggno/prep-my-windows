use std::collections::HashSet;

use super::{
    configured_policies, network_blocks, packages, policies, scheduled_tasks, software,
    system_settings,
};
use crate::models::{AppConfig, RegistryValue, ValueEditor};

fn all_unique(values: impl IntoIterator<Item = String>) -> bool {
    let mut unique = HashSet::new();
    values.into_iter().all(|value| unique.insert(value))
}

#[test]
fn catalog_identifiers_are_unique() {
    assert!(all_unique(
        policies()
            .into_iter()
            .map(|item| item.id.to_owned())
            .chain(system_settings().into_iter().map(|item| item.id.to_owned()))
    ));
    assert!(all_unique(
        packages().into_iter().map(|item| item.id.to_owned())
    ));
    assert!(all_unique(
        network_blocks().into_iter().map(|item| item.id.to_owned())
    ));
    assert!(all_unique(
        scheduled_tasks().into_iter().map(|item| item.id.to_owned())
    ));
}

#[test]
fn catalog_targets_are_unique() {
    assert!(all_unique(policies().into_iter().map(|item| format!(
        "{:?}\\{}\\{}",
        item.hive,
        item.path.to_ascii_lowercase(),
        item.value_name.to_ascii_lowercase()
    ))));
    assert!(all_unique(
        packages()
            .into_iter()
            .map(|item| item.pattern.to_ascii_lowercase())
    ));
    assert!(all_unique(
        scheduled_tasks()
            .into_iter()
            .map(|item| item.path.to_ascii_lowercase())
    ));
}

#[test]
fn default_profile_follows_catalog_defaults() {
    let config = AppConfig::default();
    for definition in policies() {
        assert_eq!(
            config.policies.get(definition.id),
            Some(&definition.default_enabled),
            "{}",
            definition.id
        );
    }
    let software_items = software()
        .iter()
        .flat_map(|item| item.item_ids().collect::<Vec<_>>())
        .collect::<HashSet<_>>();
    let windows_policies = policies()
        .into_iter()
        .filter(|definition| !software_items.contains(definition.id))
        .collect::<Vec<_>>();
    assert!(
        windows_policies
            .iter()
            .filter(|definition| definition.default_enabled)
            .count()
            > windows_policies.len() * 3 / 4
    );
    assert!(
        system_settings()
            .iter()
            .all(|definition| config.policies[definition.id])
    );
    assert!(config.packages.values().all(|enabled| *enabled));
    assert!(config.network_blocks.values().all(|enabled| *enabled));
    for definition in scheduled_tasks() {
        assert_eq!(
            config.scheduled_tasks.get(definition.id),
            Some(&(definition.default_enabled && !software_items.contains(definition.id))),
            "{}",
            definition.id
        );
    }
    assert!(config.process_rules.iter().all(|rule| rule.built_in));
    for (id, enabled) in [
        ("built-in-onedrive", true),
        ("built-in-widgets", true),
        ("built-in-steam", false),
        ("built-in-discord", false),
        ("built-in-zoom", false),
    ] {
        assert_eq!(
            config
                .process_rules
                .iter()
                .find(|rule| rule.id == id)
                .map(|rule| rule.enabled),
            Some(enabled),
            "{id}"
        );
    }
}

#[test]
fn location_configuration_removes_policies_and_enables_consent() {
    let definitions = policies();
    for id in [
        "location",
        "location-scripting",
        "location-provider",
        "app-location",
    ] {
        let definition = definitions.iter().find(|item| item.id == id).unwrap();
        assert_eq!(definition.desired, RegistryValue::Absent);
        assert!(definition.path.contains(r"SOFTWARE\Policies\"));
    }

    for id in [
        "location-sensor",
        "location-service",
        "location-service-start",
        "location-consent",
        "location-user-consent",
        "location-desktop-consent",
    ] {
        let definition = definitions.iter().find(|item| item.id == id).unwrap();
        assert_ne!(definition.desired, RegistryValue::Absent);
        assert!(!definition.path.contains(r"SOFTWARE\Policies\"));
    }

    assert!(
        network_blocks()
            .iter()
            .all(|definition| definition.domain != "inference.location.live.net")
    );
}

#[test]
fn configured_policies_use_profile_active_hours() {
    let config = AppConfig {
        active_hours_start: 21,
        active_hours_end: 6,
        ..AppConfig::default()
    };

    let definitions = configured_policies(&config);

    assert_eq!(
        definitions
            .iter()
            .find(|item| item.id == "update-active-hours-start")
            .unwrap()
            .desired,
        RegistryValue::Dword(21)
    );
    assert_eq!(
        definitions
            .iter()
            .find(|item| item.id == "update-active-hours-end")
            .unwrap()
            .desired,
        RegistryValue::Dword(6)
    );
}

#[test]
fn quality_of_life_policies_use_current_windows_settings() {
    let definitions = policies();
    let expected = [
        (
            "file-extensions",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "HideFileExt",
            RegistryValue::Dword(0),
        ),
        (
            "start-iris-recommendations",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_IrisRecommendations",
            RegistryValue::Dword(0),
        ),
        (
            "start-recent-items",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_TrackDocs",
            RegistryValue::Dword(0),
        ),
        (
            "start-recent-apps",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Start",
            "ShowRecentList",
            RegistryValue::Dword(0),
        ),
        (
            "clipboard-history",
            r"SOFTWARE\Microsoft\Clipboard",
            "EnableClipboardHistory",
            RegistryValue::Dword(1),
        ),
        (
            "taskbar-end-task",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            "TaskbarEndTask",
            RegistryValue::Dword(1),
        ),
        (
            "edge-first-run",
            r"SOFTWARE\Policies\Microsoft\Edge",
            "HideFirstRunExperience",
            RegistryValue::Dword(1),
        ),
        (
            "edge-default-browser-campaign",
            r"SOFTWARE\Policies\Microsoft\Edge",
            "DefaultBrowserSettingsCampaignEnabled",
            RegistryValue::Dword(0),
        ),
        (
            "edge-pdf-handler-recommendations",
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ShowPDFDefaultRecommendationsEnabled",
            RegistryValue::Dword(0),
        ),
    ];

    for (id, path, value_name, desired) in expected {
        let definition = definitions.iter().find(|item| item.id == id).unwrap();
        assert_eq!(definition.path, path);
        assert_eq!(definition.value_name, value_name);
        assert_eq!(definition.desired, desired);
    }

    let recommended_section = definitions
        .iter()
        .find(|item| item.id == "start-recommended-section")
        .unwrap();
    assert_eq!(recommended_section.support.minimum_build, 22_621);

    let system_setting_ids = system_settings()
        .into_iter()
        .map(|item| item.id)
        .collect::<HashSet<_>>();
    assert_eq!(
        system_setting_ids,
        HashSet::from([
            "hibernation",
            "sticky-keys-shortcut",
            "filter-keys-shortcut",
            "toggle-keys-shortcut",
            "widgets-taskbar-button",
        ])
    );
}

#[test]
fn every_policy_has_a_value_editor() {
    for definition in policies() {
        match definition.editor {
            ValueEditor::Choice => assert!(
                !definition.choices.is_empty(),
                "{} has no choices",
                definition.id
            ),
            ValueEditor::ActiveHoursStart | ValueEditor::ActiveHoursEnd => {
                assert!(definition.choices.is_empty(), "{}", definition.id);
            }
        }
    }
}

#[test]
fn values_are_shown_with_their_meaning() {
    let definitions = policies();
    let find = |id: &str| definitions.iter().find(|item| item.id == id).unwrap();

    assert_eq!(find("file-extensions").value_label(Some("1")), "Hidden");
    assert_eq!(find("web-search").wanted_label(), "Off");
    assert_eq!(find("web-search").value_label(Some("0")), "On");
    assert_eq!(find("clipboard-history").wanted_label(), "On");
    assert_eq!(find("update-active-hours-end").wanted_label(), "07:00");
    assert_eq!(
        find("update-active-hours-start").value_label(Some("8")),
        "08:00"
    );
    assert_eq!(
        find("update-active-hours-start").value_label(None),
        "Not set"
    );
    assert_eq!(find("location").value_label(Some("1")), "Blocked");
}

#[test]
fn policy_choices_include_the_default_value() {
    for definition in policies().iter().filter(|item| !item.choices.is_empty()) {
        assert!(
            definition.selected_choice().is_some(),
            "{} has no choice for its default value",
            definition.id
        );
        assert!(all_unique(
            definition.choices.iter().map(|choice| choice.id.to_owned())
        ));
        assert!(all_unique(
            definition.choices.iter().map(|choice| choice.value.label())
        ));
    }
}

#[test]
fn configured_policies_use_selected_choices() {
    let mut config = AppConfig::default();
    config
        .policy_choices
        .insert("diagnostic-data".to_owned(), "required".to_owned());
    config
        .policy_choices
        .insert("location".to_owned(), "unknown".to_owned());

    let definitions = configured_policies(&config);
    let desired = |id: &str| {
        definitions
            .iter()
            .find(|item| item.id == id)
            .unwrap()
            .desired
            .clone()
    };

    assert_eq!(desired("diagnostic-data"), RegistryValue::Dword(1));
    assert_eq!(desired("location"), RegistryValue::Absent);
}

#[test]
fn software_fixes_start_disabled_and_are_listed_once() {
    let bundles = software();
    assert!(all_unique(bundles.iter().map(|item| item.id.to_owned())));
    assert!(all_unique(bundles.iter().flat_map(|item| {
        item.item_ids().map(str::to_owned).collect::<Vec<_>>()
    })));

    let config = AppConfig::default();
    for bundle in &bundles {
        assert!(!bundle.markers.is_empty(), "{}", bundle.id);
        assert!(
            !bundle.policies.is_empty() || !bundle.tasks.is_empty(),
            "{}",
            bundle.id
        );
        for definition in &bundle.policies {
            assert!(!definition.default_enabled, "{}", definition.id);
            assert_eq!(definition.category, bundle.name, "{}", definition.id);
            assert_eq!(config.policies.get(definition.id), Some(&false));
        }
        for task in &bundle.tasks {
            assert!(!task.default_enabled, "{}", task.id);
            assert_eq!(config.scheduled_tasks.get(task.id), Some(&false));
        }
    }

    let amd = bundles
        .iter()
        .find(|item| item.id == "amd-software")
        .unwrap();
    let external_events = amd
        .policies
        .iter()
        .find(|item| item.id == "amd-external-events-service")
        .unwrap();
    assert_eq!(
        external_events.service_name(),
        Some("AMD External Events Utility")
    );
    assert_eq!(external_events.desired, RegistryValue::Dword(4));
}
