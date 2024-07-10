use super::ModpackProviderMetadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForgeModpack {
    pub minecraft: ModpackLoaderMeta,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<ModpackFiles>,
    overrides: String,
}

impl ModpackProviderMetadata for ForgeModpack {
    fn overrides_dir(&self) -> &str {
        &self.overrides
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModpackLoaderMeta {
    pub version: String,
    pub mod_loaders: Vec<ModpackLoaderVersion>,
}

#[derive(Debug, Deserialize)]
pub struct ModpackLoaderVersion {
    pub id: String,
    pub primary: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModpackFiles {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    pub required: bool,
}
