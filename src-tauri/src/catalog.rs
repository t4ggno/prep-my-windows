use std::collections::BTreeMap;

use crate::models::{
    AppConfig, DEFAULT_ACTIVE_HOURS_END, DEFAULT_ACTIVE_HOURS_START, NetworkDefinition,
    PackageDefinition, PolicyDefinition, ProcessRule, RegistryHive, RegistryValue,
    ScheduledTaskDefinition, SupportRequirement, SystemSettingDefinition, SystemSettingKind,
};

macro_rules! policy {
    ($id:expr, $name:expr, $category:expr, $hive:expr, $path:expr, $value_name:expr, $value:expr) => {
        PolicyDefinition {
            id: $id,
            name: $name,
            category: $category,
            hive: $hive,
            path: $path,
            value_name: $value_name,
            desired: RegistryValue::Dword($value),
            support: SupportRequirement::any(),
        }
    };
}

macro_rules! absent_policy {
    ($id:expr, $name:expr, $category:expr, $hive:expr, $path:expr, $value_name:expr) => {
        PolicyDefinition {
            id: $id,
            name: $name,
            category: $category,
            hive: $hive,
            path: $path,
            value_name: $value_name,
            desired: RegistryValue::Absent,
            support: SupportRequirement::any(),
        }
    };
}

pub fn policies() -> Vec<PolicyDefinition> {
    policies_with_active_hours(DEFAULT_ACTIVE_HOURS_START, DEFAULT_ACTIVE_HOURS_END)
}

pub fn configured_policies(config: &AppConfig) -> Vec<PolicyDefinition> {
    policies_with_active_hours(config.active_hours_start, config.active_hours_end)
}

