use crate::models::{PolicyChoice, RegistryValue};

const fn choice(id: &'static str, label: &'static str, value: RegistryValue) -> PolicyChoice {
    PolicyChoice { id, label, value }
}

pub(super) const ON_WHEN_ONE: &[PolicyChoice] = &[
    choice("on", "On", RegistryValue::Dword(1)),
    choice("off", "Off", RegistryValue::Dword(0)),
];

pub(super) const OFF_WHEN_ONE: &[PolicyChoice] = &[
    choice("on", "On", RegistryValue::Dword(0)),
    choice("off", "Off", RegistryValue::Dword(1)),
];

pub(super) const SHOWN_WHEN_ZERO: &[PolicyChoice] = &[
    choice("shown", "Shown", RegistryValue::Dword(0)),
    choice("hidden", "Hidden", RegistryValue::Dword(1)),
];

pub(super) const DIAGNOSTIC_DATA: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Dword(0)),
    choice("required", "Required", RegistryValue::Dword(1)),
    choice("enhanced", "Enhanced", RegistryValue::Dword(2)),
    choice("optional", "Optional", RegistryValue::Dword(3)),
];

pub(super) const EDGE_DIAGNOSTIC_DATA: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Dword(0)),
    choice("required", "Required", RegistryValue::Dword(1)),
    choice("optional", "Optional", RegistryValue::Dword(2)),
];

pub(super) const SERVICE_START: &[PolicyChoice] = &[
    choice("automatic", "Automatic", RegistryValue::Dword(2)),
    choice("manual", "Manual", RegistryValue::Dword(3)),
    choice("disabled", "Disabled", RegistryValue::Dword(4)),
];

pub(super) const DRIVER_START: &[PolicyChoice] = &[
    choice("on", "On", RegistryValue::Dword(1)),
    choice("disabled", "Disabled", RegistryValue::Dword(4)),
];

pub(super) const DEFENDER_SAMPLES: &[PolicyChoice] = &[
    choice("never", "Never send", RegistryValue::Dword(2)),
    choice("ask", "Always ask", RegistryValue::Dword(0)),
    choice("safe", "Safe samples", RegistryValue::Dword(1)),
    choice("all", "All samples", RegistryValue::Dword(3)),
];

pub(super) const UPDATE_PEERS: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Dword(0)),
    choice("local", "Local network", RegistryValue::Dword(1)),
    choice("internet", "Local and internet", RegistryValue::Dword(3)),
];

pub(super) const ADMIN_ELEVATION: &[PolicyChoice] = &[
    choice("never", "Never ask", RegistryValue::Dword(0)),
    choice(
        "non-windows",
        "Ask for non-Windows apps",
        RegistryValue::Dword(5),
    ),
    choice("always", "Always ask", RegistryValue::Dword(2)),
    choice(
        "always-without-dimming",
        "Always ask without dimming",
        RegistryValue::Dword(4),
    ),
    choice(
        "credentials",
        "Ask for credentials",
        RegistryValue::Dword(1),
    ),
    choice(
        "credentials-without-dimming",
        "Ask for credentials without dimming",
        RegistryValue::Dword(3),
    ),
];

pub(super) const LOCATION_POLICY: &[PolicyChoice] = &[
    choice("allowed", "Allowed", RegistryValue::Absent),
    choice("blocked", "Blocked", RegistryValue::Dword(1)),
];

pub(super) const CONSENT: &[PolicyChoice] = &[
    choice("allow", "Allow", RegistryValue::Text("Allow")),
    choice("deny", "Deny", RegistryValue::Text("Deny")),
];

pub(super) const APP_ACCESS_DENIED: &[PolicyChoice] = &[
    choice("deny", "Deny", RegistryValue::Dword(2)),
    choice("user", "User decides", RegistryValue::Dword(0)),
    choice("allow", "Force allow", RegistryValue::Dword(1)),
];

pub(super) const APP_ACCESS_USER: &[PolicyChoice] = &[
    choice("user", "User decides", RegistryValue::Absent),
    choice("deny", "Deny", RegistryValue::Dword(2)),
];

pub(super) const EDGE_NETWORK_PREDICTION: &[PolicyChoice] = &[
    choice("never", "Never", RegistryValue::Dword(2)),
    choice("always", "Always", RegistryValue::Dword(0)),
];

