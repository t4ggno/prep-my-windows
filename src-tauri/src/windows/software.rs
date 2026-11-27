use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

use crate::models::{SoftwareDefinition, SoftwareMarker};

use super::services;

pub fn is_installed(definition: &SoftwareDefinition) -> bool {
    definition.markers.iter().any(|marker| match *marker {
        SoftwareMarker::Service(name) => services::is_installed(name),
        SoftwareMarker::File { root, path } => {
            env::var_os(root).is_some_and(|root| PathBuf::from(root).join(path).is_file())
        }
    })
}

pub fn uninstalled_items(software: &[SoftwareDefinition]) -> HashSet<&'static str> {
    software
        .iter()
        .filter(|definition| !is_installed(definition))
        .flat_map(SoftwareDefinition::item_ids)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{is_installed, uninstalled_items};
    use crate::models::{SoftwareDefinition, SoftwareMarker};

    fn definition(markers: &'static [SoftwareMarker]) -> SoftwareDefinition {
        SoftwareDefinition {
            id: "test",
            name: "Test",
            markers,
            policies: Vec::new(),
            tasks: Vec::new(),
        }
    }

    #[test]
    fn detects_software_by_service_or_file() {
        assert!(is_installed(&definition(&[SoftwareMarker::Service(
            "EventLog"
        )])));
        assert!(is_installed(&definition(&[SoftwareMarker::File {
            root: "SystemRoot",
            path: r"System32\cmd.exe",
        }])));
        assert!(!is_installed(&definition(&[
            SoftwareMarker::Service("PrepMyWindowsMissingService"),
            SoftwareMarker::File {
                root: "SystemRoot",
                path: r"System32\prep-my-windows-missing.exe",
            },
        ])));
        assert!(uninstalled_items(&[definition(&[])]).is_empty());
    }
}
