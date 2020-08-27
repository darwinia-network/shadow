//! Data models
use super::schema::*;

/// MMR Store table
#[derive(AsChangeset, Clone, Insertable, Queryable, Debug, Default)]
#[table_name = "mmr_store"]
pub struct Header {
    /// MMR Store elem
    pub elem: String,
    /// MMR Store pos
    pub pos: i64,
}

impl Header {
    /// Generate new header
    pub fn new(relem: String, rpos: i64) -> Header {
        Header {
            elem: relem,
            pos: rpos,
        }
    }
}
