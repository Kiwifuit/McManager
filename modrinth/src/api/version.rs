use super::{APIError, ENDPOINT};
use crate::types::project::ModrinthProject;
use crate::types::query::VersionQuery;
use crate::types::version::ModrinthProjectVersion;
use reqwest::Client;

pub async fn get_versions(
    client: &Client,
    project: &ModrinthProject,
    params: String,
) -> Result<Vec<ModrinthProjectVersion>, APIError> {
    let resp: Vec<ModrinthProjectVersion> = client
        .get(format!(
            "{}/v2/project/{}/version{}",
            ENDPOINT, project.id, params
        ))
        // .query(params)
        .send()
        .await
        .unwrap()
        // .text()
        .json()
        .await?;

    Ok(resp)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        get_client, get_project, search_project, IndexBy, ProjectQueryBuilder, VersionQueryBuilder,
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

        // let v_query = VersionQueryBuilder::new()
        //     .featured(true)
        //     .versions(vec!["1.19.2"])
        //     .build();

        // dbg!(&v_query);
        // dbg!(serde_urlencoded::to_string(&v_query).unwrap());
        let version = get_versions(&client, &project, "?featured=true".to_string()).await;

        assert!(version.is_ok());
        assert!(!version.unwrap().is_empty());
    }
}
