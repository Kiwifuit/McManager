use serde::Deserialize;

pub struct MavenRepository {
    base_url: String,
    artifacts: Vec<MavenArtifact>,
}

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersionVersioning {
    release: String,
    latest: String,
    last_updated: u64,
    versions: MavenArtifactVersionVersions,
}
#[derive(Debug, Deserialize)]
pub struct MavenArtifactVersionVersions {
    version: Vec<String>,
}
