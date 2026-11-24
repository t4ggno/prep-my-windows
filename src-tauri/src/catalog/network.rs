use crate::models::NetworkDefinition;

pub fn network_blocks() -> Vec<NetworkDefinition> {
    [
        ("events", "v10.events.data.microsoft.com"),
        ("events-config", "v10c.events.data.microsoft.com"),
        ("events-config-us", "us-v10c.events.data.microsoft.com"),
        ("events-config-eu", "eu-v10c.events.data.microsoft.com"),
        ("vortex-windows", "v10.vortex-win.data.microsoft.com"),
        ("self-events", "self.events.data.microsoft.com"),
        ("functional-events", "functional.events.data.microsoft.com"),
        ("browser-events", "browser.events.data.msn.com"),
        ("watson-telemetry", "watson.telemetry.microsoft.com"),
        ("watson-events", "watson.events.data.microsoft.com"),
        ("watson-user-events", "umwatsonc.events.data.microsoft.com"),
        ("watson-current", "watsonc.events.data.microsoft.com"),
        ("watson-current-us", "us-watsonc.events.data.microsoft.com"),
        ("watson-current-eu", "eu-watsonc.events.data.microsoft.com"),
        ("watson-kernel", "kmwatsonc.events.data.microsoft.com"),
        ("oca-telemetry", "oca.telemetry.microsoft.com"),
        ("oca", "oca.microsoft.com"),
        ("telecommand", "telecommand.telemetry.microsoft.com"),
        ("telecommand-service", "www.telecommandsvc.microsoft.com"),
        ("settings-windows", "settings-win.data.microsoft.com"),
        ("settings", "settings.data.microsoft.com"),
        ("wer-blob-ceus-1", "ceuswatcab01.blob.core.windows.net"),
        ("wer-blob-ceus-2", "ceuswatcab02.blob.core.windows.net"),
        ("wer-blob-eaus-1", "eaus2watcab01.blob.core.windows.net"),
        ("wer-blob-eaus-2", "eaus2watcab02.blob.core.windows.net"),
        ("wer-blob-weus-1", "weus2watcab01.blob.core.windows.net"),
        ("wer-blob-weus-2", "weus2watcab02.blob.core.windows.net"),
    ]
    .into_iter()
    .map(|(id, domain)| NetworkDefinition { id, domain })
    .collect()
}
