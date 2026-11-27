use super::bundle;
use crate::catalog::choices;
use crate::models::RegistryHive::LocalMachine as Lm;
use crate::models::{SoftwareDefinition, SoftwareMarker};

const CHROME: &str = "Google Chrome";
const CHROME_POLICIES: &str = r"SOFTWARE\Policies\Google\Chrome";
const CHROME_EXECUTABLE: &str = r"Google\Chrome\Application\chrome.exe";
const FIREFOX: &str = "Mozilla Firefox";
const FIREFOX_POLICIES: &str = r"SOFTWARE\Policies\Mozilla\Firefox";
const FIREFOX_MESSAGING: &str = r"SOFTWARE\Policies\Mozilla\Firefox\UserMessaging";
const FIREFOX_EXECUTABLE: &str = r"Mozilla Firefox\firefox.exe";
const BRAVE: &str = "Brave";
const BRAVE_POLICIES: &str = r"SOFTWARE\Policies\BraveSoftware\Brave";
const BRAVE_EXECUTABLE: &str = r"BraveSoftware\Brave-Browser\Application\brave.exe";

pub(super) fn chrome() -> SoftwareDefinition {
    bundle(
        "google-chrome",
        CHROME,
        &[
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: CHROME_EXECUTABLE,
            },
            SoftwareMarker::File {
                root: "ProgramFiles(x86)",
                path: CHROME_EXECUTABLE,
            },
            SoftwareMarker::File {
                root: "LOCALAPPDATA",
                path: CHROME_EXECUTABLE,
            },
        ],
        vec![
            policy!(
                "chrome-background-apps",
                "Chrome background apps after closing",
                CHROME,
                Lm,
                CHROME_POLICIES,
                "BackgroundModeEnabled",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "chrome-memory-saver",
                "Chrome Memory Saver",
                CHROME,
                Lm,
                CHROME_POLICIES,
                "HighEfficiencyModeEnabled",
                1
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "chrome-metrics",
                "Chrome usage and crash reports",
                CHROME,
                Lm,
                CHROME_POLICIES,
                "MetricsReportingEnabled",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "chrome-default-browser-prompt",
                "Chrome default browser prompt",
                CHROME,
                Lm,
                CHROME_POLICIES,
                "DefaultBrowserSettingEnabled",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "chrome-shopping",
                "Chrome shopping features",
                CHROME,
                Lm,
                CHROME_POLICIES,
                "ShoppingListEnabled",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "chrome-feedback",
                "Chrome feedback reports",
                CHROME,
                Lm,
                CHROME_POLICIES,
                "UserFeedbackAllowed",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
        ],
        &[
            (
                "chrome-experience-helper",
                "Google platform experience helper",
                r"\GoogleUserPEH\RunPlatformExperienceHelper_Daily",
            ),
            (
                "chrome-experience-helper-metrics",
                "Google platform experience helper metrics",
                r"\GoogleUserPEH\RunPlatformExperienceHelper_Metrics",
            ),
        ],
    )
}

pub(super) fn firefox() -> SoftwareDefinition {
    bundle(
        "mozilla-firefox",
        FIREFOX,
        &[
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: FIREFOX_EXECUTABLE,
            },
            SoftwareMarker::File {
                root: "ProgramFiles(x86)",
                path: FIREFOX_EXECUTABLE,
            },
        ],
        vec![
            policy!(
                "firefox-telemetry",
                "Firefox telemetry",
                FIREFOX,
                Lm,
                FIREFOX_POLICIES,
                "DisableTelemetry",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "firefox-studies",
                "Firefox studies",
                FIREFOX,
                Lm,
                FIREFOX_POLICIES,
                "DisableFirefoxStudies",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "firefox-default-browser-agent",
                "Firefox default browser agent",
                FIREFOX,
                Lm,
                FIREFOX_POLICIES,
                "DisableDefaultBrowserAgent",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "firefox-default-browser-prompt",
                "Firefox default browser prompt",
                FIREFOX,
                Lm,
                FIREFOX_POLICIES,
                "DontCheckDefaultBrowser",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "firefox-feature-recommendations",
                "Firefox feature recommendations",
                FIREFOX,
                Lm,
                FIREFOX_MESSAGING,
                "FeatureRecommendations",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "firefox-extension-recommendations",
                "Firefox extension recommendations",
                FIREFOX,
                Lm,
                FIREFOX_MESSAGING,
                "ExtensionRecommendations",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
        ],
        &[],
    )
}

pub(super) fn brave() -> SoftwareDefinition {
    bundle(
        "brave",
        BRAVE,
        &[
            SoftwareMarker::File {
                root: "ProgramFiles",
                path: BRAVE_EXECUTABLE,
            },
            SoftwareMarker::File {
                root: "LOCALAPPDATA",
                path: BRAVE_EXECUTABLE,
            },
        ],
        vec![
            policy!(
                "brave-rewards",
                "Brave Rewards",
                BRAVE,
                Lm,
                BRAVE_POLICIES,
                "BraveRewardsDisabled",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "brave-wallet",
                "Brave Wallet",
                BRAVE,
                Lm,
                BRAVE_POLICIES,
                "BraveWalletDisabled",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "brave-vpn",
                "Brave VPN",
                BRAVE,
                Lm,
                BRAVE_POLICIES,
                "BraveVPNDisabled",
                1
            )
            .with_choices(choices::OFF_WHEN_ONE),
            policy!(
                "brave-leo",
                "Brave Leo AI assistant",
                BRAVE,
                Lm,
                BRAVE_POLICIES,
                "BraveAIChatEnabled",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
            policy!(
                "brave-background-apps",
                "Brave background apps after closing",
                BRAVE,
                Lm,
                BRAVE_POLICIES,
                "BackgroundModeEnabled",
                0
            )
            .with_choices(choices::ON_WHEN_ONE),
        ],
        &[],
    )
}
