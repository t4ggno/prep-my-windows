use super::choices;
use crate::models::{
    PolicyDefinition, RegistryHive, SupportRequirement, SystemSettingDefinition, SystemSettingKind,
};

const STANDBY_NETWORK: &str =
    r"SOFTWARE\Policies\Microsoft\Power\PowerSettings\f15576e8-98b7-4186-b944-eafa664402d9";

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
        )
        .with_choices(choices::SHOWN_WHEN_ZERO),
        policy!(
            "taskbar-end-task",
            "Taskbar End task",
            "Windows behavior",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            "TaskbarEndTask",
            1
        )
        .with_choices(choices::ON_WHEN_ONE)
        .with_support(SupportRequirement::windows_build(22_631)),
        policy!(
            "detailed-crash-screen",
            "Detailed crash screen",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\CrashControl",
            "DisplayParameters",
            1
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "firmware-installed-software",
            "Firmware-installed software",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\Session Manager",
            "DisableWpbtExecution",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "user-choice-protection",
            "User Choice Protection",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\UCPD",
            "Start",
            4
        )
        .with_choices(choices::DRIVER_START),
        policy!(
            "maps-service",
            "Downloaded maps service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\MapsBroker",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "retail-demo-service",
            "Retail demo service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\RetailDemo",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "fast-startup",
            "Fast startup",
            "Power",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
            "HiberbootEnabled",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "remote-registry-service",
            "Remote Registry service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\RemoteRegistry",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "fax-service",
            "Fax service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\Fax",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "media-sharing-service",
            "Windows Media Player sharing service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\WMPNetworkSvc",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "wallet-service",
            "Wallet service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\WalletService",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "parental-controls-service",
            "Parental controls service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\WpcMonSvc",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "link-tracking-service",
            "Distributed Link Tracking service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\TrkWks",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START),
        policy!(
            "sysmain-service",
            "SysMain service",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\SysMain",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_START)
        .off_by_default(),
        policy!(
            "long-paths",
            "Long file paths",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "LongPathsEnabled",
            1
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "autorun",
            "AutoRun on all drives",
            "Windows behavior",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoDriveTypeAutoRun",
            255
        )
        .with_choices(choices::AUTORUN),
        policy!(
            "remote-assistance",
            "Remote Assistance",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\Remote Assistance",
            "fAllowToGetHelp",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "automatic-device-encryption",
            "Automatic device encryption",
            "Windows behavior",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\BitLocker",
            "PreventDeviceEncryption",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE)
        .off_by_default(),
        policy!(
            "standby-network-plugged-in",
            "Network in standby when plugged in",
            "Power",
            Lm,
            STANDBY_NETWORK,
            "ACSettingIndex",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "standby-network-on-battery",
            "Network in standby on battery",
            "Power",
            Lm,
            STANDBY_NETWORK,
            "DCSettingIndex",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "power-throttling",
            "Power throttling",
            "Power",
            Lm,
            r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
            "PowerThrottlingOff",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE)
        .off_by_default(),
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
