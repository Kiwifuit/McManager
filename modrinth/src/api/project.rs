use super::{APIError, ENDPOINT};
use reqwest::Client;

use crate::types::project::ModrinthProject;
use crate::types::query::ProjectQuery;
use crate::types::result::SearchProjectHit;
use crate::types::result::SearchProjectResult;

pub async fn search_project(
    client: &Client,
    params: &ProjectQuery,
) -> Result<(Vec<SearchProjectHit>, usize), APIError> {
    let raw_res: SearchProjectResult = client
        .get(format!("{}/v2/search", ENDPOINT))
        .query(params)
        .send()
        .await
        .unwrap()
        .json()
        .await?;

    assert_eq!(raw_res.hits.len(), params.limit as usize);
    Ok((raw_res.hits, raw_res.total_hits))
}

pub async fn get_project(
    client: &Client,
    project: &SearchProjectHit,
) -> Result<ModrinthProject, APIError> {
    Ok(client
        .get(format!("{}/v2/project/{}", ENDPOINT, project.project_id))
        .send()
        .await
        .unwrap()
        .json()
        .await?)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::get_client;
    use crate::types::query::ProjectQueryBuilder;
    use crate::types::Facet;
    use crate::types::Loader;
    use crate::types::{IndexBy, ProjectType};

    #[tokio::test]
    async fn check_search_projects() {
        let client = get_client().await;

        let query = ProjectQueryBuilder::new()
            .query("gravestones")
            .limit(3)
            .index(IndexBy::Relevance)
            .facets(vec![
                vec![Facet::Loader(Loader::Forge)],
                vec![
                    Facet::Category("adventure".to_string()),
                    Facet::Category("utility".to_string()),
                ],
            ])
            .build();

        let res = search_project(&client, &query).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn check_get_project() {
        let client = get_client().await;

        let query = ProjectQueryBuilder::new()
            .query("kontraption")
            .limit(1)
            .index(IndexBy::Relevance)
            .build();

        let (res, _) = search_project(&client, &query).await.unwrap();

        let res = res.first().unwrap();
        assert_eq!(res.project_id, "5yJ5IDKm"); // https://modrinth.com/mod/kontraption
        assert_eq!(res.project_type, "mod");

        let project = get_project(&client, res).await;

        assert!(project.is_ok());

        let project = project.unwrap();

        assert_eq!(project.id, "5yJ5IDKm");
        assert_eq!(project.project_type, ProjectType::Mod);
    }
}
