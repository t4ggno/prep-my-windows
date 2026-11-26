use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, SupportRequirement};

pub(super) fn policies(active_hours_start: u8, active_hours_end: u8) -> Vec<PolicyDefinition> {
    use RegistryHive::LocalMachine as Lm;

    vec![
        policy!(
            "update-active-hours-mode",
            "Automatic active hours",
            "Windows Update",
            Lm,
            r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "SmartActiveHoursState",
            0
        )
        .with_support(SupportRequirement::windows_build(14_393)),
        policy!(
            "update-active-hours-start",
            "Active hours start",
            "Windows Update",
            Lm,
            r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "ActiveHoursStart",
            u32::from(active_hours_start)
        )
        .with_support(SupportRequirement::windows_build(14_393)),
        policy!(
            "update-active-hours-end",
            "Active hours end",
            "Windows Update",
            Lm,
            r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "ActiveHoursEnd",
            u32::from(active_hours_end)
        )
        .with_support(SupportRequirement::windows_build(14_393)),
        policy!(
            "update-early-features",
            "Early feature updates",
            "Windows Update",
            Lm,
            r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "IsContinuousInnovationOptedIn",
            0
        )
        .with_support(SupportRequirement::windows_build(22_621)),
        policy!(
            "update-signed-in-restart",
            "Restart while signed in",
            "Windows Update",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU",
            "NoAutoRebootWithLoggedOnUsers",
            1
        ),
        policy!(
            "update-restart-notifications",
            "Update restart notifications",
            "Windows Update",
            Lm,
            r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "RestartNotificationsAllowed2",
            1
        )
        .with_choices(choices::ON_OFF),
        policy!(
            "update-drivers",
            "Driver updates",
            "Windows Update",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
            "ExcludeWUDriversInQualityUpdate",
            1
        )
        .off_by_default(),
    ]
}
