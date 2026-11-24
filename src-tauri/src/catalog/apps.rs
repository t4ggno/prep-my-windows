use crate::models::{PackageDefinition, ProcessRule};

pub fn packages() -> Vec<PackageDefinition> {
    [
        ("clipchamp", "Clipchamp", "Clipchamp.Clipchamp"),
        ("news", "Microsoft News", "Microsoft.BingNews"),
        ("weather", "Microsoft Weather", "Microsoft.BingWeather"),
        ("gaming-app", "Xbox app", "Microsoft.GamingApp"),
        ("get-help", "Get Help", "Microsoft.GetHelp"),
        ("get-started", "Get Started", "Microsoft.Getstarted"),
        (
            "office-hub",
            "Microsoft 365",
            "Microsoft.MicrosoftOfficeHub",
        ),
        (
            "solitaire",
            "Microsoft Solitaire Collection",
            "Microsoft.MicrosoftSolitaireCollection",
        ),
        (
            "mixed-reality",
            "Mixed Reality Portal",
            "Microsoft.MixedReality.Portal",
        ),
        ("people", "Microsoft People", "Microsoft.People"),
        (
            "power-automate",
            "Power Automate",
            "Microsoft.PowerAutomateDesktop",
        ),
        ("skype", "Skype", "Microsoft.SkypeApp"),
        ("todo", "Microsoft To Do", "Microsoft.Todos"),
        (
            "feedback-hub",
            "Feedback Hub",
            "Microsoft.WindowsFeedbackHub",
        ),
        ("maps", "Windows Maps", "Microsoft.WindowsMaps"),
        ("xbox-tcui", "Xbox TCUI", "Microsoft.Xbox.TCUI"),
        ("xbox-app", "Xbox Console Companion", "Microsoft.XboxApp"),
        (
            "xbox-overlay",
            "Xbox Game Overlay",
            "Microsoft.XboxGameOverlay",
        ),
        (
            "xbox-gamebar",
            "Xbox Game Bar",
            "Microsoft.XboxGamingOverlay",
        ),
        (
            "xbox-identity",
            "Xbox Identity Provider",
            "Microsoft.XboxIdentityProvider",
        ),
        (
            "xbox-speech",
            "Xbox Speech to Text",
            "Microsoft.XboxSpeechToTextOverlay",
        ),
        ("phone-link", "Phone Link", "Microsoft.YourPhone"),
        (
            "media-player",
            "Windows Media Player",
            "Microsoft.ZuneMusic",
        ),
        ("movies-tv", "Movies & TV", "Microsoft.ZuneVideo"),
        (
            "family",
            "Microsoft Family",
            "MicrosoftCorporationII.MicrosoftFamily",
        ),
        (
            "quick-assist",
            "Quick Assist",
            "MicrosoftCorporationII.QuickAssist",
        ),
        ("teams-classic", "Microsoft Teams classic", "MicrosoftTeams"),
        ("teams", "Microsoft Teams", "MSTeams"),
        (
            "outlook",
            "Outlook for Windows",
            "Microsoft.OutlookForWindows",
        ),
        ("dev-home", "Dev Home", "Microsoft.Windows.DevHome"),
        ("cortana-app", "Cortana", "Microsoft.549981C3F5F10"),
        (
            "web-experience",
            "Windows Web Experience Pack",
            "MicrosoftWindows.Client.WebExperience",
        ),
        ("copilot-app", "Microsoft Copilot", "Microsoft.Copilot"),
        ("3d-builder", "3D Builder", "Microsoft.3DBuilder"),
        ("3d-viewer", "3D Viewer", "Microsoft.Microsoft3DViewer"),
        ("print-3d", "Print 3D", "Microsoft.Print3D"),
        ("bing-finance", "Microsoft Money", "Microsoft.BingFinance"),
        ("bing-food", "Food & Drink", "Microsoft.BingFoodAndDrink"),
        (
            "bing-health",
            "Health & Fitness",
            "Microsoft.BingHealthAndFitness",
        ),
        ("bing-sports", "Microsoft Sports", "Microsoft.BingSports"),
        ("bing-translator", "Translator", "Microsoft.BingTranslator"),
        ("bing-travel", "Travel", "Microsoft.BingTravel"),
        ("bing-search", "Bing Search", "Microsoft.BingSearch"),
        ("ai-hub", "AI Hub", "Microsoft.Windows.AIHub"),
        ("pc-manager", "PC Manager", "Microsoft.PCManager"),
        (
            "pc-manager-store",
            "Microsoft PC Manager",
            "Microsoft.MicrosoftPCManager",
        ),
        ("messaging", "Messaging", "Microsoft.Messaging"),
        ("journal", "Microsoft Journal", "Microsoft.MicrosoftJournal"),
        (
            "power-bi",
            "Power BI",
            "Microsoft.MicrosoftPowerBIForWindows",
        ),
        (
            "network-speed-test",
            "Network Speed Test",
            "Microsoft.NetworkSpeedTest",
        ),
        (
            "onenote",
            "OneNote for Windows 10",
            "Microsoft.Office.OneNote",
        ),
        ("sway", "Sway", "Microsoft.Office.Sway"),
        ("mobile-plans", "Mobile Plans", "Microsoft.OneConnect"),
        (
            "mail-calendar",
            "Mail and Calendar",
            "Microsoft.windowscommunicationsapps",
        ),
        (
            "m365-companions",
            "Microsoft 365 Companions",
            "Microsoft.M365Companions",
        ),
        (
            "windows-365",
            "Windows 365",
            "MicrosoftCorporationII.Windows365",
        ),
        ("whiteboard", "Microsoft Whiteboard", "Microsoft.Whiteboard"),
        (
            "cross-device",
            "Cross Device Experience Host",
            "MicrosoftWindows.CrossDevice",
        ),
        ("disney-plus", "Disney+", "Disney.37853FC22B2CE"),
        ("netflix", "Netflix", "4DF9E0F8.Netflix"),
        ("prime-video", "Prime Video", "AmazonVideo.PrimeVideo"),
        ("hulu", "Hulu", "HULULLC.HULUPLUS"),
        ("amazon", "Amazon", "Amazon.com.Amazon"),
        ("facebook", "Facebook", "Facebook.Facebook"),
        ("instagram", "Instagram", "Facebook.Instagram"),
        ("tiktok", "TikTok", "BytedancePte.Ltd.TikTok"),
        ("candy-crush", "Candy Crush Saga", "king.com.CandyCrushSaga"),
        (
            "candy-crush-soda",
            "Candy Crush Soda Saga",
            "king.com.CandyCrushSodaSaga",
        ),
        (
            "bubble-witch",
            "Bubble Witch 3 Saga",
            "king.com.BubbleWitch3Saga",
        ),
        ("royal-revolt", "Royal Revolt", "flaregamesGmbH.RoyalRevolt"),
        (
            "photoshop-express",
            "Photoshop Express",
            "AdobeSystemsIncorporated.AdobePhotoshopExpress",
        ),
    ]
    .into_iter()
    .map(|(id, name, pattern)| PackageDefinition { id, name, pattern })
    .collect()
}

