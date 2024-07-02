use super::{APIError, ENDPOINT};
use crate::types::query::VersionQuery;
use crate::types::version::ModrinthProjectVersion;
use crate::types::ModrinthProjectMeta;
use reqwest::Client;

#[allow(private_bounds)]
pub async fn get_versions<M>(
    client: &Client,
    project: &M,
    params: &VersionQuery,
) -> Result<Vec<ModrinthProjectVersion>, APIError>
where
    M: ModrinthProjectMeta,
{
    let resp: Vec<ModrinthProjectVersion> = client
        // TODO: ADD ERROR
        .get(format!(
            "{}/v2/project/{}/version",
            ENDPOINT,
            project.project_id().unwrap()
        ))
        .query(params)
        .send()
        .await
        .unwrap()
        // .text()
        .json()
        .await?;

    Ok(resp)
}

pub(crate) async fn get_version<M>(
    client: &Client,
    project: &M,
) -> Result<ModrinthProjectVersion, APIError>
where
    M: ModrinthProjectMeta,
{
    let resp: ModrinthProjectVersion = client
        // TODO: ADD ERROR
        .get(format!(
            "{}/v2/version/{}",
            ENDPOINT,
            project.version_id().unwrap()
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await?;

    Ok(resp)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        get_client, get_project, search_project, IndexBy, Loader, ProjectQueryBuilder,
        VersionQueryBuilder,
    };

    #[tokio::test]
    async fn check_get_versions() {
        let client = get_client().await;

        let query = ProjectQueryBuilder::new()
            .query("kontraption")
            .limit(1)
            .index(IndexBy::Relevance)
            .build();

        let (res, _) = search_project(&client, &query).await.unwrap();
        let project = get_project(&client, res.first().unwrap()).await.unwrap();

        let v_query = VersionQueryBuilder::new()
            .featured(true)
            .versions(vec!["1.20.1"])
            .loaders(vec![Loader::Forge])
            .build();

        let version = get_versions(&client, &project, &v_query).await;

        assert!(version.is_ok());
        assert!(!version.unwrap().is_empty());
    }
}
