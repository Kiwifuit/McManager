use crate::types::Loader;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VersionQuery {
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::types::serialize_vec_urlencoded"
    )]
    pub(crate) loaders: Vec<Loader>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::types::serialize_vec_urlencoded"
    )]
    pub(crate) game_versions: Vec<String>,
    pub(crate) featured: bool,
}

#[derive(Debug, Default)]
pub struct VersionQueryBuilder {
    pub loaders: Option<Vec<Loader>>,
    pub versions: Option<Vec<String>>,
    pub featured: Option<bool>,
}

impl VersionQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn loaders(mut self, loaders: Vec<Loader>) -> Self {
        self.loaders = Some(loaders);
        self
    }

    pub fn versions<A: ToString>(mut self, versions: Vec<A>) -> Self {
        self.versions = Some(versions.iter().map(|a| a.to_string()).collect());
        self
    }

    pub fn featured(mut self, featured: bool) -> Self {
        self.featured = Some(featured);
        self
    }

    pub fn build(self) -> VersionQuery {
        VersionQuery {
            loaders: self.loaders.unwrap_or_default(),
            game_versions: self.versions.unwrap_or_default(),
            featured: self.featured.unwrap_or_default(),
        }
    }
}
