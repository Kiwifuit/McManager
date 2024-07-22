use super::GenericPagination;
use crate::types::{HangarPlatform, HangarTags};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    pub(crate) prioritize_exact_match: bool,
    #[serde(flatten)]
    pub(crate) pagination: GenericPagination,
    pub(crate) sort: SortBy,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) category: String,
    pub(crate) platform: HangarPlatform,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) owner: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) query: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) license: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) version: String,
    #[serde(skip_serializing_if = "HangarTags::is_empty")]
    pub(crate) tag: HangarTags,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    Views,
    #[default]
    Downloads,
    Newest,
    Stars,
    Updated,
    RecentDownloads,
    RecentViews,
    Slugs,
}

#[derive(Debug, Default)]
pub struct SearchQueryBuilder {
    prioritize_exact_match: Option<bool>,
    pagination: Option<GenericPagination>,
    sort: Option<SortBy>,
    category: Option<String>,
    platform: Option<HangarPlatform>,
    owner: Option<String>,
    query: Option<String>,
    license: Option<String>,
    version: Option<String>,
    tag: Option<HangarTags>,
}

impl SearchQueryBuilder {
    pub fn prioritize_exact_match(mut self, prioritize_exact_match: bool) -> Self {
        self.prioritize_exact_match = Some(prioritize_exact_match);

        self
    }

    pub fn pagination(mut self, pagination: GenericPagination) -> Self {
        self.pagination = Some(pagination);

        self
    }

    pub fn sort(mut self, sort: SortBy) -> Self {
        self.sort = Some(sort);

        self
    }

    pub fn category<T: ToString>(mut self, category: T) -> Self {
        self.category = Some(category.to_string());

        self
    }

    pub fn platform<T: ToString>(mut self, platform: HangarPlatform) -> Self {
        self.platform = Some(platform);

        self
    }

    pub fn owner<T: ToString>(mut self, owner: T) -> Self {
        self.owner = Some(owner.to_string());

        self
    }

    pub fn query<T: ToString>(mut self, query: T) -> Self {
        self.query = Some(query.to_string());

        self
    }

    pub fn license<T: ToString>(mut self, license: T) -> Self {
        self.license = Some(license.to_string());

        self
    }

    pub fn version<T: ToString>(mut self, version: T) -> Self {
        self.version = Some(version.to_string());

        self
    }

    pub fn tag(mut self, tag: HangarTags) -> Self {
        self.tag = Some(tag);

        self
    }

    pub fn build(self) -> SearchQuery {
        SearchQuery {
            prioritize_exact_match: self.prioritize_exact_match.unwrap_or(true),
            pagination: self.pagination.unwrap_or_default(),
            sort: self.sort.unwrap_or_default(),
            category: self.category.unwrap_or_default(),
            platform: self.platform.unwrap_or_default(),
            owner: self.owner.unwrap_or_default(),
            query: self.query.unwrap_or_default(),
            license: self.license.unwrap_or_default(),
            version: self.version.unwrap_or_default(),
            tag: self.tag.unwrap_or_default(),
        }
    }
}
