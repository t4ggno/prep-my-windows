use super::choices;
use crate::models::{PolicyDefinition, RegistryHive, SupportRequirement};

const CATEGORY: &str = "Microsoft Edge";
const EDGE_POLICIES: &str = r"SOFTWARE\Policies\Microsoft\Edge";

pub(super) fn policies() -> Vec<PolicyDefinition> {
    use RegistryHive::LocalMachine as Lm;

    [
        (
            "edge-first-run",
            "Edge first-run experience",
            "HideFirstRunExperience",
            1,
            80,
        ),
        (
            "edge-default-browser-campaign",
            "Edge default-browser campaign",
            "DefaultBrowserSettingsCampaignEnabled",
            0,
            113,
        ),
        (
            "edge-pdf-handler-recommendations",
            "Edge PDF-handler recommendations",
            "ShowPDFDefaultRecommendationsEnabled",
            0,
            93,
        ),
        (
            "edge-personalization",
            "Edge personalization reporting",
            "PersonalizationReportingEnabled",
            0,
            0,
        ),
        (
            "edge-serp-telemetry",
            "Edge search result telemetry",
            "Edge3PSerpTelemetryEnabled",
            0,
            0,
        ),
        (
            "edge-feedback",
            "Edge feedback",
            "UserFeedbackAllowed",
            0,
            0,
        ),
        (
            "edge-search-suggestions",
            "Edge search suggestions",
            "SearchSuggestEnabled",
            0,
            0,
        ),
        (
            "edge-error-pages",
            "Edge web error service",
            "AlternateErrorPagesEnabled",
            0,
            0,
        ),
        (
            "edge-navigation-errors",
            "Edge navigation error service",
            "ResolveNavigationErrorsUseWebService",
            0,
            0,
        ),
        ("edge-sync", "Edge sync", "SyncDisabled", 1, 0),
        ("edge-signin", "Edge browser sign-in", "BrowserSignin", 0, 0),
        (
            "edge-shopping",
            "Edge shopping assistant",
            "EdgeShoppingAssistantEnabled",
            0,
            0,
        ),
        (
            "edge-recommendations",
            "Edge recommendations",
            "ShowRecommendationsEnabled",
            0,
            0,
        ),
        (
            "edge-spotlight",
            "Edge spotlight experiences",
            "SpotlightExperiencesAndRecommendationsEnabled",
            0,
            0,
        ),
        (
            "edge-promotions",
            "Edge promotional tabs",
            "PromotionalTabsEnabled",
            0,
            0,
        ),
        ("edge-sidebar", "Edge sidebar", "HubsSidebarEnabled", 0, 0),
        (
            "edge-startup-boost",
            "Edge startup boost",
            "StartupBoostEnabled",
            0,
            0,
        ),
        (
            "edge-background",
            "Edge background mode",
            "BackgroundModeEnabled",
            0,
            0,
        ),
        (
            "edge-new-tab-content",
            "Edge new tab content",
            "NewTabPageContentEnabled",
            0,
            0,
        ),
        (
            "edge-insider-promotion",
            "Edge Insider promotion",
            "MicrosoftEdgeInsiderPromotionEnabled",
            0,
            0,
        ),
        (
            "edge-copilot-context",
            "Edge Copilot page context",
            "CopilotPageContextEnabled",
            0,
            0,
        ),
        (
            "edge-assets",
            "Edge asset delivery",
            "EdgeAssetDeliveryServiceEnabled",
            0,
            0,
        ),
        (
            "edge-copilot-browsing-context",
            "Edge Copilot browsing context",
            "CopilotCDPPageContext",
            0,
            0,
        ),
        (
            "edge-work-copilot-context",
            "Edge work Copilot page context",
            "EdgeEntraCopilotPageContext",
            0,
            0,
        ),
        (
            "edge-copilot-icon",
            "Edge Copilot toolbar icon",
            "Microsoft365CopilotChatIconEnabled",
            0,
            0,
        ),
        (
            "edge-new-tab-copilot",
            "Edge new tab Copilot",
            "NewTabPageBingChatEnabled",
            0,
            0,
        ),
        (
            "edge-history-ai-search",
            "Edge AI history search",
            "EdgeHistoryAISearchEnabled",
            0,
            0,
        ),
        (
            "edge-compose",
            "Edge writing assistance",
            "ComposeInlineEnabled",
            0,
            0,
        ),
        (
            "edge-local-ai-model",
            "Edge local AI model",
            "GenAILocalFoundationalModelSettings",
            1,
            0,
        ),
        (
            "edge-top-sites",
            "Edge sponsored top sites",
            "NewTabPageHideDefaultTopSites",
            1,
            0,
        ),
        (
            "edge-tab-services",
            "Edge tab services",
            "TabServicesEnabled",
            0,
            0,
        ),
        (
            "edge-rewards",
            "Edge Microsoft Rewards",
            "ShowMicrosoftRewards",
            0,
            0,
        ),
        (
            "edge-wallet-donations",
            "Edge wallet donations",
            "WalletDonationEnabled",
            0,
            0,
        ),
        (
            "edge-acrobat",
            "Edge Acrobat subscription button",
            "ShowAcrobatSubscriptionButton",
            0,
            0,
        ),
        (
            "edge-collections",
            "Edge collections",
            "EdgeCollectionsEnabled",
            0,
            0,
        ),
        (
            "edge-follow",
            "Edge creator follow",
            "EdgeFollowEnabled",
            0,
            0,
        ),
        (
            "edge-mini-menu",
            "Edge text selection menu",
            "QuickSearchShowMiniMenu",
            0,
            0,
        ),
        ("edge-web-widget", "Edge bar", "WebWidgetAllowed", 0, 0),
        (
            "edge-default-browser",
            "Edge default browser check",
            "DefaultBrowserSettingEnabled",
            0,
            0,
        ),
        (
            "edge-do-not-track",
            "Edge Do Not Track",
            "ConfigureDoNotTrack",
            1,
            0,
        ),
    ]
    .into_iter()
    .map(|(id, name, value_name, value, minimum_edge_major)| {
        policy!(id, name, CATEGORY, Lm, EDGE_POLICIES, value_name, value)
            .with_support(SupportRequirement::edge(minimum_edge_major))
    })
    .chain([
        policy!(
            "edge-network-prediction",
            "Edge network prediction",
            CATEGORY,
            Lm,
            EDGE_POLICIES,
            "NetworkPredictionOptions",
            2
        )
        .with_choices(choices::EDGE_NETWORK_PREDICTION),
        policy!(
            "edge-diagnostic-data",
            "Edge diagnostic data",
            CATEGORY,
            Lm,
            EDGE_POLICIES,
            "DiagnosticData",
            0
        )
        .with_choices(choices::EDGE_DIAGNOSTIC_DATA),
        policy!(
            "edge-update-desktop-shortcut",
            "Edge desktop shortcut after updates",
            CATEGORY,
            Lm,
            r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
            "CreateDesktopShortcutDefault",
            0
        ),
    ])
    .collect()
}
