use std::fs;
use std::path::PathBuf;

use crate::models::NetworkDefinition;

const START_MARKER: &str = "# Prep My Windows";
const END_MARKER: &str = "# End Prep My Windows";

fn hosts_path() -> PathBuf {
    PathBuf::from(std::env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into()))
        .join("System32")
        .join("drivers")
        .join("etc")
        .join("hosts")
}

fn without_managed_block(content: &str) -> String {
    let mut output = Vec::new();
    let mut inside = false;

    for line in content.lines() {
        if line.trim() == START_MARKER {
            inside = true;
            continue;
        }
        if line.trim() == END_MARKER {
            inside = false;
            continue;
        }
        if !inside {
            output.push(line);
        }
    }

    output.join("\r\n").trim_end().to_owned()
}

pub fn enforce(definitions: &[NetworkDefinition]) -> Result<bool, String> {
    let path = hosts_path();
    let current = fs::read_to_string(&path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    let base = without_managed_block(&current);
    let mut wanted = base;

    if !definitions.is_empty() {
        if !wanted.is_empty() {
            wanted.push_str("\r\n\r\n");
        }
        wanted.push_str(START_MARKER);
        wanted.push_str("\r\n");
        for definition in definitions {
            wanted.push_str("0.0.0.0 ");
            wanted.push_str(definition.domain);
            wanted.push_str("\r\n");
        }
        wanted.push_str(END_MARKER);
    }
    wanted.push_str("\r\n");

    if current.replace('\n', "\r\n").replace("\r\r\n", "\r\n") == wanted {
        return Ok(false);
    }

    fs::write(&path, wanted)
        .map_err(|error| format!("Could not write {}: {error}", path.display()))?;
    Ok(true)
}

pub fn statuses(definitions: &[NetworkDefinition]) -> Result<Vec<bool>, String> {
    let path = hosts_path();
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?
        .to_ascii_lowercase();

    Ok(definitions
        .iter()
        .map(|definition| {
            content
                .lines()
                .any(|line| line.split_whitespace().nth(1) == Some(definition.domain))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::without_managed_block;

    #[test]
    fn replaces_only_managed_hosts_block() {
        let content = "127.0.0.1 local\n# Prep My Windows\n0.0.0.0 example.com\n# End Prep My Windows\n10.0.0.2 private";
        assert_eq!(
            without_managed_block(content),
            "127.0.0.1 local\r\n10.0.0.2 private"
        );
    }
}
