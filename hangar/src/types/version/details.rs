use super::HangarPlatform;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HPDownloadDetails {
    pub file_info: HangarVersionDownloadFile,
    pub external_url: Option<String>,
    pub download_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HPPluginDependencyDetails {
    pub required: bool,
    pub external_url: Option<String>,
    pub platform: HangarPlatform,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarVersionDownloadFile {
    pub name: String,
    pub size_bytes: usize,
    pub sha_256_hash: String,
}
