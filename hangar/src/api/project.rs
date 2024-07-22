use crate::types::{query::search::SearchQuery, HangarProjects};
use reqwest::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

pub async fn search_project(
    client: &Client,
    params: &SearchQuery,
) -> Result<HangarProjects, ProjectError> {
    Ok(client
        .get(format!("{}/api/v1/projects", super::HANGAR_ENDPOINT))
        .query(params)
        .send()
        .await?
        .json()
        .await?)
}
