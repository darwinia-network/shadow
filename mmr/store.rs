//! MMR store
use cmmr::{MMRStore, Result};
use std::path::PathBuf;

/// Shadow db table
#[derive(Clone, Insertable, Queryable, Debug)]
#[table_name = "eth_header_with_proof_caches"]
pub struct Shadow {
    number: i64,
    header: String,
    proof: String,
    mmr: String,
}

table! {
    eth_header_with_proof_caches(number) {
        number -> BigInt,
        header -> Text,
        proof -> Text,
        mmr -> Text,
    }
}

/// MMR Store
pub struct Store {
    _path: PathBuf,
}

impl MMRStore<String> for Store {
    fn get_elem(&self, _pos: u64) -> Result<Option<String>> {
        Ok(Some("".into()))
    }

    fn append(&mut self, _pos: u64, _elems: Vec<String>) -> Result<()> {
        Ok(())
    }
}
