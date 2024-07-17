use std::collections::HashMap;
use std::path::Path;

use super::ModpackProviderMetadata;
use serde::{Deserialize, Deserializer};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpack {
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<ModrinthModpackFiles>,
    #[serde(deserialize_with = "deserialize_deps")]
    pub dependencies: Vec<ModrinthModpackDependency>,
}

impl ModpackProviderMetadata for ModrinthModpack {
    fn overrides_dir(&self) -> &str {
        "overrides"
    }

    fn modpack_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpackFiles {
    pub path: Box<Path>,
    pub hashes: ModpackFileHashes,
    pub env: Option<ModpackEnv>,
    pub downloads: Vec<String>,
    pub file_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct ModpackFileHashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Deserialize)]
pub struct ModpackEnv {
    pub server: String,
    pub client: String,
}

#[derive(Debug, Deserialize)]
pub struct ModrinthModpackDependency {
    pub dependency: String,
    pub version: String,
}

fn deserialize_deps<'de, D>(deserializer: D) -> Result<Vec<ModrinthModpackDependency>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_map: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    let deps = raw_map
        .into_iter()
        .map(|(dependency, version)| ModrinthModpackDependency {
            dependency,
            version,
        })
        .collect();

    Ok(deps)
}
