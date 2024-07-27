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

    pub(crate) fn set_version<T: ToString>(&mut self, version: T) {
        self.version = Some(version.to_string());
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersions {
    pub group_id: String,
    pub artifact_id: String,
    pub versioning: MavenArtifactVersionVersioning,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersionVersioning {
    release: String,
    latest: String,
    last_updated: u64,
    versions: MAVersioningVersions,
}

impl MavenArtifactVersionVersioning {
    pub fn release(&self) -> String {
        self.release.clone()
    }

    pub fn latest(&self) -> String {
        self.latest.clone()
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated.clone()
    }

    pub fn versions(&self) -> Vec<String> {
        (&self.versions).into()
    }
}

#[derive(Debug, Deserialize)]
pub struct MAVersioningVersions {
    version: Vec<String>,
}

impl Into<Vec<String>> for MAVersioningVersions {
    fn into(self) -> Vec<String> {
        self.version
    }
}

impl Into<Vec<String>> for &MAVersioningVersions {
    fn into(self) -> Vec<String> {
        self.version.clone()
    }
}
