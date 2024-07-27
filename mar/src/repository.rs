use crate::types::*;

use quick_xml::de::from_str;
use reqwest::get;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("version already available")]
    RepositoryAvailable,

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("xml deserialization error: {0}")]
    XmlParse(#[from] quick_xml::DeError),
}

pub async fn get_versions(
    artifact: &MavenArtifact,
    base_url: String,
) -> Result<MavenArtifactVersions, RepositoryError> {
    let raw = get(format!(
        "{}/{}/{}/maven-metadata.xml",
        base_url,
        artifact.group_id.replace('.', "/"),
        artifact.artifact_id
    ))
    .await?
    .text()
    .await?;

    let parsed = from_str(&raw)?;

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_versions() {
        let artifact = MavenArtifact::new("forge", "net.minecraftforge");

        let versions =
            get_versions(&artifact, "https://maven.minecraftforge.net".to_string()).await;

        assert!(versions.is_ok());
    }
}
