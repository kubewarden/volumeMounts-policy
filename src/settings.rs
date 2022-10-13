use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Settings {
    pub operator: Reject,
    pub volume_mount_names: HashSet<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)] // allow for all to end in `In`
pub enum Reject {
    #[default]
    AnyIn,
    AnyNotIn,
    AllAreUsed,
    NotAllAreUsed,
}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_with_no_settings() -> Result<(), ()> {
        let settings = serde_yaml::from_str::<Settings>("");
        assert!(
            settings.is_ok(),
            "settings parse should not fail if it is empty"
        );
        assert!(
            matches!(settings.as_ref().unwrap().operator, Reject::AnyIn),
            "operator should be 'anyIn' when not defined by the user"
        );
        Ok(())
    }

    #[test]
    fn test_policy_with_settings() -> Result<(), serde_yaml::Error> {
        let payload = "
operator: anyNotIn
volumeMountNames:
  - test1
";
        let settings = serde_yaml::from_str::<Settings>(payload);
        assert!(settings.is_ok());
        assert!(matches!(
            settings.as_ref().unwrap().operator,
            Reject::AnyNotIn
        ));
        assert!(settings
            .as_ref()
            .unwrap()
            .volume_mount_names
            .contains(&"test1".to_string()));
        Ok(())
    }
}
