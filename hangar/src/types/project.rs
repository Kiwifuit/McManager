use super::{DateTime, HangarVisibility};
use bitflags::bitflags;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProject {
    pub created_at: DateTime,
    pub name: String,
    pub namespace: HangarProjectNamespace,
    pub last_updated: DateTime,
    pub avatar_url: String,
    pub description: String,
    pub category: HangarProjectCategory,
    pub visibility: HangarVisibility,
    pub settings: HangarProjectSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarProjectSettings {
    pub links: Option<HangarProjectLinks>,
    pub tags: HangarProjectTags,
    pub license: HangarProjectLicense,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarProjectNamespace {
    owner: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarProjectLinks {
    links: Vec<HangarProjectLink>,
}

#[derive(Debug, Deserialize)]
struct HangarProjectLink {
    pub id: u8,
    pub name: String,
    pub url: String,
}

bitflags! {
    #[derive(Debug, Deserialize)]
    pub struct HangarProjectTags: u8 {
        const ADDON          = 1;
        const LIBRARY        = 2;
        const SUPPORTS_FOLIA = 3;
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HangarProjectCategory {
    AdminTools,
    Chat,
    DevTools,
    Economy,
    Gameplay,
    Games,
    Protection,
    RolePlaying,
    WorldManagement,
    Misc,
    Undefined,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectLicense {
    name: String,
    url: String,

    #[serde(rename = "type")]
    license_type: String,
}
