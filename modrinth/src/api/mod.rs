#![cfg_attr(not(feature = "api"), allow(unused_imports, dead_code))]

use std::time::Duration;

#[cfg(feature = "api")]
use log::{debug, error, info};
#[cfg(feature = "api")]
use reqwest::Client;
#[cfg(feature = "api")]
use thiserror::Error;

#[cfg(feature = "api")]
pub mod dependency;
#[cfg(feature = "api")]
pub mod project;
#[cfg(feature = "api")]
pub mod version;

#[cfg(feature = "api")]
pub use project::*;

const ENDPOINT: &str = "https://api.modrinth.com";

#[cfg(feature = "api")]
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

#[cfg(feature = "api")]
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

#[cfg(feature = "api")]
pub async fn get_client() -> Option<Client> {
    info!("Checking api");
    let api_check = check_api().await;

    if let Err(api_err) = api_check {
        error!("Error while testing Modrinth api: {:}. Are you sure you are connected to the internet?", api_err);
        return None;
    }
    let (_labrinth_responding, client) = api_check.unwrap();

    Some(client)
}

#[cfg(test)]
#[cfg(feature = "api")]
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
