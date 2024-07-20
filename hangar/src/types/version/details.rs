use super::HangarProjectPlatform;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HPDownloadDetails {
    pub file_info: HangarProjectDownloadFile,
    pub external_url: Option<String>,
    pub download_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HPPluginDependencyDetails {
    pub required: bool,
    pub external_url: Option<String>,
    pub platform: HangarProjectPlatform,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarProjectDownloadFile {
    pub name: String,
    pub size_bytes: usize,
    pub sha_256_hash: String,
}
