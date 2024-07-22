mod search;
mod version;

#[derive(Debug)]
pub struct GenericPagination {
    pub(crate) limit: u8,
    pub(crate) offset: u8,
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
