use super::GenericPagination;
use crate::types::{HangarPlatform, HangarTags};

pub struct SearchQuery {
    prioritize_exact_match: bool,
    pagination: GenericPagination,
    sort: SortBy,
    category: String,
    platform: HangarPlatform,
    owner: String,
    query: String,
    license: String,
    version: String,
    tag: HangarTags,
}

pub enum SortBy {
    Views,
    Downloads,
    Newest,
    Stars,
    Updated,
    RecentDownloads,
    RecentViews,
    Slugs,
}
