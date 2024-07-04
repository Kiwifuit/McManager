use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForgeModpack {
    pub minecraft: ModpackLoaderMeta,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<ModpackFiles>,
    pub overrides: String,
}

#[derive(Debug, Deserialize)]
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
    pub project_id: u32,
    pub file_id: u32,
    pub required: bool,
}
