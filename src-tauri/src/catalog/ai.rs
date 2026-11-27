use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, SupportRequirement};

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::{CurrentUser as Cu, LocalMachine as Lm};

    vec![
        policy!(
            "search-box-suggestions",
            "Search box web suggestions",
            "Search & AI",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "DisableSearchBoxSuggestions",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "cloud-search",
            "Cloud search",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AllowCloudSearch",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "web-search",
            "Windows web search",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "DisableWebSearch",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "connected-search",
            "Connected search",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "ConnectedSearchUseWeb",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "cortana",
            "Cortana",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AllowCortana",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "recall-component",
            "Recall",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "AllowRecallEnablement",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "recall-snapshots",
            "Recall snapshots",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "DisableAIDataAnalysis",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "click-to-do",
            "Click to Do",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "DisableClickToDo",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "settings-agent",
            "Settings agent",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "DisableSettingsAgent",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "paint-cocreator",
            "Paint Cocreator",
            "Search & AI",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableCocreator",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "paint-image-creator",
            "Paint Image Creator",
            "Search & AI",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableImageCreator",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "paint-generative-fill",
            "Paint generative fill",
            "Search & AI",
            Lm,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableGenerativeFill",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "search-highlights",
            "Search highlights",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "EnableDynamicContentInWSB",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "cortana-consent",
            "Cortana consent",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "CortanaConsent",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "copilot-user",
            "Windows Copilot",
            "Search & AI",
            Cu,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
            "TurnOffWindowsCopilot",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "copilot-machine",
            "Windows Copilot for all users",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
            "TurnOffWindowsCopilot",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "copilot-taskbar-button",
            "Copilot taskbar button",
            "Search & AI",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCopilotButton",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "copilot-pin-prompt",
            "Copilot pin prompt",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableCopilotPinScreen",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "notepad-ai",
            "Notepad AI features",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\WindowsNotepad",
            "DisableAIFeatures",
            1
        )
        .with_choices(choices::OFF_WHEN_ONE),
        policy!(
            "ai-components-service",
            "AI components service autostart",
            "Search & AI",
            Lm,
            r"SYSTEM\CurrentControlSet\Services\WSAIFabricSvc",
            "Start",
            3
        )
        .with_choices(choices::SERVICE_START)
        .with_support(SupportRequirement::windows_build(26_100)),
        policy!(
            "search-location",
            "Search access to location",
            "Search & AI",
            Lm,
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AllowSearchToUseLocation",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "bing-search",
            "Bing in Windows Search",
            "Search & AI",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
            "BingSearchEnabled",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "search-microsoft-account",
            "Search Microsoft account content",
            "Search & AI",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\SearchSettings",
            "IsMSACloudSearchEnabled",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "search-work-account",
            "Search work or school account content",
            "Search & AI",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\SearchSettings",
            "IsAADCloudSearchEnabled",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
        policy!(
            "search-history",
            "Search history on this device",
            "Search & AI",
            Cu,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\SearchSettings",
            "IsDeviceSearchHistoryEnabled",
            0
        )
        .with_choices(choices::ON_WHEN_ONE),
    ]
}
