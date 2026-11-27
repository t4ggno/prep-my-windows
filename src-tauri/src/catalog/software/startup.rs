use super::bundle;
use crate::catalog::choices;
use crate::models::RegistryHive::{self, CurrentUser as Cu, LocalMachine as Lm};
use crate::models::{PolicyDefinition, SoftwareDefinition, SoftwareMarker};

const RUN: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";

fn startup_entry(
    id: &'static str,
    name: &'static str,
    category: &'static str,
    hive: RegistryHive,
    value_name: &'static str,
) -> PolicyDefinition {
    absent_policy!(id, name, category, hive, RUN, value_name).with_choices(choices::STARTUP_ENTRY)
}

pub(super) fn steam() -> SoftwareDefinition {
    bundle(
        "steam",
        "Steam",
        &[SoftwareMarker::File {
            root: "ProgramFiles(x86)",
            path: r"Steam\steam.exe",
        }],
        vec![startup_entry(
            "steam-startup",
            "Steam at sign-in",
            "Steam",
            Cu,
            "Steam",
        )],
        &[],
    )
}

pub(super) fn discord() -> SoftwareDefinition {
    bundle(
        "discord",
        "Discord",
        &[SoftwareMarker::File {
            root: "LOCALAPPDATA",
            path: r"Discord\Update.exe",
        }],
        vec![startup_entry(
            "discord-startup",
            "Discord at sign-in",
            "Discord",
            Cu,
            "Discord",
        )],
        &[],
    )
}

pub(super) fn spotify() -> SoftwareDefinition {
    bundle(
        "spotify",
        "Spotify",
        &[SoftwareMarker::File {
            root: "APPDATA",
            path: r"Spotify\Spotify.exe",
        }],
        vec![startup_entry(
            "spotify-startup",
            "Spotify at sign-in",
            "Spotify",
            Cu,
            "Spotify",
        )],
        &[],
    )
}

pub(super) fn epic_games() -> SoftwareDefinition {
    bundle(
        "epic-games",
        "Epic Games Launcher",
        &[
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: r"Epic Games\Launcher\Portal\Binaries\Win64\EpicGamesLauncher.exe",
            },
            SoftwareMarker::File {
                root: "ProgramFiles(x86)",
                path: r"Epic Games\Launcher\Portal\Binaries\Win64\EpicGamesLauncher.exe",
            },
        ],
        vec![startup_entry(
            "epic-games-startup",
            "Epic Games Launcher at sign-in",
            "Epic Games Launcher",
            Cu,
            "EpicGamesLauncher",
        )],
        &[],
    )
}

pub(super) fn gog_galaxy() -> SoftwareDefinition {
    bundle(
        "gog-galaxy",
        "GOG Galaxy",
        &[
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: r"GOG Galaxy\GalaxyClient.exe",
            },
            SoftwareMarker::File {
                root: "ProgramFiles(x86)",
                path: r"GOG Galaxy\GalaxyClient.exe",
            },
        ],
        vec![startup_entry(
            "gog-galaxy-startup",
            "GOG Galaxy at sign-in",
            "GOG Galaxy",
            Cu,
            "GogGalaxy",
        )],
        &[],
    )
}

pub(super) fn logitech() -> SoftwareDefinition {
    bundle(
        "logitech",
        "Logitech",
        &[SoftwareMarker::File {
            root: "ProgramFiles",
            path: r"LogiDownloadAssistant\bin\logi_download_assistant.exe",
        }],
        vec![startup_entry(
            "logitech-download-assistant-startup",
            "Logitech Download Assistant at sign-in",
            "Logitech",
            Lm,
            "Logi Download Assistant",
        )],
        &[],
    )
}
