use super::choices;
use crate::models::{
    PolicyDefinition, RegistryHive, SupportRequirement, SystemSettingDefinition, SystemSettingKind,
};

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        policy!(
            "silent-admin-elevation",
            "Administrator elevation prompts",
            "Windows behavior",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "ConsentPromptBehaviorAdmin",
            0
        )
        .with_choices(choices::ADMIN_ELEVATION),
        policy!(
            "file-extensions",
            "File name extensions",
            "Windows behavior",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "HideFileExt",
            0
        ),
        policy!(
            "taskbar-end-task",
            "Taskbar End task",
            "Windows behavior",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            "TaskbarEndTask",
            1
        )
        .with_support(SupportRequirement::windows_build(22_631)),
        policy!(
            "detailed-crash-screen",
            "Detailed crash screen",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\CrashControl",
            "DisplayParameters",
            1
        ),
        policy!(
            "firmware-installed-software",
            "Firmware-installed software",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\Session Manager",
            "DisableWpbtExecution",
            1
        ),
        policy!(
            "user-choice-protection",
            "User Choice Protection",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\UCPD",
            "Start",
            4
        )
        .with_choices(choices::DRIVER_DISABLED),
        policy!(
            "maps-service",
            "Downloaded maps service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\MapsBroker",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_DISABLED),
        policy!(
            "retail-demo-service",
            "Retail demo service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\RetailDemo",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_DISABLED),
        policy!(
            "fast-startup",
            "Fast startup",
            "Power",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
            "HiberbootEnabled",
            0
        ),
        policy!(
            "game-dvr",
            "Game DVR",
            "Gaming",
            Cu,
            r"System\GameConfigStore",
            "GameDVR_Enabled",
            0
        ),
        policy!(
            "game-capture",
            "Background game capture",
            "Gaming",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
            "AppCaptureEnabled",
            0
        ),
        policy!(
            "game-recording-policy",
            "Game recording policy",
            "Gaming",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\GameDVR",
            "AllowGameDVR",
            0
        ),
        policy!(
            "game-bar-controller",
            "Game Bar controller button",
            "Gaming",
            Cu,
            r"SOFTWARE\Microsoft\GameBar",
            "UseNexusForGameBarEnabled",
            0
        ),
    ]
}

pub fn system_settings() -> Vec<SystemSettingDefinition> {
    vec![
        SystemSettingDefinition {
            id: "hibernation",
            name: "Hibernation",
            category: "Power",
            scope: "All users",
            kind: SystemSettingKind::Hibernation,
            support: SupportRequirement::any(),
        },
        SystemSettingDefinition {
            id: "sticky-keys-shortcut",
            name: "StickyKeys activation shortcut",
            category: "Accessibility",
            scope: "Current user",
            kind: SystemSettingKind::StickyKeysShortcut,
            support: SupportRequirement::any(),
        },
        SystemSettingDefinition {
            id: "filter-keys-shortcut",
            name: "FilterKeys activation shortcut",
            category: "Accessibility",
            scope: "Current user",
            kind: SystemSettingKind::FilterKeysShortcut,
            support: SupportRequirement::any(),
        },
        SystemSettingDefinition {
            id: "toggle-keys-shortcut",
            name: "ToggleKeys activation shortcut",
            category: "Accessibility",
            scope: "Current user",
            kind: SystemSettingKind::ToggleKeysShortcut,
            support: SupportRequirement::any(),
        },
        SystemSettingDefinition {
            id: "widgets-taskbar-button",
            name: "Widgets taskbar button",
            category: "Windows behavior",
            scope: "Current user",
            kind: SystemSettingKind::WidgetsTaskbarButton,
            support: SupportRequirement::windows_build(22_000).blocked_by_user_choice_protection(),
        },
    ]
}
