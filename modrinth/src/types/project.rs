use serde::Deserialize;

use super::{Loader, ModRequirement, ProjectType};

// TODO: Add serde_valid (optional?)
//       https://docs.rs/serde_valid/latest/serde_valid/
#[derive(Debug, Deserialize)]
pub struct ModrinthProject {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: ModRequirement,
    pub server_side: ModRequirement,
    pub body: String,
    pub status: Status,

    pub additional_categories: Option<Vec<String>>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,

    pub project_type: ProjectType,
    pub downloads: usize,
    pub icon_url: Option<String>,

    pub color: Option<u32>,

    pub id: String,
    pub team: String,

    pub published: String,
    pub updated: String,
    pub approved: Option<String>,
    pub queued: Option<String>,

    pub followers: u32,
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<Loader>,
    pub gallery: Option<Vec<GalleryEntry>>,
}

impl super::ModrinthProjectMeta for ModrinthProject {
    fn project_id(&self) -> Option<&String> {
        Some(&self.id)
    }
}

#[derive(Debug, Deserialize)]
pub struct GalleryEntry {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: String,
    pub ordering: Option<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Withheld,
    Scheduled,
    Private,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Approved,
    Archived,
    Unlisted,
    Private,
    Draft,
}
