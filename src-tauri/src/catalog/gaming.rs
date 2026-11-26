use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, SupportRequirement};

const CATEGORY: &str = "Gaming";
const MOUSE: &str = r"Control Panel\Mouse";

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        policy!(
            "game-dvr",
            "Game DVR",
            CATEGORY,
            Cu,
            r"System\GameConfigStore",
            "GameDVR_Enabled",
            0
        ),
        policy!(
            "game-capture",
            "Background game capture",
            CATEGORY,
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
            "AppCaptureEnabled",
            0
        ),
        policy!(
            "game-recording-policy",
            "Game recording policy",
            CATEGORY,
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\GameDVR",
            "AllowGameDVR",
            0
        ),
        policy!(
            "game-bar-controller",
            "Game Bar controller button",
            CATEGORY,
            Cu,
            r"SOFTWARE\Microsoft\GameBar",
            "UseNexusForGameBarEnabled",
            0
        ),
        policy!(
            "game-bar-startup-tips",
            "Game Bar startup tips",
            CATEGORY,
            Cu,
            r"SOFTWARE\Microsoft\GameBar",
            "ShowStartupPanel",
            0
        ),
        policy!(
            "game-mode",
            "Game Mode",
            CATEGORY,
            Cu,
            r"SOFTWARE\Microsoft\GameBar",
            "AutoGameModeEnabled",
            1
        )
        .with_choices(choices::ON_OFF),
        policy!(
            "gpu-scheduling",
            "Hardware-accelerated GPU scheduling",
            CATEGORY,
            Lm,
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "HwSchMode",
            2
        )
        .with_choices(choices::GPU_SCHEDULING)
        .with_support(SupportRequirement::windows_build(19_041)),
        text_policy!(
            "pointer-precision",
            "Enhance pointer precision",
            CATEGORY,
            Cu,
            MOUSE,
            "MouseSpeed",
            "0"
        ),
        text_policy!(
            "pointer-acceleration-threshold",
            "Pointer acceleration threshold",
            CATEGORY,
            Cu,
            MOUSE,
            "MouseThreshold1",
            "0"
        ),
        text_policy!(
            "pointer-acceleration-high-threshold",
            "Pointer high acceleration threshold",
            CATEGORY,
            Cu,
            MOUSE,
            "MouseThreshold2",
            "0"
        ),
        policy!(
            "multimedia-cpu-reservation",
            "Background CPU reservation for multimedia",
            CATEGORY,
            Lm,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            "SystemResponsiveness",
            10
        )
        .off_by_default(),
    ]
}
