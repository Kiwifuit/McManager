pub struct MavenRepository {
    base_url: String,
    artifacts: Vec<MavenArtifact>,
}

pub struct MavenArtifact {
    group_id: String,
    artifact_id: String,
    version: Option<String>,
}
