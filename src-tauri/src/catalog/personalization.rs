use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, RegistryValue, SupportRequirement};

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        policy!(
            "advertising-id-machine",
            "Advertising ID policy",
            "Personalization",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
            "DisabledByGroupPolicy",
            1
        ),
        policy!(
            "advertising-id-user",
            "Advertising ID",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
            "Enabled",
            0
        ),
        policy!(
            "tailored-experiences",
            "Tailored experiences",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableTailoredExperiencesWithDiagnosticData",
            1
        ),
        policy!(
            "consumer-features",
            "Microsoft consumer features",
            "Personalization",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsConsumerFeatures",
            1
        ),
        policy!(
            "third-party-suggestions",
            "Third-party suggestions",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableThirdPartySuggestions",
            1
        ),
        policy!(
            "spotlight",
            "Windows Spotlight",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightFeatures",
            1
        ),
        policy!(
            "spotlight-action-center",
            "Spotlight in notifications",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightOnActionCenter",
            1
        ),
        policy!(
            "spotlight-settings",
            "Settings suggestions",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightOnSettings",
            1
        ),
        policy!(
            "welcome-experience",
            "Windows welcome experience",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightWindowsWelcomeExperience",
            1
        ),
        policy!(
            "soft-landing",
            "Windows tips",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableSoftLanding",
            1
        ),
        policy!(
            "cloud-content",
            "Cloud optimized content",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableCloudOptimizedContent",
            1
        ),
        policy!(
            "silent-app-installs",
            "Silent suggested app installs",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SilentInstalledAppsEnabled",
            0
        ),
        policy!(
            "start-suggestions",
            "Start menu suggestions",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SystemPaneSuggestionsEnabled",
            0
        ),
        policy!(
            "start-iris-recommendations",
            "Start tips and app recommendations",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_IrisRecommendations",
            0
        )
        .with_support(SupportRequirement::windows_build(22_000)),
        policy!(
            "start-recommended-section",
            "Start Recommended section",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "HideRecommendedSection",
            1
        )
        .with_support(SupportRequirement::professional(22_621)),
        policy!(
            "start-recent-items",
            "Start recent items",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_TrackDocs",
            0
        )
        .with_support(SupportRequirement::windows_build(22_000)),
        policy!(
            "start-recent-apps",
            "Start recently added apps",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Start",
            "ShowRecentList",
            0
        ),
        policy!(
            "lock-screen-spotlight",
            "Lock screen Spotlight",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "RotatingLockScreenEnabled",
            0
        ),
        policy!(
            "lock-screen-overlay",
            "Lock screen suggestions",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "RotatingLockScreenOverlayEnabled",
            0
        ),
        policy!(
            "content-tips",
            "Suggested content",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338388Enabled",
            0
        ),
        policy!(
            "content-start",
            "Suggested apps in Start",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338389Enabled",
            0
        ),
        policy!(
            "content-welcome",
            "Suggested welcome content",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-353694Enabled",
            0
        ),
        policy!(
            "content-settings",
            "Suggested Settings content",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-353696Enabled",
            0
        ),
        policy!(
            "lock-screen-tips",
            "Lock screen tips",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338387Enabled",
            0
        ),
        policy!(
            "content-after-updates",
            "Welcome experience after updates",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-310093Enabled",
            0
        ),
        policy!(
            "content-settings-suggestions",
            "Suggested content in Settings",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338393Enabled",
            0
        ),
        policy!(
            "content-timeline",
            "Timeline suggestions",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-353698Enabled",
            0
        ),
        policy!(
            "setup-reminders",
            "Finish setup reminders",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\UserProfileEngagement",
            "ScoobeSystemSettingEnabled",
            0
        ),
        policy!(
            "settings-account-notifications",
            "Account notifications in Settings",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\SystemSettings\AccountNotifications",
            "EnableAccountNotifications",
            0
        ),
        policy!(
            "start-account-notifications",
            "Account notifications in Start",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_AccountNotifications",
            0
        ),
        policy!(
            "explorer-sync-ads",
            "Sync provider ads in File Explorer",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowSyncProviderNotifications",
            0
        ),
        policy!(
            "suggested-notifications",
            "Suggested notifications",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.Suggested",
            "Enabled",
            0
        ),
        policy!(
            "backup-reminders",
            "Backup reminders",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.BackupReminder",
            "Enabled",
            0
        ),
        policy!(
            "mobile-device-suggestions",
            "Mobile device suggestions",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Mobility",
            "OptedIn",
            0
        ),
        policy!(
            "start-phone-link",
            "Phone Link in Start",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Start\Companions\Microsoft.YourPhone_8wekyb3d8bbwe",
            "IsEnabled",
            0
        )
        .with_support(SupportRequirement::windows_build(22_621)),
        policy!(
            "start-website-recommendations",
            "Website recommendations in Start",
            "Personalization",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "HideRecommendedPersonalizedSites",
            1
        )
        .with_support(SupportRequirement::windows_build(22_621)),
        policy!(
            "desktop-spotlight",
            "Desktop Spotlight",
            "Personalization",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableSpotlightCollectionOnDesktop",
            1
        )
        .with_support(SupportRequirement::windows_build(22_621)),
        policy!(
            "desktop-spotlight-icon",
            "Desktop Spotlight icon",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
            "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}",
            1
        )
        .with_support(SupportRequirement::windows_build(22_621)),
        policy!(
            "settings-account-ads",
            "Microsoft 365 ads in Settings",
            "Personalization",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableConsumerAccountStateContent",
            1
        ),
        PolicyDefinition {
            id: "settings-home",
            name: "Settings home page",
            category: "Personalization",
            hive: Lm,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            value_name: "SettingsPageVisibility",
            desired: RegistryValue::Text("hide:home"),
            choices: &[],
            support: SupportRequirement::windows_build(22_621),
        },
        policy!(
            "widgets",
            "Widgets",
            "Personalization",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Dsh",
            "AllowNewsAndInterests",
            0
        )
        .with_support(SupportRequirement::windows_build(22_000)),
        policy!(
            "news-and-interests",
            "News and interests",
            "Personalization",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Feeds",
            "EnableFeeds",
            0
        ),
        policy!(
            "taskbar-search",
            "Taskbar search",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
            "SearchboxTaskbarMode",
            1
        )
        .with_choices(choices::TASKBAR_SEARCH)
        .with_support(SupportRequirement::windows_build(22_000)),
        policy!(
            "chat-taskbar-button",
            "Chat taskbar button",
            "Personalization",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarMn",
            0
        )
        .with_support(SupportRequirement::windows_build(22_000)),
        policy!(
            "meet-now",
            "Meet Now",
            "Personalization",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "HideSCAMeetNow",
            1
        ),
    ]
}
