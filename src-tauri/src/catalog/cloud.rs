use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, SupportRequirement};

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        policy!(
            "onedrive",
            "OneDrive file sync",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
            "DisableFileSyncNGSC",
            1
        ),
        policy!(
            "cloud-clipboard",
            "Cloud clipboard",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "AllowCrossDeviceClipboard",
            0
        ),
        policy!(
            "clipboard-history",
            "Clipboard history",
            "Cloud & sync",
            Cu,
            r"SOFTWARE\Microsoft\Clipboard",
            "EnableClipboardHistory",
            1
        )
        .with_support(SupportRequirement::windows_build(17_763)),
        policy!(
            "cross-device",
            "Cross-device experiences",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "EnableCdp",
            0
        ),
        policy!(
            "activity-publishing",
            "Activity publishing",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "PublishUserActivities",
            0
        ),
        policy!(
            "activity-upload",
            "Activity upload",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "UploadUserActivities",
            0
        ),
        policy!(
            "activity-feed",
            "Activity feed",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "EnableActivityFeed",
            0
        ),
        policy!(
            "settings-sync",
            "Windows settings sync",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
            "DisableSettingSync",
            2
        ),
        policy!(
            "settings-sync-override",
            "Settings sync override",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
            "DisableSettingSyncUserOverride",
            1
        ),
        policy!(
            "delivery-optimization",
            "Update peer uploads",
            "Cloud & sync",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization",
            "DODownloadMode",
            0
        )
        .with_choices(choices::UPDATE_PEERS),
    ]
}
