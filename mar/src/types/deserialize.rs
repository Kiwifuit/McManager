#[cfg(feature = "types")]
use serde::Deserialize;

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersions {
    pub group_id: String,
    pub artifact_id: String,
    pub versioning: MavenArtifactVersionVersioning,
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersionVersioning {
    release: String,
    latest: String,
    last_updated: u64,
    versions: MAVersioningVersions,
}

#[cfg(feature = "types")]
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

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
pub struct MAVersioningVersions {
    version: Vec<String>,
}

#[cfg(feature = "types")]
impl Into<Vec<String>> for MAVersioningVersions {
    fn into(self) -> Vec<String> {
        self.version
    }
}

#[cfg(feature = "types")]
impl Into<Vec<String>> for &MAVersioningVersions {
    fn into(self) -> Vec<String> {
        self.version.clone()
    }
}
