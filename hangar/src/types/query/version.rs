use super::GenericPagination;
use crate::types::HangarPlatform;

pub struct VersionQuery {
    pub(crate) pagination: GenericPagination,
    pub(crate) include_hidden_channels: bool,
    pub(crate) platform: HangarPlatform,
    pub(crate) platform_version: String,
}
