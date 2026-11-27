use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, SupportRequirement};

const CATEGORY: &str = "Usability";
const EXPLORER: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer";
const EXPLORER_ADVANCED: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced";
const EXPLORER_STARTUP: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize";

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        text_policy!(
            "classic-context-menu",
            "Full context menu",
            CATEGORY,
            Cu,
            r"SOFTWARE\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32",
            "",
            ""
        )
        .with_choices(choices::FULL_CONTEXT_MENU)
        .with_support(SupportRequirement::windows_build(22_000)),
        policy!(
            "startup-app-delay",
            "Startup app delay",
            CATEGORY,
            Cu,
            EXPLORER_STARTUP,
            "StartupDelayInMSec",
            0
        )
        .with_choices(choices::STARTUP_DELAY),
        policy!(
            "startup-app-idle-wait",
            "Startup apps wait for idle",
            CATEGORY,
            Cu,
            EXPLORER_STARTUP,
            "WaitForIdleState",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "autoplay",
            "AutoPlay",
            CATEGORY,
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers",
            "DisableAutoplay",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "managed-default-printer",
            "Windows-managed default printer",
            CATEGORY,
            Cu,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows",
            "LegacyDefaultPrinterMode",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "explorer-cloud-files",
            "Cloud files in File Explorer Home",
            CATEGORY,
            Cu,
            EXPLORER,
            "ShowCloudFilesInQuickAccess",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "explorer-recent-files",
            "Recent files in File Explorer",
            CATEGORY,
            Cu,
            EXPLORER,
            "ShowRecent",
            0
        )
        .with_choices(choices::ON_WHEN_ONE)
        .off_by_default(),
        policy!(
            "explorer-frequent-folders",
            "Frequent folders in File Explorer",
            CATEGORY,
            Cu,
            EXPLORER,
            "ShowFrequent",
            0
        )
        .with_choices(choices::ON_WHEN_ONE)
        .off_by_default(),
        policy!(
            "explorer-start-page",
            "File Explorer start page",
            CATEGORY,
            Cu,
            EXPLORER_ADVANCED,
            "LaunchTo",
            1
        )
        .with_choices(choices::EXPLORER_START_PAGE)
        .off_by_default(),
        policy!(
            "hidden-files",
            "Hidden files",
            CATEGORY,
            Cu,
            EXPLORER_ADVANCED,
            "Hidden",
            1
        )
        .with_choices(choices::HIDDEN_FILES)
        .off_by_default(),
        policy!(
            "task-view-button",
            "Task View taskbar button",
            CATEGORY,
            Cu,
            EXPLORER_ADVANCED,
            "ShowTaskViewButton",
            0
        )
        .with_choices(choices::ON_WHEN_ONE)
        .off_by_default(),
        policy!(
            "taskbar-alignment",
            "Taskbar alignment",
            CATEGORY,
            Cu,
            EXPLORER_ADVANCED,
            "TaskbarAl",
            0
        )
        .with_choices(choices::TASKBAR_ALIGNMENT)
        .with_support(SupportRequirement::windows_build(22_000))
        .off_by_default(),
        policy!(
            "snap-assist",
            "Snap Assist suggestions",
            CATEGORY,
            Cu,
            EXPLORER_ADVANCED,
            "SnapAssist",
            0
        )
        .with_choices(choices::ON_WHEN_ONE)
        .off_by_default(),
        policy!(
            "taskbar-last-active-click",
            "Taskbar opens last active window",
            CATEGORY,
            Cu,
            EXPLORER_ADVANCED,
            "LastActiveClick",
            1
        )
        .with_choices(choices::ON_WHEN_ONE)
        .off_by_default(),
        policy!(
            "verbose-status",
            "Detailed startup and shutdown messages",
            CATEGORY,
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "VerboseStatus",
            1
        )
        .with_choices(choices::ON_WHEN_ONE)
        .off_by_default(),
    ]
}
