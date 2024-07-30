use crate::types::*;

use log::debug;
use quick_xml::de::from_str;
use reqwest::get;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("version already available")]
    VersionAvailable,

    #[error("version not available")]
    NoVersion,

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("xml deserialization error: {0}")]
    XmlParse(#[from] quick_xml::DeError),
}

pub async fn get_versions<T: ToString>(
    artifact: &MavenArtifact,
    base_url: T,
) -> Result<MavenArtifactVersions, RepositoryError> {
    if artifact.version.is_some() {
        Err(RepositoryError::VersionAvailable)
    } else {
        debug!(
            "got url: {}",
            format!(
                "{}/{}/{}/maven-metadata.xml",
                base_url.to_string(),
                artifact.group_id.replace('.', "/"),
                artifact.artifact_id
            )
        );

        let raw = get(format!(
            "{}/{}/{}/maven-metadata.xml",
            base_url.to_string(),
            artifact.group_id.replace('.', "/"),
            artifact.artifact_id
        ))
        .await?
        .text()
        .await?;

        let parsed = from_str(&raw)?;

        Ok(parsed)
    }
}

pub fn get_artifact<T: ToString>(
    artifact_data: &MavenArtifact,
    base_url: T,
    artifact_name: T,
) -> Result<String, RepositoryError> {
    if artifact_data.version.is_none() {
        Err(RepositoryError::NoVersion)
    } else {
        Ok(format!(
            "{}/{}/{}/{}/{}",
            base_url.to_string(),
            artifact_data.group_id.replace('.', "/"),
            artifact_data.artifact_id,
            artifact_data.version.clone().unwrap(),
            artifact_name.to_string()
        ))
    }
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

    #[tokio::test]
    async fn test_get_version() {
        let mut artifact = MavenArtifact::new("forge", "net.minecraftforge");
        let base_url = "https://maven.minecraftforge.net".to_string();

        let selected_version = get_versions(&artifact, &base_url)
            .await
            .unwrap()
            .versioning
            .latest();

        artifact.set_version(&selected_version);
        let artifact_name = format!("forge-{}-installer.jar", selected_version);
        let expected_artifact_url = format!(
            "https://maven.minecraftforge.net/net/minecraftforge/forge/{0}/forge-{0}-installer.jar",
            selected_version
        );
        let artifact_url = get_artifact(&artifact, base_url, artifact_name);

        assert!(artifact_url.is_ok());
        let artifact_url = artifact_url.unwrap();
        assert_eq!(artifact_url, expected_artifact_url);
    }
}
