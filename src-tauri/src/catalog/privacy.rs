use super::choices;
use crate::models::{PolicyDefinition, RegistryHive};

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        absent_policy!(
            "location",
            "Windows location",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableLocation"
        )
        .with_choices(choices::LOCATION_POLICY),
        absent_policy!(
            "location-scripting",
            "Location scripting",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableLocationScripting"
        )
        .with_choices(choices::LOCATION_POLICY),
        absent_policy!(
            "location-provider",
            "Windows location provider",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableWindowsLocationProvider"
        )
        .with_choices(choices::LOCATION_POLICY),
        policy!(
            "location-sensor",
            "Location sensor",
            "Privacy",
            Lm,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}",
            "SensorPermissionState",
            1
        )
        .with_choices(choices::ON_OFF),
        policy!(
            "location-service",
            "Location service",
            "Privacy",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\lfsvc\Service\Configuration",
            "Status",
            1
        )
        .with_choices(choices::ON_OFF),
        policy!(
            "location-service-start",
            "Location service startup",
            "Privacy",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\lfsvc",
            "Start",
            3
        )
        .with_choices(choices::SERVICE_MANUAL),
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
            "LetAppsGetDiagnosticInfo",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-account-info",
            "App access to account info",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessAccountInfo",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        absent_policy!(
            "app-location",
            "App access to location",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessLocation"
        )
        .with_choices(choices::APP_ACCESS_USER),
        policy!(
            "background-apps",
            "Background app access",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsRunInBackground",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-motion",
            "App access to motion",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessMotion",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-trusted-devices",
            "App access to trusted devices",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessTrustedDevices",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-messaging",
            "App access to messages",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessMessaging",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-call-history",
            "App access to call history",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessCallHistory",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-contacts",
            "App access to contacts",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessContacts",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        text_policy!(
            "location-consent",
            "Location capability",
            "Privacy",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
            "Value",
            "Allow"
        )
        .with_choices(choices::CONSENT),
        text_policy!(
            "location-user-consent",
            "App location access",
            "App permissions",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
            "Value",
            "Allow"
        )
        .with_choices(choices::CONSENT),
        text_policy!(
            "location-desktop-consent",
            "Desktop app location access",
            "App permissions",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location\NonPackaged",
            "Value",
            "Allow"
        )
        .with_choices(choices::CONSENT),
        policy!(
            "tailored-experiences-setting",
            "Tailored experiences setting",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy",
            "TailoredExperiencesWithDiagnosticDataEnabled",
            0
        ),
        policy!(
            "online-speech",
            "Online speech recognition",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy",
            "HasAccepted",
            0
        ),
        policy!(
            "inking-typing-personalization",
            "Inking and typing personalization",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\Personalization\Settings",
            "AcceptedPrivacyPolicy",
            0
        ),
        policy!(
            "app-launch-tracking",
            "App launch tracking",
            "Privacy",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_TrackProgs",
            0
        ),
        policy!(
            "device-metadata",
            "Device metadata downloads",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata",
            "PreventDeviceMetadataFromNetwork",
            1
        ),
        policy!(
            "app-calendar",
            "App access to calendar",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessCalendar",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-email",
            "App access to email",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessEmail",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-tasks",
            "App access to tasks",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessTasks",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-phone-calls",
            "App access to phone calls",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessPhone",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-radios",
            "App control of radios",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessRadios",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-notifications",
            "App access to notifications",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessNotifications",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-unpaired-devices",
            "App communication with unpaired devices",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsSyncWithDevices",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-voice-activation",
            "App voice activation",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsActivateWithVoice",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-voice-activation-locked",
            "App voice activation while locked",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsActivateWithVoiceAboveLock",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-eye-tracking",
            "App access to eye tracking",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessGazeInput",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED),
        policy!(
            "app-camera",
            "App access to camera",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessCamera",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED)
        .off_by_default(),
        policy!(
            "app-microphone",
            "App access to microphone",
            "App permissions",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessMicrophone",
            2
        )
        .with_choices(choices::APP_ACCESS_DENIED)
        .off_by_default(),
        policy!(
            "website-language-list",
            "Website access to language list",
            "Privacy",
            Cu,
            r"Control Panel\International\User Profile",
            "HttpAcceptLanguageOptOut",
            1
        ),
        policy!(
            "find-my-device",
            "Find my device",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\FindMyDevice",
            "AllowFindMyDevice",
            0
        ),
        policy!(
            "speech-model-updates",
            "Speech model updates",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Speech",
            "AllowSpeechModelUpdate",
            0
        ),
        policy!(
            "online-font-providers",
            "Online font providers",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "EnableFontProviders",
            0
        ),
        policy!(
            "online-app-lookup",
            "Online app lookup for unknown files",
            "Privacy",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoInternetOpenWith",
            1
        ),
        policy!(
            "media-drm-online",
            "Windows Media DRM internet access",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\WMDRM",
            "DisableOnline",
            1
        ),
        policy!(
            "apps-for-websites",
            "Apps for websites",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "EnableAppUriHandlers",
            0
        ),
        policy!(
            "connectivity-probe",
            "Internet connectivity probe",
            "Privacy",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\NetworkConnectivityStatusIndicator",
            "NoActiveProbe",
            1
        )
        .off_by_default(),
    ]
}
