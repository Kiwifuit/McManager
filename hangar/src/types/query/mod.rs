mod search;
mod version;

pub use search::SearchQueryBuilder;
use serde::Serialize;
use serde_json::to_string;
pub use version::VersionQueryBuilder;

#[derive(Debug)]
pub struct GenericPagination {
    pub(crate) limit: u8,
    pub(crate) offset: u8,
}

impl Serialize for GenericPagination {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serialized = to_string(self).unwrap();

        serializer.serialize_str(serialized.as_str())
    }
}

impl Default for GenericPagination {
    fn default() -> Self {
        Self {
            limit: 25,
            offset: 0,
        }
    }
}

impl GenericPagination {
    pub fn set_limit(&mut self, limit: u8) {
        self.limit = limit;
    }

    pub fn set_offset(&mut self, offset: u8) {
        self.offset = offset;
    }
}
