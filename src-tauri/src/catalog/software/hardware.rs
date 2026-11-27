use super::bundle;
use crate::catalog::choices;
use crate::models::RegistryHive::LocalMachine as Lm;
use crate::models::{SoftwareDefinition, SoftwareMarker};

const AMD: &str = "AMD Software";
const NVIDIA: &str = "NVIDIA";
const LENOVO: &str = "Lenovo";

pub(super) fn amd() -> SoftwareDefinition {
    bundle(
        "amd-software",
        AMD,
        &[
            SoftwareMarker::Service("AMD External Events Utility"),
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: r"AMD\CNext\CNext\RadeonSoftware.exe",
            },
        ],
        vec![
            policy!(
                "amd-external-events-service",
                "AMD External Events service",
                AMD,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\AMD External Events Utility",
                "Start",
                4
            )
            .with_choices(choices::SERVICE_START),
            policy!(
                "amd-crash-defender-service",
                "AMD Crash Defender service",
                AMD,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\AMD Crash Defender Service",
                "Start",
                4
            )
            .with_choices(choices::SERVICE_START),
        ],
        &[
            (
                "amd-software-startup",
                "Radeon Software at sign-in",
                r"\StartCN",
            ),
            (
                "amd-recording-startup",
                "AMD recording at sign-in",
                r"\StartDVR",
            ),
            (
                "amd-update-check",
                "AMD Install Manager update check",
                r"\AMD Install Manager - Check For Updates",
            ),
        ],
    )
}

pub(super) fn nvidia() -> SoftwareDefinition {
    bundle(
        "nvidia",
        NVIDIA,
        &[SoftwareMarker::Service("NVDisplay.ContainerLocalSystem")],
        vec![
            policy!(
                "nvidia-usage-data",
                "NVIDIA usage data",
                NVIDIA,
                Lm,
                r"SOFTWARE\NVIDIA Corporation\NvControlPanel2\Client",
                "OptInOrOutPreference",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "nvidia-driver-telemetry",
                "NVIDIA driver telemetry",
                NVIDIA,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\Startup",
                "SendTelemetryData",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
        ],
        &[],
    )
}

pub(super) fn lenovo() -> SoftwareDefinition {
    bundle(
        "lenovo",
        LENOVO,
        &[
            SoftwareMarker::Service("UDCService"),
            SoftwareMarker::Service("SUService"),
        ],
        vec![
            policy!(
                "lenovo-device-client-service",
                "Lenovo Universal Device Client service",
                LENOVO,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\UDCService",
                "Start",
                3
            )
            .with_choices(choices::SERVICE_START),
        ],
        &[
            (
                "lenovo-system-update-install",
                "Lenovo System Update automatic install",
                r"\TVT\TVSUUpdateTask",
            ),
            (
                "lenovo-system-update-sign-in",
                "Lenovo System Update at sign-in",
                r"\TVT\TVSUUpdateTask_UserLogOn",
            ),
        ],
    )
}
