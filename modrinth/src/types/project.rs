use serde::Deserialize;

use super::version::Loader;

#[derive(Debug, Deserialize)]
pub struct ModrinthProject {
    pub id: String,
    pub slug: String,
    // TODO: Turn this into an enum
    pub project_type: Option<String>,
    pub team: Option<String>,
    pub author: Option<String>,

    pub title: String,
    pub description: String,
    #[serde(rename = "downloads")]
    _downloads: u64,
    #[serde(rename = "followers")]
    _followers: u64,
    pub body: String,
    pub license: License,

    pub categories: Vec<String>,
    pub additional_categories: Option<Vec<String>>,
    pub display_categories: Option<Vec<String>>,

    pub server_side: ModRequirement,
    pub client_side: ModRequirement,

    // _thread_id: String,
    color: u32,
    #[serde(rename = "status")]
    _status: String,
    #[serde(rename = "requested_status")]
    _requested_status: String,
    #[serde(rename = "issues_url")]
    _issues_url: String,
    #[serde(rename = "source_url")]
    _source_url: String,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub icon_url: Option<String>,
    pub body_url: Option<String>,
    #[serde(rename = "moderator_message")]
    _moderator_message: Option<String>,
    #[serde(rename = "donation_urls")]
    _donation_urls: Vec<DonationUrl>,
    #[serde(rename = "published")]
    _published: String,
    #[serde(rename = "updated")]
    _updated: String,
    #[serde(rename = "approved")]
    _approved: String,
    #[serde(rename = "queued")]
    _queued: String,

    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<Loader>,
    pub gallery: Vec<GalleryEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModRequirement {
    Optional,
    Required,
    Unsupported,
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
#[serde(untagged)]
pub enum License {
    Single(String),
    Detailed {
        id: String,
        name: String,
        url: Option<String>,
    },
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
