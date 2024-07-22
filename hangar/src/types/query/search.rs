use super::GenericPagination;
use crate::types::{HangarPlatform, HangarTags};

pub struct SearchQuery {
    pub(crate) prioritize_exact_match: bool,
    pub(crate) pagination: GenericPagination,
    pub(crate) sort: SortBy,
    pub(crate) category: String,
    pub(crate) platform: HangarPlatform,
    pub(crate) owner: String,
    pub(crate) query: String,
    pub(crate) license: String,
    pub(crate) version: String,
    pub(crate) tag: HangarTags,
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

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);

        self
    }

    pub fn platform(mut self, platform: HangarPlatform) -> Self {
        self.platform = Some(platform);

        self
    }

    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);

        self
    }

    pub fn query(mut self, query: String) -> Self {
        self.query = Some(query);

        self
    }

    pub fn license(mut self, license: String) -> Self {
        self.license = Some(license);

        self
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);

        self
    }

    pub fn tag(mut self, tag: HangarTags) -> Self {
        self.tag = Some(tag);

        self
    }

    pub fn build(self) -> SearchQuery {
        SearchQuery {
            prioritize_exact_match: self.prioritize_exact_match.unwrap(),
            pagination: self.pagination.unwrap(),
            sort: self.sort.unwrap(),
            category: self.category.unwrap(),
            platform: self.platform.unwrap(),
            owner: self.owner.unwrap(),
            query: self.query.unwrap(),
            license: self.license.unwrap(),
            version: self.version.unwrap(),
            tag: self.tag.unwrap(),
        }
    }
}
