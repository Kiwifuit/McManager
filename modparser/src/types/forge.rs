use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;
use thiserror::Error;

pub struct ModManifest {
    mod_loader: String,
    loader_version: ModVersionRange,
    license: String,
    issue_tracker: String,
    homepage_url: String,
}

pub struct Mod {
    id: String,
    version: ModVersion,
    display_name: String,
    authors: String,
    credits: String,
    description: String,
    update_url: String,
    homepage_url: String,
    logo: Path,
}

pub struct Dependency {
    id: String,
    version: ModVersion,
    mandatory: bool,
    version_range: ModVersionRange,
    ordering: String,
    side: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModVersion {
    major: Option<u32>,
    minor: Option<u32>,
    patch: Option<u32>,
}

impl FromStr for ModVersion {
    type Err = ModVersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = s
            .split('.')
            .map(|component| {
                component
                    .parse::<u32>()
                    .map_err(ModVersionParseError::Parse)
                // .map(|num| if num == 0 { None } else { Some(num) })
            })
            .collect::<Vec<Result<u32, ModVersionParseError>>>();

        Ok(ModVersion {
            major: a.first().and_then(|some_case| some_case.to_owned().ok()),
            minor: a.get(1).and_then(|some_case| some_case.to_owned().ok()),
            patch: a.get(2).and_then(|some_case| some_case.to_owned().ok()),
        })
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ModVersionParseError {
    #[error("error while parsing version: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

#[derive(Debug)]
pub struct ModVersionRange {
    from: ModVersion,
    to: Option<ModVersion>,
    mode: ModVersionRangeMode,
}

impl ModVersionRange {
    fn is_infinity(version: &Option<ModVersion>) -> bool {
        if let Some(max_version) = version {
            max_version.major.is_none()
                && max_version.minor.is_none()
    pub from: ModVersion,
    pub to: Option<ModVersion>,
    pub mode: ModVersionRangeMode,
}

impl<'de> Deserialize<'de> for ModVersionRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mod_data = String::deserialize(deserializer)?;
        mod_data.parse().map_err(serde::de::Error::custom)
    }
}

impl ModVersionRange {
    fn is_infinity(version: &Option<ModVersion>) -> bool {
        if let Some(max_version) = version {
            max_version.major.is_none()
                && max_version.minor.is_none()
                && max_version.patch.is_none()
        } else {
            true
        }
    }
}

impl FromStr for ModVersionRange {
    type Err = ModVersionRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let delimeter_loc = s.find(',');
        let closing_loc = s.find(']').or(s.find(')'));
        let mut is_strict_version = false;

        if delimeter_loc.is_none() && closing_loc.is_none() {
            // we assume that we will find a comma somewhere
            return Err(ModVersionRangeParseError::Malformed);
        } else if delimeter_loc.is_none() && closing_loc.is_some() {
            // we assume that we are given a 'strict version requirement'
            // e.g. STRICTLY 1.19.2 and no other version
            is_strict_version = true;
        } else if delimeter_loc.unwrap() == 1 {
            // we assume that we will find a minimum version at the beginning
            return Err(ModVersionRangeParseError::NoMinimum);
        }

        if closing_loc.is_none() {
            // we assume that we will find a closing `]` or `)` somewhere
            return Err(ModVersionRangeParseError::Unclosed);
        }

        let delimeter_loc = delimeter_loc.unwrap_or(closing_loc.unwrap());
        let strlen = s.len();

        let ver_min = s[1..delimeter_loc].parse::<ModVersion>();
        let ver_max = if is_strict_version {
            None
        } else {
            s[delimeter_loc + 1..strlen - 1].parse::<ModVersion>().ok()
        };
        let mode = if is_strict_version {
            ModVersionRangeMode::None
        } else if Self::is_infinity(&ver_max) {
            ModVersionRangeMode::GreaterThan
        } else {
            match s.chars().nth(closing_loc.unwrap()).unwrap() {
                ')' => ModVersionRangeMode::Between,
                ']' => ModVersionRangeMode::BetweenInclusive,
                _ => ModVersionRangeMode::None,
            }
        } else {
            ModVersionRangeMode::GreaterThan
        };

        Ok(ModVersionRange {
            from: ver_min?,
            to: ver_max,
            mode,
        })
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ModVersionRangeParseError {
    #[error("string is malformed")]
    Malformed,

    #[error("string does not supply a minimum version")]
    NoMinimum,

    #[error("expected `]` or `)` from string, found none")]
    Unclosed,

    #[error("unable to parse mod version: {0}")]
    Parse(#[from] ModVersionParseError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModVersionRangeMode {
    // "any version greater than or equal to a"
    GreaterThan,
    // // "any version lesser than or equal to a"
    // LesserThan,

    // TODO: Give this a better name
    // "any version between a and b, including a and b"
    BetweenInclusive,

    // "any version between a and b, including a but excluding b"
    Between,

    // specifically version a
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unzip::grab_meta_file;
    use toml::{from_str, toml};

    #[test]
    fn mod_version() {
        let version_raw = "1.2.3";

        let version = version_raw.parse::<ModVersion>();
        assert!(version.is_ok());

        let version_parsed = version.unwrap();

        assert_eq!(version_parsed.major, Some(1));
        assert_eq!(version_parsed.minor, Some(2));
        assert_eq!(version_parsed.patch, Some(3));
    }

    #[test]
    fn mod_version_major() {
        let version_raw = "1";

        let version = version_raw.parse::<ModVersion>();
        assert!(version.is_ok());

        let version_parsed = version.unwrap();

        assert_eq!(version_parsed.major, Some(1));
        assert_eq!(version_parsed.minor, None);
        assert_eq!(version_parsed.patch, None);
    }

    #[test]
    fn mod_version_major_minor() {
        let version_raw = "1.2";

        let version = version_raw.parse::<ModVersion>();
        assert!(version.is_ok());

        let version_parsed = version.unwrap();

        assert_eq!(version_parsed.major, Some(1));
        assert_eq!(version_parsed.minor, Some(2));
        assert_eq!(version_parsed.patch, None);
    }

        #[test]
    fn mod_version_major_minor_patch() {
        let version_raw = "1.2.3";

        let version = version_raw.parse::<ModVersion>();
        assert!(version.is_ok());

        let version_parsed = version.unwrap();

        assert_eq!(version_parsed.major, Some(1));
        assert_eq!(version_parsed.minor, Some(2));
        assert_eq!(version_parsed.patch, Some(3));
    }

    #[test]
    fn version_range_unclosed() {
        let version = "[1.2.3,";

        let mod_version = version.parse::<ModVersionRange>();
        assert!(mod_version.is_err());
        assert_eq!(
            mod_version.unwrap_err(),
            ModVersionRangeParseError::Unclosed
        )
    }

    #[test]
    fn version_range_malformed() {
        let version = "[1.2.3";

        let mod_version = version.parse::<ModVersionRange>();
        assert!(mod_version.is_err());
        assert_eq!(
            mod_version.unwrap_err(),
            ModVersionRangeParseError::Malformed
        )
    }

    #[test]
    fn version_range_no_minimum() {
        let version = "[,)";

        let mod_version = version.parse::<ModVersionRange>();
        assert!(mod_version.is_err());
        assert_eq!(
            mod_version.unwrap_err(),
            ModVersionRangeParseError::NoMinimum
        )
    }

    #[test]
    fn version_range_shouldnt_fail() {
        let version = "[1.2.3,4.5.6)";

        let mod_version = version.parse::<ModVersionRange>();
        assert!(mod_version.is_ok());

        let mod_version = mod_version.unwrap();

        assert_eq!(
            mod_version.from,
            ModVersion {
                major: Some(1),
                minor: Some(2),
                patch: Some(3)
            }
        );

        assert_eq!(
            mod_version.to,
            Some(ModVersion {
                major: Some(4),
                minor: Some(5),
                patch: Some(6)
            })
        );

        assert_eq!(mod_version.mode, ModVersionRangeMode::Between)
    }

    #[test]
    fn version_range_shouldnt_fail_inclusive() {
        let version = "[1.2.3,4.5.6]";

        let mod_version = version.parse::<ModVersionRange>();
        assert!(mod_version.is_ok());

        let mod_version = mod_version.unwrap();

        assert_eq!(
            mod_version.from,
            ModVersion {
                major: Some(1),
                minor: Some(2),
                patch: Some(3)
            }
        );

        assert_eq!(
            mod_version.to,
            Some(ModVersion {
                major: Some(4),
                minor: Some(5),
                patch: Some(6)
            })
        );

        assert_eq!(mod_version.mode, ModVersionRangeMode::BetweenInclusive)
    }

    #[test]
    fn version_range_shouldnt_fail_single_inclusive() {
        let version = "[1.2.3]";

        let mod_version = version.parse::<ModVersionRange>();
        assert!(mod_version.is_ok());

        let mod_version = mod_version.unwrap();

        assert_eq!(
            mod_version.from,
            ModVersion {
                major: Some(1),
                minor: Some(2),
                patch: Some(3)
            }
        );

        assert_eq!(mod_version.to, None);
        assert_eq!(mod_version.mode, ModVersionRangeMode::None)
    }

    #[test]
    fn version_range_shouldnt_fail_greater() {
        let version = "[1.2.3,)";

        let mod_version = version.parse::<ModVersionRange>();
        assert!(mod_version.is_ok());

        let mod_version = mod_version.unwrap();

        assert_eq!(
            mod_version.to,
            Some(ModVersion {
                major: None,
                minor: None,
                patch: None
            })
        );

        assert_eq!(mod_version.mode, ModVersionRangeMode::GreaterThan)
    }

    // modparser/samples/architectury-6.6.92-forge.jar
}