fn policies_with_active_hours(
    active_hours_start: u8,
    active_hours_end: u8,
) -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        policy!(
            "diagnostic-data",
            "Diagnostic data",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowTelemetry",
            0
        ),
        policy!(
            "diagnostic-logs",
            "Diagnostic log collection",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "LimitDiagnosticLogCollection",
            1
        ),
        policy!(
            "crash-dumps",
            "Crash dump collection",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "LimitDumpCollection",
            1
        ),
        policy!(
            "feedback-notifications",
            "Feedback requests",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "DoNotShowFeedbackNotifications",
            1
        ),
        policy!(
            "device-name-telemetry",
            "Device name in diagnostics",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowDeviceNameInTelemetry",
            0
        ),
        policy!(
            "application-telemetry",
            "Application telemetry",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "AITEnable",
            0
        ),
        policy!(
            "application-inventory",
            "Application inventory",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisableInventory",
            1
        ),
        policy!(
            "explorer-instrumentation",
            "Explorer instrumentation",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoInstrumentation",
            1
        ),
        policy!(
            "error-reporting",
            "Windows error reporting",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
            "Disabled",
            1
        ),
        policy!(
            "customer-experience",
            "Customer Experience Improvement Program",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\SQMClient\Windows",
            "CEIPEnable",
            0
        ),
        policy!(
            "connected-user-service",
            "Connected User Experiences service",
            "Diagnostics",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\DiagTrack",
            "Start",
            4
        ),
        policy!(
            "silent-admin-elevation",
            "Administrator elevation prompts",
            "Windows behavior",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "ConsentPromptBehaviorAdmin",
            0
        ),
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
            "wap-push-service",
            "Device management push service",
            "Diagnostics",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\dmwappushservice",
            "Start",
            4
        ),
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
            "search-box-suggestions",
            "Search box web suggestions",
            "Search & AI",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "DisableSearchBoxSuggestions",
            1
        ),
        policy!(
            "cloud-search",
            "Cloud search",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AllowCloudSearch",
            0
        ),
        policy!(
            "web-search",
            "Windows web search",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "DisableWebSearch",
            1
        ),
        policy!(
            "connected-search",
            "Connected search",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "ConnectedSearchUseWeb",
            0
        ),
        policy!(
            "cortana",
            "Cortana",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AllowCortana",
            0
        ),
        policy!(
            "recall-component",
            "Recall",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "AllowRecallEnablement",
            0
        ),
        policy!(
            "recall-snapshots",
            "Recall snapshots",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "DisableAIDataAnalysis",
            1
        ),
        policy!(
            "click-to-do",
            "Click to Do",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "DisableClickToDo",
            1
        ),
        policy!(
            "settings-agent",
            "Settings agent",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "DisableSettingsAgent",
            1
        ),
        policy!(
            "paint-cocreator",
            "Paint Cocreator",
            "Search & AI",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableCocreator",
            1
        ),
        policy!(
            "paint-image-creator",
            "Paint Image Creator",
            "Search & AI",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableImageCreator",
            1
        ),
        policy!(
            "paint-generative-fill",
            "Paint generative fill",
            "Search & AI",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableGenerativeFill",
            1
        ),
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
        ),
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
        absent_policy!(
            "location",
            "Windows location",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableLocation"
        ),
        absent_policy!(
            "location-scripting",
            "Location scripting",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableLocationScripting"
        ),
        absent_policy!(
            "location-provider",
            "Windows location provider",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableWindowsLocationProvider"
        ),
        policy!(
            "location-sensor",
            "Location sensor",
            "Privacy",
            Lm,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}",
            "SensorPermissionState",
            1
        ),
        policy!(
            "location-service",
            "Location service",
            "Privacy",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\lfsvc\Service\Configuration",
            "Status",
            1
        ),
        policy!(
            "location-service-start",
            "Location service startup",
            "Privacy",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\lfsvc",
            "Start",
            3
        ),
        policy!(
            "typing-data",
            "Typing data collection",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\Input\TIPC",
            "Enabled",
            0
        ),
        policy!(
            "implicit-ink",
            "Implicit ink collection",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\InputPersonalization",
            "RestrictImplicitInkCollection",
            1
        ),
        policy!(
            "implicit-text",
            "Implicit text collection",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\InputPersonalization",
            "RestrictImplicitTextCollection",
            1
        ),
        policy!(
            "contact-harvesting",
            "Contact harvesting",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\InputPersonalization\TrainedDataStore",
            "HarvestContacts",
            0
        ),
        policy!(
            "handwriting-data",
            "Handwriting data sharing",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\TabletPC",
            "PreventHandwritingDataSharing",
            1
        ),
        policy!(
            "handwriting-errors",
            "Handwriting error reports",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports",
            "PreventHandwritingErrorReports",
            1
        ),
        policy!(
            "app-diagnostics",
            "App access to diagnostics",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessDiagnosticInfo",
            2
        ),
        policy!(
            "app-account-info",
            "App access to account info",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessAccountInfo",
            2
        ),
        absent_policy!(
            "app-location",
            "App access to location",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessLocation"
        ),
        policy!(
            "background-apps",
            "Background app access",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsRunInBackground",
            2
        ),
        policy!(
            "app-motion",
            "App access to motion",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessMotion",
            2
        ),
        policy!(
            "app-trusted-devices",
            "App access to trusted devices",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessTrustedDevices",
            2
        ),
        policy!(
            "app-messaging",
            "App access to messages",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessMessaging",
            2
        ),
        policy!(
            "app-call-history",
            "App access to call history",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessCallHistory",
            2
        ),
        policy!(
            "app-contacts",
            "App access to contacts",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessContacts",
            2
        ),
        policy!(
            "edge-first-run",
            "Edge first-run experience",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "HideFirstRunExperience",
            1
        )
        .with_support(SupportRequirement::edge(80)),
        policy!(
            "edge-default-browser-campaign",
            "Edge default-browser campaign",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "DefaultBrowserSettingsCampaignEnabled",
            0
        )
        .with_support(SupportRequirement::edge(113)),
        policy!(
            "edge-pdf-handler-recommendations",
            "Edge PDF-handler recommendations",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ShowPDFDefaultRecommendationsEnabled",
            0
        )
        .with_support(SupportRequirement::edge(93)),
        policy!(
            "edge-personalization",
            "Edge personalization reporting",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "PersonalizationReportingEnabled",
            0
        ),
        policy!(
            "edge-serp-telemetry",
            "Edge search result telemetry",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "Edge3PSerpTelemetryEnabled",
            0
        ),
        policy!(
            "edge-feedback",
            "Edge feedback",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "UserFeedbackAllowed",
            0
        ),
        policy!(
            "edge-search-suggestions",
            "Edge search suggestions",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SearchSuggestEnabled",
            0
        ),
        policy!(
            "edge-network-prediction",
            "Edge network prediction",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "NetworkPredictionOptions",
            2
        ),
        policy!(
            "edge-error-pages",
            "Edge web error service",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "AlternateErrorPagesEnabled",
            0
        ),
        policy!(
            "edge-navigation-errors",
            "Edge navigation error service",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ResolveNavigationErrorsUseWebService",
            0
        ),
        policy!(
            "edge-sync",
            "Edge sync",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SyncDisabled",
            1
        ),
        policy!(
            "edge-signin",
            "Edge browser sign-in",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "BrowserSignin",
            0
        ),
        policy!(
            "edge-shopping",
            "Edge shopping assistant",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "EdgeShoppingAssistantEnabled",
            0
        ),
        policy!(
            "edge-recommendations",
            "Edge recommendations",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ShowRecommendationsEnabled",
            0
        ),
        policy!(
            "edge-spotlight",
            "Edge spotlight experiences",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SpotlightExperiencesAndRecommendationsEnabled",
            0
        ),
        policy!(
            "edge-promotions",
            "Edge promotional tabs",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "PromotionalTabsEnabled",
            0
        ),
        policy!(
            "edge-sidebar",
            "Edge sidebar",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "HubsSidebarEnabled",
            0
        ),
        policy!(
            "edge-startup-boost",
            "Edge startup boost",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "StartupBoostEnabled",
            0
        ),
        policy!(
            "edge-background",
            "Edge background mode",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "BackgroundModeEnabled",
            0
        ),
        policy!(
            "edge-new-tab-content",
            "Edge new tab content",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "NewTabPageContentEnabled",
            0
        ),
        policy!(
            "edge-insider-promotion",
            "Edge Insider promotion",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "MicrosoftEdgeInsiderPromotionEnabled",
            0
        ),
        policy!(
            "edge-copilot-context",
            "Edge Copilot page context",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "CopilotPageContextEnabled",
            0
        ),
        policy!(
            "edge-assets",
            "Edge asset delivery",
            "Microsoft Edge",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Edge",
            "EdgeAssetDeliveryServiceEnabled",
            0
        ),
        PolicyDefinition {
            id: "location-consent",
            name: "Location capability",
            category: "Privacy",
            hive: Lm,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
            value_name: "Value",
            desired: RegistryValue::Text("Allow"),
            support: SupportRequirement::any(),
        },
        PolicyDefinition {
            id: "location-user-consent",
            name: "App location access",
            category: "App permissions",
            hive: Cu,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
            value_name: "Value",
            desired: RegistryValue::Text("Allow"),
            support: SupportRequirement::any(),
        },
        PolicyDefinition {
            id: "location-desktop-consent",
            name: "Desktop app location access",
            category: "App permissions",
            hive: Cu,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location\NonPackaged",
            value_name: "Value",
            desired: RegistryValue::Text("Allow"),
            support: SupportRequirement::any(),
        },
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
            support: SupportRequirement::windows_build(22_000),
        },
    ]
}

