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
#[serde(untagged)]
pub enum VersionDependency {
    Unresolved(UnresolvedVersionDependency),
    #[serde(skip)]
    Resolved(ResolvedVersionDependency),
}

impl VersionDependency {
    pub fn is_resolved(&self) -> bool {
        match self {
            Self::Resolved(_) => true,
            Self::Unresolved(_) => false,
        }
    }
    pub fn is_unresolved(&self) -> bool {
        match self {
            Self::Resolved(_) => false,
            Self::Unresolved(_) => true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UnresolvedVersionDependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: DependencyType,
}

impl super::ModrinthProjectMeta for UnresolvedVersionDependency {
    fn project_id(&self) -> Option<&String> {
        self.project_id.as_ref()
    }

    fn version_id(&self) -> Option<&String> {
        self.version_id.as_ref()
    }
}

#[derive(Debug)]
pub struct ResolvedVersionDependency {
    pub dependency: ModrinthProjectVersion,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct VersionFile {
    pub hashes: VersionFileHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: usize,
    pub file_type: Option<VersionFileType>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct VersionFileHashes {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum VersionFileType {
    #[serde(rename = "required-resource-pack")]
    Required,
    #[serde(rename = "optional-resource-pack")]
    Optional,
}
