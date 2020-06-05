//! MMR store
use self::eth_header_with_proof_caches::{columns::pos, dsl::*};
use cmmr::{Error, MMRStore, Result as MMRResult};
use diesel::{dsl::count, prelude::*};
use std::path::PathBuf;

/// Shadow db table
#[derive(Clone, Insertable, Queryable, Debug, Default)]
#[table_name = "eth_header_with_proof_caches"]
pub struct Shadow {
    header: String,
    mmr: String,
    number: i64,
    pos: i64,
    proof: String,
}

table! {
    eth_header_with_proof_caches(number) {
        header -> Text,
        mmr -> Text,
        number -> BigInt,
        pos -> BigInt,
        proof -> Text,
    }
}

/// MMR Store
pub struct Store {
    pub conn: SqliteConnection,
}

impl Store {
    /// New store with path
    pub fn new(p: &PathBuf) -> Store {
        Store {
            conn: SqliteConnection::establish(&p.to_string_lossy())
                .unwrap_or_else(|_| panic!("Error connecting to {:?}", p)),
        }
    }
}

impl MMRStore<String> for Store {
    fn get_elem(&self, rpos: u64) -> MMRResult<Option<String>> {
        let shadow = eth_header_with_proof_caches
            .filter(pos.eq(rpos as i64))
            .first::<Shadow>(&self.conn);

        if shadow.is_ok() {
            Ok(Some(shadow.unwrap_or_default().mmr))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, rpos: u64, elems: Vec<String>) -> MMRResult<()> {
        let count_res = eth_header_with_proof_caches
            .select(count(mmr))
            .first::<i64>(&self.conn);

        if count_res.is_err() {
            return Err(Error::StoreError(
                "Counts mmr from sqlite3 failed".to_string(),
            ));
        }

        // Specify the count
        let count = count_res.unwrap() as u64;
        if rpos != count {
            Err(Error::InconsistentStore)?;
        }

        for (i, elem) in elems.into_iter().enumerate() {
            let target = eth_header_with_proof_caches.filter(pos.eq(count as i64 + i as i64));
            let res = diesel::update(target).set(mmr.eq(elem)).execute(&self.conn);
            if res.is_err() {
                return Err(Error::StoreError(
                    "Updates mmr into sqlite3 failed".to_string(),
                ));
            }
        }

        Ok(())
    }
}
