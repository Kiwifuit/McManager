use serde::Deserialize;

use super::{Loader, ModRequirement, ProjectType};

// TODO: Add serde_valid (optional?)
//       https://docs.rs/serde_valid/latest/serde_valid/
/// Represents 1 project hosted on Modrinth
/// *The documentation for the fields of this struct*
/// *have been copied over from [Modrinth's documentation](https://docs.modrinth.com/#tag/project_model)*
#[derive(Debug, Deserialize)]
pub struct ModrinthProject {
    /// The slug of a project, used for vanity URLs. Regex: `^[\w!@$()`.+,"\-']{3,64}$`
    pub slug: String,
    /// The title or name of the project
    pub title: String,
    /// A short description of the project
    pub description: String,
    /// A list of the categories that the project has
    pub categories: Vec<String>,
    /// The client side support of the project
    pub client_side: ModRequirement,
    /// The client side support of the project
    pub server_side: ModRequirement,
    /// A long form description of the project
    pub body: String,
    /// The status of the project
    pub status: Status,

    /// A list of categories which are searchable but non-primary
    pub additional_categories: Option<Vec<String>>,
    /// An optional link to where to submit bugs or issues with the project
    pub issues_url: Option<String>,
    /// An optional link to the source code of the project
    pub source_url: Option<String>,
    /// An optional link to the project's wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// An optional invite link to the project's discord
    pub discord_url: Option<String>,

    /// The project type of the project
    pub project_type: ProjectType,
    /// The URL of the project's icon
    pub icon_url: Option<String>,

    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<u32>,

    /// The ID of the project, encoded as a base62 string
    pub id: String,
    /// The ID of the team that has ownership of this project
    pub team: String,

    /// The date the project was published
    pub published: String,
    /// The date the project was last updated
    pub updated: String,

    /// A list of the version IDs of the project (will never be empty unless `draft` status)
    pub versions: Vec<String>,
    /// A list of all of the game versions supported by the project
    pub game_versions: Vec<String>,
    /// A list of all of the loaders supported by the project
    pub loaders: Vec<Loader>,
    /// A list of images that have been uploaded to the project's gallery
    pub gallery: Option<Vec<GalleryEntry>>,
}

impl super::ModrinthProjectMeta for ModrinthProject {
    fn project_id(&self) -> Option<&String> {
        Some(&self.id)
    }
}

#[derive(Debug, Deserialize)]
/// Represents an image in a gallery
pub struct GalleryEntry {
    /// The URL of the image
    pub url: String,
    /// The image's title
    pub title: Option<String>,
    /// The image's description
    pub description: Option<String>,
    /// When the image was uploaded
    pub created: String,
    /// What order/index the image should be at
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
