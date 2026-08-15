use std::io::ErrorKind;

use winreg::RegKey;
use winreg::RegValue;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE, KEY_WOW64_64KEY, KEY_WRITE,
};
use winreg::types::FromRegValue;

use crate::models::{PolicyDefinition, RegistryHive, RegistryValue};

fn root(hive: RegistryHive) -> RegKey {
    match hive {
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
    }
}

pub fn read_policy(definition: &PolicyDefinition) -> Result<Option<String>, String> {
    let key = match root(definition.hive)
        .open_subkey_with_flags(definition.path, KEY_READ | KEY_WOW64_64KEY)
    {
        Ok(key) => key,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("Could not read {}: {error}", definition.name)),
    };

    let value = match key.get_raw_value(definition.value_name) {
        Ok(value) => value,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("Could not read {}: {error}", definition.name)),
    };
    Ok(Some(format_current_value(&value, &definition.desired)))
}

fn format_current_value(value: &RegValue<'_>, desired: &RegistryValue) -> String {
    let decoded = match desired {
        RegistryValue::Dword(_) => u32::from_reg_value(value).map(|value| value.to_string()),
        RegistryValue::Text(_) => String::from_reg_value(value),
        RegistryValue::Absent => return "Set".to_owned(),
    };
    decoded.unwrap_or_else(|_| format!("Unexpected {:?} value", value.vtype))
}

pub fn enforce_policy(definition: &PolicyDefinition) -> Result<bool, String> {
    if definition
        .desired
        .is_satisfied_by(read_policy(definition)?.as_deref())
    {
        return Ok(false);
    }

    let root = root(definition.hive);
    if definition.desired == RegistryValue::Absent {
        let key =
            match root.open_subkey_with_flags(definition.path, KEY_SET_VALUE | KEY_WOW64_64KEY) {
                Ok(key) => key,
                Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
                Err(error) => return Err(format!("Could not open {}: {error}", definition.name)),
            };
        return match key.delete_value(definition.value_name) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
            Err(error) => Err(format!("Could not enforce {}: {error}", definition.name)),
        };
    }

    let (key, _) = root
        .create_subkey_with_flags(definition.path, KEY_WRITE | KEY_SET_VALUE | KEY_WOW64_64KEY)
        .map_err(|error| format!("Could not open {}: {error}", definition.name))?;

    match &definition.desired {
        RegistryValue::Dword(value) => key.set_value(definition.value_name, value),
        RegistryValue::Text(value) => key.set_value(definition.value_name, value),
        RegistryValue::Absent => unreachable!(),
    }
    .map_err(|error| format!("Could not enforce {}: {error}", definition.name))?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use winreg::types::ToRegValue;

    use crate::models::RegistryValue;

    use super::format_current_value;

    #[test]
    fn formats_registry_values() {
        assert_eq!(RegistryValue::Dword(2).label(), "2");
        assert_eq!(RegistryValue::Text("Deny").label(), "Deny");
        assert_eq!(RegistryValue::Absent.label(), "Not set");
    }

    #[test]
    fn compares_registry_values() {
        assert!(RegistryValue::Dword(1).is_satisfied_by(Some("1")));
        assert!(RegistryValue::Text("Allow").is_satisfied_by(Some("Allow")));
        assert!(RegistryValue::Absent.is_satisfied_by(None));
        assert!(!RegistryValue::Absent.is_satisfied_by(Some("Set")));
    }

    #[test]
    fn reports_mismatched_registry_types_as_noncompliant() {
        let text = "1".to_reg_value();
        let current = format_current_value(&text, &RegistryValue::Dword(1));

        assert!(current.starts_with("Unexpected"));
        assert!(!RegistryValue::Dword(1).is_satisfied_by(Some(&current)));
    }
}
