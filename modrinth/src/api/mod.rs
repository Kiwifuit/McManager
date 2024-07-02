use std::time::Duration;

use log::debug;
use reqwest::Client;
use thiserror::Error;

pub mod dependency;
pub mod project;
pub mod version;

pub use project::*;

const ENDPOINT: &str = "https://api.modrinth.com";

#[derive(Debug, Error)]
pub enum APIError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("http error: {0}")]
    ResolvedDependency(String),
    #[error("http error")]
    NoDependencies,

    #[error("http error")]
    UnresolvableDependency,
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

pub async fn get_client() -> Client {
    let api_check = check_api().await;

    assert!(api_check.is_ok());
    let (_labrinth_responding, client) = api_check.unwrap();

    client
}

#[cfg(test)]
mod tests {
    use super::check_api;

    #[tokio::test]
    async fn check_api_works() {
        let api_check = check_api().await;

        assert!(api_check.is_ok());
        let (labrinth_responding, _client) = api_check.unwrap();

        assert!(labrinth_responding);
    }
}
