use super::{Gallery, License, ModRequirement, ModrinthProjectMeta, ProjectType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchProjectResult {
    pub hits: Vec<SearchProjectHit>,
    pub offset: u8,
    pub limit: u8,
    pub total_hits: u16,
}

#[derive(Debug, Deserialize)]
/// Represents a search result returned by Modrinth.
///
/// *The documentation for this struct's fields have*
/// *been copied over from [Modrinth's documentation](https://docs.modrinth.com/#tag/project_result_model)*
pub struct SearchProjectHit {
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
    /// The server side support of the project
    pub server_side: ModRequirement,
    /// The project type of the project
    pub project_type: ProjectType,
    /// The total number of downloads of the project
    pub downloads: u32,
    /// The URL of the project's icon
    pub icon_url: String,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: u32,
    /// The ID of the project
    pub project_id: String,
    /// The username of the project's author
    pub author: String,
    /// A list of the minecraft versions supported by the project
    pub versions: Vec<String>,
    /// The date the project was added to search
    pub date_created: String,
    /// The date the project was last modified
    pub date_modified: String,
    /// The latest version of minecraft that this project supports
    pub latest_version: String,
    /// The SPDX license ID of a project
    pub license: License,
    /// All gallery images attached to the project
    pub gallery: Gallery,
    /// The featured gallery image of the project
    pub featured_gallery: Option<String>,
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
