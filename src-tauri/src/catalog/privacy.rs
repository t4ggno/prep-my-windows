use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, RegistryValue, SupportRequirement};

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
            "LetAppsAccessDiagnosticInfo",
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
        PolicyDefinition {
            id: "location-consent",
            name: "Location capability",
            category: "Privacy",
            hive: Lm,
            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
            value_name: "Value",
            desired: RegistryValue::Text("Allow"),
            choices: choices::CONSENT,
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
            choices: choices::CONSENT,
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
            choices: choices::CONSENT,
            support: SupportRequirement::any(),
        },
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
    ]
}
