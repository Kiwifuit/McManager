use super::{Gallery, License, ModRequirement, ModrinthProjectMeta};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchProjectResult {
    pub hits: Vec<SearchProjectHit>,
    pub offset: u8,
    pub limit: u8,
    pub total_hits: u16,
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

impl ModrinthProjectMeta for SearchProjectHit {
    fn project_id(&self) -> Option<&String> {
        Some(&self.project_id)
    }
}

impl ModrinthProjectMeta for &SearchProjectHit {
    fn project_id(&self) -> Option<&String> {
        Some(&self.project_id)
    }
}
