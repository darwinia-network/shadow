//! Data models
use super::schema::*;

/// Shadow db table
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

// /// Shadow db table
// #[derive(AsChangeset, Clone, Insertable, Queryable, Debug, Default)]
// #[table_name = "mmr_store"]
// pub struct Header2 {
//     pub elem: String,
//     pub pos: i64,
// }
//
// impl Header2 {
//     pub fn new(relem: String, rpos: i64) -> Header {
//         Header {
//             elem: relem,
//             pos: rpos,
//         }
//     }
// }