pub(super) fn default_process_rules() -> Vec<ProcessRule> {
    [
        ("onedrive", "Microsoft OneDrive", "OneDrive.exe"),
        ("teams", "Microsoft Teams", "ms-teams.exe"),
        ("teams-classic", "Microsoft Teams classic", "Teams.exe"),
        ("widgets", "Windows Widgets", "Widgets.exe"),
        (
            "widget-service",
            "Windows Widget Service",
            "WidgetService.exe",
        ),
        ("copilot", "Microsoft Copilot", "Copilot.exe"),
        ("phone-link", "Phone Link", "PhoneExperienceHost.exe"),
        ("game-bar", "Xbox Game Bar", "GameBar.exe"),
        (
            "game-bar-server",
            "Xbox Game Bar server",
            "GameBarFTServer.exe",
        ),
        ("discord", "Discord", "Discord.exe"),
        ("spotify", "Spotify", "Spotify.exe"),
        ("steam", "Steam", "steam.exe"),
        ("epic", "Epic Games Launcher", "EpicGamesLauncher.exe"),
        ("battle-net", "Battle.net", "Battle.net.exe"),
        (
            "creative-cloud",
            "Adobe Creative Cloud",
            "Creative Cloud.exe",
        ),
        (
            "adobe-service",
            "Adobe Desktop Service",
            "Adobe Desktop Service.exe",
        ),
        ("ccx-process", "Adobe CCX Process", "CCXProcess.exe"),
        (
            "adobe-collab",
            "Adobe Collaboration Synchronizer",
            "AdobeCollabSync.exe",
        ),
        ("dropbox", "Dropbox", "Dropbox.exe"),
        ("google-drive", "Google Drive", "GoogleDriveFS.exe"),
        ("slack", "Slack", "slack.exe"),
        ("notion", "Notion", "Notion.exe"),
        ("whatsapp", "WhatsApp", "WhatsApp.exe"),
        ("zoom", "Zoom", "Zoom.exe"),
    ]
    .into_iter()
    .map(|(id, name, executable_name)| ProcessRule {
        id: format!("built-in-{id}"),
        name: name.to_owned(),
        executable_name: executable_name.to_owned(),
        executable_path: None,
        enabled: true,
        built_in: true,
    })
    .collect()
}