pub fn packages() -> Vec<PackageDefinition> {
    [
        ("clipchamp", "Clipchamp", "Clipchamp.Clipchamp"),
        ("news", "Microsoft News", "Microsoft.BingNews"),
        ("weather", "Microsoft Weather", "Microsoft.BingWeather"),
        ("gaming-app", "Xbox app", "Microsoft.GamingApp"),
        ("get-help", "Get Help", "Microsoft.GetHelp"),
        ("get-started", "Get Started", "Microsoft.Getstarted"),
        (
            "office-hub",
            "Microsoft 365",
            "Microsoft.MicrosoftOfficeHub",
        ),
        (
            "solitaire",
            "Microsoft Solitaire Collection",
            "Microsoft.MicrosoftSolitaireCollection",
        ),
        (
            "mixed-reality",
            "Mixed Reality Portal",
            "Microsoft.MixedReality.Portal",
        ),
        ("people", "Microsoft People", "Microsoft.People"),
        (
            "power-automate",
            "Power Automate",
            "Microsoft.PowerAutomateDesktop",
        ),
        ("skype", "Skype", "Microsoft.SkypeApp"),
        ("todo", "Microsoft To Do", "Microsoft.Todos"),
        (
            "feedback-hub",
            "Feedback Hub",
            "Microsoft.WindowsFeedbackHub",
        ),
        ("maps", "Windows Maps", "Microsoft.WindowsMaps"),
        ("xbox-tcui", "Xbox TCUI", "Microsoft.Xbox.TCUI"),
        ("xbox-app", "Xbox Console Companion", "Microsoft.XboxApp"),
        (
            "xbox-overlay",
            "Xbox Game Overlay",
            "Microsoft.XboxGameOverlay",
        ),
        (
            "xbox-gamebar",
            "Xbox Game Bar",
            "Microsoft.XboxGamingOverlay",
        ),
        (
            "xbox-identity",
            "Xbox Identity Provider",
            "Microsoft.XboxIdentityProvider",
        ),
        (
            "xbox-speech",
            "Xbox Speech to Text",
            "Microsoft.XboxSpeechToTextOverlay",
        ),
        ("phone-link", "Phone Link", "Microsoft.YourPhone"),
        (
            "media-player",
            "Windows Media Player",
            "Microsoft.ZuneMusic",
        ),
        ("movies-tv", "Movies & TV", "Microsoft.ZuneVideo"),
        (
            "family",
            "Microsoft Family",
            "MicrosoftCorporationII.MicrosoftFamily",
        ),
        (
            "quick-assist",
            "Quick Assist",
            "MicrosoftCorporationII.QuickAssist",
        ),
        ("teams-classic", "Microsoft Teams classic", "MicrosoftTeams"),
        ("teams", "Microsoft Teams", "MSTeams"),
        (
            "outlook",
            "Outlook for Windows",
            "Microsoft.OutlookForWindows",
        ),
        ("dev-home", "Dev Home", "Microsoft.Windows.DevHome"),
        ("cortana-app", "Cortana", "Microsoft.549981C3F5F10"),
        (
            "web-experience",
            "Windows Web Experience Pack",
            "MicrosoftWindows.Client.WebExperience",
        ),
        ("copilot-app", "Microsoft Copilot", "Microsoft.Copilot"),
    ]
    .into_iter()
    .map(|(id, name, pattern)| PackageDefinition { id, name, pattern })
    .collect()
}

