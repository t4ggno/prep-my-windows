use super::choices;
use crate::models::{PolicyDefinition, RegistryHive};

pub(super) fn policies() -> Vec<PolicyDefinition> {
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
        )
        .with_choices(choices::DIAGNOSTIC_DATA),
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
        )
        .with_choices(choices::SERVICE_DISABLED),
        policy!(
            "wap-push-service",
            "Device management push service",
            "Diagnostics",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\dmwappushservice",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_DISABLED),
        policy!(
            "diagnostic-data-preference",
            "Diagnostic data preference",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
            "AllowTelemetry",
            0
        )
        .with_choices(choices::DIAGNOSTIC_DATA),
        policy!(
            "feedback-frequency",
            "Feedback frequency",
            "Diagnostics",
            Cu,
            r"SOFTWARE\Microsoft\Siuf\Rules",
            "NumberOfSIUFInPeriod",
            0
        ),
        policy!(
            "error-reporting-service",
            "Error reporting service",
            "Diagnostics",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\WerSvc",
            "Start",
            4
        )
        .with_choices(choices::SERVICE_DISABLED),
        policy!(
            "defender-sample-submission",
            "Defender sample submission",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows Defender\Spynet",
            "SubmitSamplesConsent",
            2
        )
        .with_choices(choices::DEFENDER_SAMPLES),
        policy!(
            "one-settings-downloads",
            "OneSettings configuration downloads",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "DisableOneSettingsDownloads",
            1
        ),
        policy!(
            "diagnostic-change-notifications",
            "Diagnostic data change notifications",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "DisableTelemetryOptInChangeNotification",
            1
        ),
        policy!(
            "error-reporting-additional-data",
            "Additional error report data",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
            "DontSendAdditionalData",
            1
        ),
        policy!(
            "removal-tool-reports",
            "Malicious Software Removal Tool reports",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\MRT",
            "DontReportInfectionInformation",
            1
        ),
        policy!(
            "steps-recorder",
            "Steps Recorder",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisableUAR",
            1
        ),
        policy!(
            "disk-health-updates",
            "Disk health model updates",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth",
            "AllowDiskHealthModelUpdates",
            0
        ),
        policy!(
            "kms-online-validation",
            "KMS client online validation",
            "Diagnostics",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows NT\CurrentVersion\Software Protection Platform",
            "NoGenTicket",
            1
        ),
    ]
}
