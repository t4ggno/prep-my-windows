use crate::models::ScheduledTaskDefinition;

pub fn scheduled_tasks() -> Vec<ScheduledTaskDefinition> {
    [
        (
            "compatibility-appraiser",
            "Microsoft Compatibility Appraiser",
            r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser",
        ),
        (
            "program-data-updater",
            "Program Data Updater",
            r"\Microsoft\Windows\Application Experience\ProgramDataUpdater",
        ),
        (
            "startup-app-task",
            "Startup App Task",
            r"\Microsoft\Windows\Application Experience\StartupAppTask",
        ),
        (
            "ceip-consolidator",
            "CEIP Consolidator",
            r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator",
        ),
        (
            "usb-ceip",
            "USB CEIP",
            r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip",
        ),
        (
            "disk-diagnostics",
            "Disk diagnostic collector",
            r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector",
        ),
        (
            "feedback-client",
            "Feedback client",
            r"\Microsoft\Windows\Feedback\Siuf\DmClient",
        ),
        (
            "feedback-scenario",
            "Feedback scenario downloader",
            r"\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload",
        ),
        (
            "maps-update",
            "Maps update",
            r"\Microsoft\Windows\Maps\MapsUpdateTask",
        ),
        (
            "wer-queue",
            "Error reporting queue",
            r"\Microsoft\Windows\Windows Error Reporting\QueueReporting",
        ),
        (
            "compatibility-appraiser-exp",
            "Microsoft Compatibility Appraiser Exp",
            r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser Exp",
        ),
        (
            "mare-backup",
            "Compatibility data backup",
            r"\Microsoft\Windows\Application Experience\MareBackup",
        ),
        (
            "kernel-ceip",
            "Kernel CEIP",
            r"\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask",
        ),
        (
            "autochk-proxy",
            "Autochk SQM proxy",
            r"\Microsoft\Windows\Autochk\Proxy",
        ),
        (
            "device-census",
            "Device information",
            r"\Microsoft\Windows\Device Information\Device",
        ),
        (
            "device-census-user",
            "Device user information",
            r"\Microsoft\Windows\Device Information\Device User",
        ),
        ("sqm-tasks", "SQM tasks", r"\Microsoft\Windows\PI\Sqm-Tasks"),
        (
            "network-info",
            "Network information collector",
            r"\Microsoft\Windows\NetTrace\GatherNetworkInfo",
        ),
        (
            "disk-footprint",
            "Disk footprint diagnostics",
            r"\Microsoft\Windows\DiskFootprint\Diagnostics",
        ),
        (
            "power-efficiency",
            "Power efficiency diagnostics",
            r"\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem",
        ),
        (
            "feature-usage-reporting",
            "Feature usage reporting",
            r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting",
        ),
        (
            "feature-usage-flushing",
            "Feature usage flushing",
            r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing",
        ),
        (
            "maps-toast",
            "Maps notifications",
            r"\Microsoft\Windows\Maps\MapsToastTask",
        ),
        (
            "family-safety-monitor",
            "Family Safety monitor",
            r"\Microsoft\Windows\Shell\FamilySafetyMonitor",
        ),
        (
            "family-safety-refresh",
            "Family Safety refresh",
            r"\Microsoft\Windows\Shell\FamilySafetyRefreshTask",
        ),
        (
            "retail-demo-cleanup",
            "Retail demo cleanup",
            r"\Microsoft\Windows\RetailDemo\CleanupOfflineContent",
        ),
        (
            "xbox-game-save",
            "Xbox Live game save",
            r"\Microsoft\XblGameSave\XblGameSaveTask",
        ),
        (
            "ai-settings-setup",
            "AI settings setup",
            r"\Microsoft\Windows\WindowsAI\Settings\InitialConfiguration",
        ),
        (
            "recall-setup",
            "Recall setup",
            r"\Microsoft\Windows\WindowsAI\Recall\InitialConfiguration",
        ),
        (
            "click-to-do-model-idle",
            "Click to Do model caching (idle)",
            r"\Microsoft\Windows\WindowsAI\ClickToDo\ModelCachingIdle",
        ),
        (
            "click-to-do-model-limit",
            "Click to Do model caching (limit)",
            r"\Microsoft\Windows\WindowsAI\ClickToDo\ModelCachingLimit",
        ),
        (
            "user-choice-protection-restore",
            "User Choice Protection restore",
            r"\Microsoft\Windows\AppxDeploymentClient\UCPD velocity",
        ),
        (
            "click-to-do-model-update",
            "Click to Do model updates",
            r"\Microsoft\Windows\WindowsAI\ClickToDo\ModelCachingUpdate",
        ),
    ]
    .into_iter()
    .map(|(id, name, path)| ScheduledTaskDefinition { id, name, path })
    .collect()
}