pub fn network_blocks() -> Vec<NetworkDefinition> {
    [
        ("events", "v10.events.data.microsoft.com"),
        ("events-config", "v10c.events.data.microsoft.com"),
        ("events-config-us", "us-v10c.events.data.microsoft.com"),
        ("events-config-eu", "eu-v10c.events.data.microsoft.com"),
        ("vortex-windows", "v10.vortex-win.data.microsoft.com"),
        ("self-events", "self.events.data.microsoft.com"),
        ("functional-events", "functional.events.data.microsoft.com"),
        ("browser-events", "browser.events.data.msn.com"),
        ("watson-telemetry", "watson.telemetry.microsoft.com"),
        ("watson-events", "watson.events.data.microsoft.com"),
        ("watson-user-events", "umwatsonc.events.data.microsoft.com"),
        ("watson-current", "watsonc.events.data.microsoft.com"),
        ("watson-current-us", "us-watsonc.events.data.microsoft.com"),
        ("watson-current-eu", "eu-watsonc.events.data.microsoft.com"),
        ("watson-kernel", "kmwatsonc.events.data.microsoft.com"),
        ("oca-telemetry", "oca.telemetry.microsoft.com"),
        ("oca", "oca.microsoft.com"),
        ("telecommand", "telecommand.telemetry.microsoft.com"),
        ("telecommand-service", "www.telecommandsvc.microsoft.com"),
        ("settings-windows", "settings-win.data.microsoft.com"),
        ("settings", "settings.data.microsoft.com"),
        ("wer-blob-ceus-1", "ceuswatcab01.blob.core.windows.net"),
        ("wer-blob-ceus-2", "ceuswatcab02.blob.core.windows.net"),
        ("wer-blob-eaus-1", "eaus2watcab01.blob.core.windows.net"),
        ("wer-blob-eaus-2", "eaus2watcab02.blob.core.windows.net"),
        ("wer-blob-weus-1", "weus2watcab01.blob.core.windows.net"),
        ("wer-blob-weus-2", "weus2watcab02.blob.core.windows.net"),
    ]
    .into_iter()
    .map(|(id, domain)| NetworkDefinition { id, domain })
    .collect()
}

