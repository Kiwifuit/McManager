use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Deserialize)]
pub struct FabricMod {
    #[serde(rename = "schemaVersion")]
    _schema_version: u8,
    #[serde(rename = "entrypoints")]
    _entrypoints: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "accessWidener")]
    _access_widener: Option<String>,
    #[serde(rename = "jars")]
    _jars: Option<Vec<HashMap<String, String>>>,

    #[serde(rename = "id")]
    pub mod_id: String,
    pub icon: String,
    #[serde(rename = "version")]
    pub mod_version: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub contributors: Option<Vec<String>>,
    pub contact: Option<FabricModContact>,
    pub license: Option<String>,
    #[serde(rename = "depends")]
    pub dependencies: Option<HashMap<String, FabricDependencyVersion>>,
    pub recommends: Option<HashMap<String, FabricDependencyVersion>>,
    pub conflicts: Option<HashMap<String, FabricDependencyVersion>>,
    pub breaks: Option<HashMap<String, FabricDependencyVersion>>,
}

#[derive(Debug, Deserialize)]
pub struct FabricModContact {
    pub homepage: Option<String>,
    pub issues: Option<String>,
    pub sources: Option<String>,
    pub email: Option<String>,
    pub irc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum FabricDependencyVersionMode {
    Any,
    ExactMatch,
    SameMinor,
    SameMajor,
    GreaterThan,
    LesserThan,
    GreaterThanEqual,
    LesserThanEqual,
}

#[derive(Debug)]
pub struct FabricDependencyVersion {
    pub mode: FabricDependencyVersionMode,
    pub version: String,
}

impl<'de> Deserialize<'de> for FabricDependencyVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mod_data = String::deserialize(deserializer)?;
        mod_data.parse().map_err(serde::de::Error::custom)
    }
}

impl FabricDependencyVersion {
    fn check_equals(s: &str) -> bool {
        // For some reason, there is a chance for
        // a "*" to end up in here, so we gotta check
        // for out-of-bounds edge cases and immidiately
        // return false if we can't even begin to
        // check
        if s.len() < 2 {
            false
        } else {
            s.chars().nth(1).unwrap() == '='
        }
    }
}

impl FromStr for FabricDependencyVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::wildcard_in_or_patterns)]
        let mode = match s.chars().next().unwrap() {
            any_char if any_char.is_numeric() => FabricDependencyVersionMode::ExactMatch,
            '>' if Self::check_equals(s) => FabricDependencyVersionMode::GreaterThanEqual,
            '<' if Self::check_equals(s) => FabricDependencyVersionMode::LesserThanEqual,
            '>' => FabricDependencyVersionMode::GreaterThan,
            '<' => FabricDependencyVersionMode::LesserThan,
            '^' => FabricDependencyVersionMode::SameMajor,
            '~' => FabricDependencyVersionMode::SameMinor,
            '*' | _ => FabricDependencyVersionMode::Any,
        };

        let version_start = if Self::check_equals(s) { 2 } else { 1 };
        let version = s[version_start..].to_string();

        Ok(Self { mode, version })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unzip::grab_meta_file;
    use serde_json::from_str;
    use std::fs::read_dir;

    #[test]
    fn mod_manifest() {
        for file in read_dir("samples/fabric/").unwrap() {
            let file = file.unwrap();

            if file.file_type().unwrap().is_dir() {
                continue;
            }

            let mod_meta = from_str::<FabricMod>(
                grab_meta_file(file.path(), crate::unzip::ModLoader::Fabric)
                    .expect("expected meta file to be grabbed")
                    .raw
                    .as_str(),
            );

            assert!(mod_meta.is_ok());
        }
    }
}