pub(super) const EDGE_SIGNIN: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Dword(0)),
    choice("allowed", "Allowed", RegistryValue::Dword(1)),
    choice("required", "Required", RegistryValue::Dword(2)),
];

pub(super) const EDGE_FIRST_RUN_IMPORT: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Dword(4)),
    choice(
        "default-browser",
        "From default browser",
        RegistryValue::Dword(0),
    ),
    choice("chrome", "From Google Chrome", RegistryValue::Dword(2)),
];

pub(super) const TASKBAR_SEARCH: &[PolicyChoice] = &[
    choice("icon", "Icon", RegistryValue::Dword(1)),
    choice("hidden", "Hidden", RegistryValue::Dword(0)),
    choice("box", "Search box", RegistryValue::Dword(2)),
    choice("label", "Icon and label", RegistryValue::Dword(3)),
];

pub(super) const GPU_SCHEDULING: &[PolicyChoice] = &[
    choice("on", "On", RegistryValue::Dword(2)),
    choice("off", "Off", RegistryValue::Dword(1)),
];

pub(super) const EXPLORER_START_PAGE: &[PolicyChoice] = &[
    choice("this-pc", "This PC", RegistryValue::Dword(1)),
    choice("home", "Home", RegistryValue::Dword(2)),
    choice("downloads", "Downloads", RegistryValue::Dword(3)),
];

pub(super) const HIDDEN_FILES: &[PolicyChoice] = &[
    choice("shown", "Shown", RegistryValue::Dword(1)),
    choice("hidden", "Hidden", RegistryValue::Dword(2)),
];

pub(super) const TASKBAR_ALIGNMENT: &[PolicyChoice] = &[
    choice("left", "Left", RegistryValue::Dword(0)),
    choice("center", "Center", RegistryValue::Dword(1)),
];

pub(super) const EDGE_TRACKING_PREVENTION: &[PolicyChoice] = &[
    choice("strict", "Strict", RegistryValue::Dword(3)),
    choice("balanced", "Balanced", RegistryValue::Dword(2)),
    choice("basic", "Basic", RegistryValue::Dword(1)),
    choice("off", "Off", RegistryValue::Dword(0)),
];

pub(super) const EDGE_EXPERIMENTATION: &[PolicyChoice] = &[
    choice(
        "configurations",
        "Configurations only",
        RegistryValue::Dword(1),
    ),
    choice("off", "Off", RegistryValue::Dword(0)),
    choice(
        "full",
        "Configurations and experiments",
        RegistryValue::Dword(2),
    ),
];

pub(super) const AUTORUN: &[PolicyChoice] = &[
    choice("off", "Off on all drives", RegistryValue::Dword(255)),
    choice("default", "Windows default", RegistryValue::Absent),
];

pub(super) const SETTINGS_SYNC: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Dword(2)),
    choice("user", "User decides", RegistryValue::Absent),
];

pub(super) const FEEDBACK_FREQUENCY: &[PolicyChoice] = &[
    choice("never", "Never", RegistryValue::Dword(0)),
    choice("automatic", "Automatic", RegistryValue::Absent),
];

pub(super) const STARTUP_DELAY: &[PolicyChoice] = &[
    choice("off", "No delay", RegistryValue::Dword(0)),
    choice("default", "Windows default", RegistryValue::Absent),
];

pub(super) const MULTIMEDIA_RESERVATION: &[PolicyChoice] = &[
    choice("ten", "10%", RegistryValue::Dword(10)),
    choice("twenty", "20%", RegistryValue::Dword(20)),
];

pub(super) const POINTER_PRECISION: &[PolicyChoice] = &[
    choice("on", "On", RegistryValue::Text("1")),
    choice("off", "Off", RegistryValue::Text("0")),
];

pub(super) const POINTER_THRESHOLD: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Text("0")),
    choice("default", "Windows default", RegistryValue::Text("6")),
];

pub(super) const POINTER_HIGH_THRESHOLD: &[PolicyChoice] = &[
    choice("off", "Off", RegistryValue::Text("0")),
    choice("default", "Windows default", RegistryValue::Text("10")),
];

pub(super) const SETTINGS_HOME: &[PolicyChoice] = &[
    choice("hidden", "Hidden", RegistryValue::Text("hide:home")),
    choice("shown", "Shown", RegistryValue::Absent),
];

pub(super) const FULL_CONTEXT_MENU: &[PolicyChoice] =
    &[choice("on", "On", RegistryValue::Text(""))];
