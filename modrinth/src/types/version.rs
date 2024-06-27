use serde::Deserialize;

use crate::Loader;

#[derive(Debug, Deserialize)]
pub struct ModrinthProjectVersion {
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    pub dependencies: Option<Vec<VersionDependency>>,
    pub game_versions: Vec<String>,
    pub version_type: VersionType,
    pub loaders: Option<Vec<Loader>>,
    pub featured: bool,
    pub status: Option<VersionStatus>,
    pub requrested_status: Option<VersionRequestedStatus>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub date_published: String,
    pub downloads: usize,
    pub files: Vec<VersionFile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionRequestedStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
}

#[derive(Debug, Deserialize)]
pub struct VersionDependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Debug, Deserialize)]
pub struct VersionFile {
    pub hashes: VersionFileHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: usize,
    pub file_type: Option<VersionFileType>,
}

#[derive(Debug, Deserialize)]
pub struct VersionFileHashes {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Debug, Deserialize)]
pub enum VersionFileType {
    #[serde(rename = "required-resource-pack")]
    Required,
    #[serde(rename = "optional-resource-pack")]
    Optional,
}
