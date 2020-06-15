//! MMR Runner
use super::{
    model::Cache,
    schema::{eth_header_with_proof_caches::dsl::*, mmr_store::dsl::*},
    store::DEFAULT_RELATIVE_MMR_DB,
    Error, MergeHash, Store, H256,
};
use cmmr::MMR;
use diesel::{dsl::count, prelude::*};
use std::{path::PathBuf, thread, time};

/// MMR Runner
pub struct Runner {
    /// MMR Storage
    pub path: PathBuf,
}

impl Default for Runner {
    fn default() -> Runner {
        let mut path = dirs::home_dir().unwrap_or_default();
        path.push(DEFAULT_RELATIVE_MMR_DB);

        Runner { path }
    }
}

impl Runner {
    /// Start the runner
    pub fn start(&mut self) -> Result<(), Error> {
        if let Ok(mut base) = self.cache_count() {
            loop {
                if let Ok(_) = self.push(base) {
                    base += 1;
                } else {
                    thread::sleep(time::Duration::from_secs(3));
                    return self.start();
                }
            }
        } else {
            thread::sleep(time::Duration::from_secs(3));
            return self.start();
        }
    }

    /// Get the cache count
    fn cache_count(&self) -> Result<i64, Error> {
        let store = Store::new(&self.path);
        let res = eth_header_with_proof_caches
            .select(count(root))
            .first::<i64>(&store.conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }

    /// Get the count of mmr store
    fn mmr_count(&self) -> Result<i64, Error> {
        let store = Store::new(&self.path);
        let res = mmr_store.select(count(elem)).first::<i64>(&store.conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }

    /// Push new header hash into storage
    pub fn push(&mut self, pnumber: i64) -> Result<(), Error> {
        let store = Store::new(&self.path);
        let count = self.mmr_count()?;

        // Get Hash
        let conn = store.conn();
        let cache = eth_header_with_proof_caches
            .filter(number.eq(pnumber))
            .first::<Cache>(&store.conn)?;

        let mut mmr = MMR::<_, MergeHash, _>::new(count as u64, store);
        mmr.push(H256::from(&cache.hash))?;

        // eth_header_with_proof_caches
        let proot = mmr.get_root()?;
        diesel::replace_into(eth_header_with_proof_caches)
            .values(&vec![(number.eq(pnumber), root.eq(H256::hex(&proot)))])
            .execute(&conn)?;

        mmr.commit()?;
        Ok(())
    }
}
