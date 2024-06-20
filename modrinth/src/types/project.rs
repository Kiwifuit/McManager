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

    #[serde(rename = "requested_status")]
    _requested_status: Option<RequestedStatus>,
    #[serde(rename = "donation_urls")]
    _donation_urls: Option<Vec<DonationUrl>>,
    #[serde(rename = "thread_id")]
    _thread_id: Option<String>,
    #[serde(rename = "monetization_status")]
    _monetization_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GalleryEntry {
    pub url: String,
    #[serde(rename = "featured")]
    _featured: bool,
    pub title: String,
    pub description: String,
    pub created: String,
    pub ordering: u8,
}

#[derive(Debug, Deserialize)]
pub struct DonationUrl {
    #[serde(rename = "id")]
    _id: String,
    #[serde(rename = "platform")]
    _platform: String,
    #[serde(rename = "url")]
    _url: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
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
#[serde(untagged, rename_all = "lowercase")]
pub enum RequestedStatus {
    Approved,
    Archived,
    Unlisted,
    Private,
    Draft,
}