pub fn scheduled_tasks() -> Vec<ScheduledTaskDefinition> {
    [
        (
            "compatibility-appraiser",
            "Microsoft Compatibility Appraiser",
            r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser",
        ),
        (
            "program-data-updater",
            "Program Data Updater",
            r"\Microsoft\Windows\Application Experience\ProgramDataUpdater",
        ),
        (
            "startup-app-task",
            "Startup App Task",
            r"\Microsoft\Windows\Application Experience\StartupAppTask",
        ),
        (
            "ceip-consolidator",
            "CEIP Consolidator",
            r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator",
        ),
        (
            "usb-ceip",
            "USB CEIP",
            r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip",
        ),
        (
            "disk-diagnostics",
            "Disk diagnostic collector",
            r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector",
        ),
        (
            "feedback-client",
            "Feedback client",
            r"\Microsoft\Windows\Feedback\Siuf\DmClient",
        ),
        (
            "feedback-scenario",
            "Feedback scenario downloader",
            r"\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload",
        ),
        (
            "maps-update",
            "Maps update",
            r"\Microsoft\Windows\Maps\MapsUpdateTask",
        ),
        (
            "wer-queue",
            "Error reporting queue",
            r"\Microsoft\Windows\Windows Error Reporting\QueueReporting",
        ),
    ]
    .into_iter()
    .map(|(id, name, path)| ScheduledTaskDefinition { id, name, path })
    .collect()
}

fn default_process_rules() -> Vec<ProcessRule> {
    [
        ("onedrive", "Microsoft OneDrive", "OneDrive.exe"),
        ("teams", "Microsoft Teams", "ms-teams.exe"),
        ("teams-classic", "Microsoft Teams classic", "Teams.exe"),
        ("widgets", "Windows Widgets", "Widgets.exe"),
        (
            "widget-service",
            "Windows Widget Service",
            "WidgetService.exe",
        ),
        ("copilot", "Microsoft Copilot", "Copilot.exe"),
        ("phone-link", "Phone Link", "PhoneExperienceHost.exe"),
        ("game-bar", "Xbox Game Bar", "GameBar.exe"),
        (
            "game-bar-server",
            "Xbox Game Bar server",
            "GameBarFTServer.exe",
        ),
        ("discord", "Discord", "Discord.exe"),
        ("spotify", "Spotify", "Spotify.exe"),
        ("steam", "Steam", "steam.exe"),
        ("epic", "Epic Games Launcher", "EpicGamesLauncher.exe"),
        ("battle-net", "Battle.net", "Battle.net.exe"),
        (
            "creative-cloud",
            "Adobe Creative Cloud",
            "Creative Cloud.exe",
        ),
        (
            "adobe-service",
            "Adobe Desktop Service",
            "Adobe Desktop Service.exe",
        ),
        ("ccx-process", "Adobe CCX Process", "CCXProcess.exe"),
        (
            "adobe-collab",
            "Adobe Collaboration Synchronizer",
            "AdobeCollabSync.exe",
        ),
        ("dropbox", "Dropbox", "Dropbox.exe"),
        ("google-drive", "Google Drive", "GoogleDriveFS.exe"),
        ("slack", "Slack", "slack.exe"),
        ("notion", "Notion", "Notion.exe"),
        ("whatsapp", "WhatsApp", "WhatsApp.exe"),
        ("zoom", "Zoom", "Zoom.exe"),
    ]
    .into_iter()
    .map(|(id, name, executable_name)| ProcessRule {
        id: format!("built-in-{id}"),
        name: name.to_owned(),
        executable_name: executable_name.to_owned(),
        executable_path: None,
        enabled: true,
        built_in: true,
    })
    .collect()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            policies: policies()
                .into_iter()
                .map(|item| (item.id.to_owned(), true))
                .chain(
                    system_settings()
                        .into_iter()
                        .map(|item| (item.id.to_owned(), true)),
                )
                .collect::<BTreeMap<_, _>>(),
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
                .map(|item| (item.id.to_owned(), true))
                .collect::<BTreeMap<_, _>>(),
            process_rules: default_process_rules(),
            custom_packages: Vec::new(),
            blocked_autostarts: Vec::new(),
            enforcement_interval_seconds: 10,
            package_interval_minutes: 10,
            start_with_windows: true,
            active_hours_start: DEFAULT_ACTIVE_HOURS_START,
            active_hours_end: DEFAULT_ACTIVE_HOURS_END,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{
        configured_policies, network_blocks, packages, policies, scheduled_tasks, system_settings,
    };
    use crate::models::{AppConfig, RegistryValue};

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
    fn default_profile_enables_every_catalog_item() {
        let config = AppConfig::default();
        assert!(config.policies.values().all(|enabled| *enabled));
        assert!(config.packages.values().all(|enabled| *enabled));
        assert!(config.network_blocks.values().all(|enabled| *enabled));
        assert!(config.scheduled_tasks.values().all(|enabled| *enabled));
        assert!(config.process_rules.iter().all(|rule| rule.enabled));
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
}
