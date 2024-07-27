use serde::Deserialize;

#[derive(Debug)]
pub struct MavenRepository {
    base_url: String,
    artifacts: Vec<MavenArtifact>,
}

#[derive(Debug)]
pub struct MavenArtifact {
    pub(crate) group_id: String,
    pub(crate) artifact_id: String,
    pub(crate) version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersions {
    group_id: String,
    artifact_id: String,
    versioning: MavenArtifactVersionVersioning,
}

impl MavenArtifact {
    pub fn new<T: ToString>(artifact_id: T, group_id: T) -> Self {
        Self {
            group_id: group_id.to_string(),
            artifact_id: artifact_id.to_string(),
            version: None,
        }
    }

    pub fn new_with_version<T: ToString>(artifact_id: T, group_id: T, version: T) -> Self {
        Self {
            group_id: group_id.to_string(),
            artifact_id: artifact_id.to_string(),
            version: Some(version.to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersionVersioning {
    release: String,
    latest: String,
    last_updated: u64,
    versions: MAVersioningVersions,
}
#[derive(Debug, Deserialize)]
pub struct MAVersioningVersions {
    version: Vec<String>,
}
