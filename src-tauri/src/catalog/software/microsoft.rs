use super::bundle;
use crate::catalog::choices;
use crate::models::RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};
use crate::models::{SoftwareDefinition, SoftwareMarker};

const OFFICE: &str = "Microsoft Office";
const VISUAL_STUDIO: &str = "Visual Studio";
const VISUAL_STUDIO_CODE: &str = "Visual Studio Code";
const VISUAL_STUDIO_CODE_POLICIES: &str = r"SOFTWARE\Policies\Microsoft\VSCode";

pub(super) fn office() -> SoftwareDefinition {
    bundle(
        "microsoft-office",
        OFFICE,
        &[SoftwareMarker::File {
            root: "CommonProgramFiles",
            path: r"microsoft shared\ClickToRun\OfficeClickToRun.exe",
        }],
        vec![
            policy!(
                "office-diagnostic-data",
                "Office diagnostic data",
                OFFICE,
                Cu,
                r"Software\Policies\Microsoft\office\common\clienttelemetry",
                "sendtelemetry",
                3
            )
            .with_choices(choices::OFFICE_DIAGNOSTIC_DATA),
            policy!(
                "office-optional-connected-experiences",
                "Office optional connected experiences",
                OFFICE,
                Cu,
                r"Software\Policies\Microsoft\office\16.0\common\privacy",
                "controllerconnectedservicesenabled",
                2
            )
            .with_choices(choices::OFF_WHEN_TWO),
        ],
        &[],
    )
}

pub(super) fn visual_studio() -> SoftwareDefinition {
    bundle(
        "visual-studio",
        VISUAL_STUDIO,
        &[SoftwareMarker::File {
            root: "ProgramFiles(x86)",
            path: r"Microsoft Visual Studio\Installer\vswhere.exe",
        }],
        vec![
            policy!(
                "visual-studio-experience-program",
                "Visual Studio Customer Experience Improvement Program",
                VISUAL_STUDIO,
                Lm,
                r"SOFTWARE\Policies\Microsoft\VisualStudio\SQM",
                "OptIn",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "visual-studio-feedback",
                "Visual Studio feedback dialog",
                VISUAL_STUDIO,
                Lm,
                r"SOFTWARE\Policies\Microsoft\VisualStudio\Feedback",
                "DisableFeedbackDialog",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "visual-studio-background-downloads",
                "Visual Studio background update downloads",
                VISUAL_STUDIO,
                Lm,
                r"SOFTWARE\Policies\Microsoft\VisualStudio\Setup",
                "BackgroundDownloadDisabled",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
        ],
        &[],
    )
}

pub(super) fn visual_studio_code() -> SoftwareDefinition {
    bundle(
        "visual-studio-code",
        VISUAL_STUDIO_CODE,
        &[
            SoftwareMarker::File {
                root: "LOCALAPPDATA",
                path: r"Programs\Microsoft VS Code\Code.exe",
            },
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: r"Microsoft VS Code\Code.exe",
            },
        ],
        vec![
            text_policy!(
                "vscode-telemetry",
                "VS Code telemetry",
                VISUAL_STUDIO_CODE,
                Lm,
                VISUAL_STUDIO_CODE_POLICIES,
                "TelemetryLevel",
                "off"
            )
            .with_choices(choices::VSCODE_TELEMETRY),
            policy!(
                "vscode-feedback",
                "VS Code feedback prompts",
                VISUAL_STUDIO_CODE,
                Lm,
                VISUAL_STUDIO_CODE_POLICIES,
                "EnableFeedback",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
        ],
        &[],
    )
}
