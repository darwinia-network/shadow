//! MMR Runner
use super::{
    hash::{MergeHash, H256},
    result::Error,
    schema::{eth_header_with_proof_caches::dsl::*, mmr_store::dsl::*},
    store::{Store, DEFAULT_RELATIVE_MMR_DB},
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
                if self.push(base).is_ok() {
                    base += 1;
                } else {
                    thread::sleep(time::Duration::from_secs(10));
                    return self.start();
                }
            }
        } else {
            info!("init mmr database");
            thread::sleep(time::Duration::from_secs(3));
            self.start()
        }
    }

    /// Get the count of mmr store
    pub fn mmr_count(&self) -> Result<i64, Error> {
        let store = Store::new(&self.path);
        let res = mmr_store.select(count(elem)).first::<i64>(&store.conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }

    /// Get the cache count
    fn cache_count(&self) -> Result<i64, Error> {
        let store = Store::new(&self.path);
        let res = eth_header_with_proof_caches
            .filter(root.is_not_null())
            .select(count(root))
            .first::<i64>(&store.conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }

    /// Push new header hash into storage
    fn push(&mut self, pnumber: i64) -> Result<(), Error> {
        let store = Store::new(&self.path);
        let count = self.mmr_count().unwrap_or(0);

        // Get Hash
        let conn = store.conn();
        let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(count as u64), store);

        // eth_header_with_proof_caches
        let proot = mmr.get_root()?;
        diesel::update(eth_header_with_proof_caches.filter(number.eq(pnumber)))
            .set(root.eq(Some(H256::hex(&proot))))
            .execute(&conn)?;

        mmr.commit()?;
        Ok(())
    }
}
