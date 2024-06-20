use super::{Gallery, License, ModRequirement};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SearchProjectResult {
    pub(crate) hits: Vec<SearchProjectHit>,
    #[serde(rename = "offset")]
    _offset: u8,
    pub(crate) limit: u8,
    pub(crate) total_hits: u16,
}

#[derive(Debug, Deserialize)]
pub struct SearchProjectHit {
    pub project_id: String,
    pub project_type: String,
    pub slug: String,
    pub author: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub downloads: u32,
    pub follows: u32,
    pub icon_url: String,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: License,
    pub client_side: ModRequirement,
    pub server_side: ModRequirement,
    pub gallery: Gallery,
    pub featured_gallery: Option<String>,
    pub color: u32,
}
