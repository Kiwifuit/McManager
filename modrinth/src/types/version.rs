use serde::Deserialize;

use crate::Loader;

#[derive(Debug, Deserialize)]
/// Represents a version of a Modrinth project.
///
/// *The documentation for this struct's fields have*
/// *been copied over from [Modrinth's documentation](https://docs.modrinth.com/#tag/version_model)*
pub struct ModrinthProjectVersion {
    /// The name of this version
    pub name: String,
    /// The version number. Ideally will follow semantic versioning
    pub version_number: String,
    /// The changelog for this version
    pub changelog: Option<String>,
    /// A list of specific versions of projects that this version depends on
    pub dependencies: Option<Vec<VersionDependency>>,
    /// The release channel for this version
    pub game_versions: Vec<String>,
    /// A list of versions of Minecraft that this version supports
    pub version_type: VersionType,
    /// The mod loaders that this version supports
    pub loaders: Option<Vec<Loader>>,
    /// Whether the version is featured or not
    pub featured: bool,
    /// The ID of the version, encoded as a base62 string
    pub id: String,
    /// The ID of the project this version is for
    pub project_id: String,
    /// The ID of the author who published this version
    pub author_id: String,
    /// The date this version has been published
    pub date_published: String,
    /// The number of times this version has been downloaded
    pub downloads: usize,
    /// A list of files available for download for this version
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
/// Represents a dependency of a `ModrinthProjectVersion`
pub enum VersionDependency {
    /// The dependency has yet to be resolved
    Unresolved(UnresolvedVersionDependency),
    #[serde(skip)]
    /// The dependency has been resolved
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
/// Represents a unresolved dependency of a `ModrinthProjectVersion`
pub struct UnresolvedVersionDependency {
    /// The version id of the unresolved dependency
    pub version_id: Option<String>,
    /// The project id of the unresolved dependency
    pub project_id: Option<String>,
    /// The file name of the unresolved dependency
    pub file_name: Option<String>,
    /// The requirement type (Required, Optional, etc.) of the unresolved dependency
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
/// Represents a resolved dependency of a `ModrinthProjectVersion`
pub struct ResolvedVersionDependency {
    /// the resolved project dependency
    pub dependency: ModrinthProjectVersion,
    /// The requirement type (Required, Optional, etc.) of the unresolved dependency
    pub dependency_type: DependencyType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
/// Represents the relationships a dependency can take
pub enum DependencyType {
    /// Dependency is required for this version
    Required,
    /// Dependency is optional for this version,
    /// no need to download
    Optional,
    /// Dependency cannot work with this version
    Incompatible,
    /// Dependency is embedded in this version,
    /// no need to download
    Embedded,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
/// Represents a file listed in the `.files` map
pub struct VersionFile {
    /// Hashes of the file provided by Modrinth
    pub hashes: VersionFileHashes,
    /// URL pointing to the resource to download
    pub url: String,
    /// Name of the file
    pub filename: String,
    /// Is the file a primary file
    pub primary: bool,
    /// Size of the file
    pub size: usize,
    /// The type of the file
    pub file_type: Option<VersionFileType>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct VersionFileHashes {
    /// SHA512 hash of the file
    pub sha512: String,
    /// SHA1 hash of the file
    pub sha1: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
/// Represents the relationships a non-dependency file can take
pub enum VersionFileType {
    /// Non-dependency file is required
    #[serde(rename = "required-resource-pack")]
    Required,
    /// Non-dependency file is optional
    #[serde(rename = "optional-resource-pack")]
    Optional,
}
