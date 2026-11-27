use super::bundle;
use crate::catalog::choices;
use crate::models::RegistryHive::LocalMachine as Lm;
use crate::models::{SoftwareDefinition, SoftwareMarker};

const ACROBAT: &str = "Adobe Acrobat";
const ACROBAT_LOCKDOWN: &str = r"SOFTWARE\Policies\Adobe\Adobe Acrobat\DC\FeatureLockDown";
const CREATIVE_CLOUD: &str = "Adobe Creative Cloud";

pub(super) fn acrobat() -> SoftwareDefinition {
    bundle(
        "adobe-acrobat",
        ACROBAT,
        &[SoftwareMarker::File {
            root: "ProgramFiles",
            path: r"Adobe\Acrobat DC\Acrobat\Acrobat.exe",
        }],
        vec![
            policy!(
                "acrobat-ai-assistant",
                "Acrobat AI Assistant",
                ACROBAT,
                Lm,
                ACROBAT_LOCKDOWN,
                "bEnableGentech",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "acrobat-upsell",
                "Acrobat upgrade offers",
                ACROBAT,
                Lm,
                ACROBAT_LOCKDOWN,
                "bAcroSuppressUpsell",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "acrobat-usage-data",
                "Acrobat usage data",
                ACROBAT,
                Lm,
                ACROBAT_LOCKDOWN,
                "bUsageMeasurement",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "acrobat-update-service",
                "Acrobat update service",
                ACROBAT,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\AdobeARMservice",
                "Start",
                3
            )
            .with_choices(choices::SERVICE_START),
        ],
        &[],
    )
}

pub(super) fn creative_cloud() -> SoftwareDefinition {
    bundle(
        "adobe-creative-cloud",
        CREATIVE_CLOUD,
        &[
            SoftwareMarker::Service("AGSService"),
            SoftwareMarker::Service("AdobeUpdateService"),
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: r"Adobe\Adobe Creative Cloud\ACC\Creative Cloud.exe",
            },
        ],
        vec![
            policy!(
                "creative-cloud-genuine-service",
                "Adobe Genuine Software Integrity service",
                CREATIVE_CLOUD,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\AGSService",
                "Start",
                4
            )
            .with_choices(choices::SERVICE_START),
            policy!(
                "creative-cloud-genuine-monitor",
                "Adobe Genuine Monitor service",
                CREATIVE_CLOUD,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\AGMService",
                "Start",
                4
            )
            .with_choices(choices::SERVICE_START),
            policy!(
                "creative-cloud-update-service",
                "Adobe Update service",
                CREATIVE_CLOUD,
                Lm,
                r"SYSTEM\CurrentControlSet\Services\AdobeUpdateService",
                "Start",
                3
            )
            .with_choices(choices::SERVICE_START),
        ],
        &[],
    )
}
