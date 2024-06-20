use crate::types::{
    project::ModrinthProject,
    query::{ProjectQuery, SearchProjectHit, SearchProjectResult},
};
use std::time::Duration;

use log::debug;
use reqwest::Client;
use thiserror::Error;

const ENDPOINT: &str = "https://api.modrinth.com";

#[derive(Debug, Error)]
pub enum APIError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

pub async fn check_api() -> Result<(bool, Client), APIError> {
    debug!("building client");
    let client = Client::builder()
        .user_agent(format!(
            "{} using {} v{}",
            std::env::var("CARGO_BIN_NAME").unwrap_or(String::from("<unknown>")),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .https_only(true)
        .timeout(Duration::from_secs(30))
        .connection_verbose(false)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    debug!("vibe checking modrinth endpoint at {:?}", ENDPOINT);
    let resp = client.get(ENDPOINT).send().await;

    Ok((resp.is_ok(), client))
}

pub async fn search_project(
    client: &Client,
    params: &ProjectQuery,
) -> Result<Vec<SearchProjectHit>, APIError> {
    let raw_res: SearchProjectResult = client
        .get(format!("{}/v2/search", ENDPOINT))
        .query(params)
        .send()
        .await
        .unwrap()
        .json()
        .await?;

    assert_eq!(raw_res.hits.len(), params.limit as usize);
    Ok(raw_res.hits)
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

pub async fn get_client() -> Client {
    let api_check = check_api().await;

    assert!(api_check.is_ok());
    let (_labrinth_responding, client) = api_check.unwrap();

    client
}

#[cfg(test)]
mod tests {
    use query::ProjectQueryBuilder;

    use super::*;
    use crate::types::*;

    #[tokio::test]
    async fn check_api_works() {
        let api_check = check_api().await;

        assert!(api_check.is_ok());
        let (labrinth_responding, _client) = api_check.unwrap();

        assert!(labrinth_responding);
    }

    #[tokio::test]
    async fn check_search_projects() {
        let client = get_client().await;

        let query = ProjectQueryBuilder::new()
            .query("gravestones")
            .limit(3)
            .index(query::IndexBy::Relevance)
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
            .index(query::IndexBy::Relevance)
            .build();

        let res = search_project(&client, &query).await.unwrap();

        let res = res.first().unwrap();
        assert_eq!(res.project_id, "5yJ5IDKm"); // https://modrinth.com/mod/kontraption
        assert_eq!(res.project_type, "mod");

        let project = get_project(&client, res).await;

        assert!(project.is_ok());

        let project = project.unwrap();
        assert_eq!(project.id, "5yJ5IDKm");
        assert_eq!(project.project_type, Some("mod"));
    }
}
