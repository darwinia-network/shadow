//! MMR Runner
use super::{
    hash::{MergeHash, H256},
    model::Cache,
    result::Error,
    schema::{eth_header_with_proof_caches::dsl::*, mmr_store::dsl::*},
    store::{Store, DEFAULT_RELATIVE_MMR_DB},
};
use cmmr::MMR;
use diesel::{dsl::count, prelude::*, result::Error as DieselError};
use std::{cmp::Ordering, path::PathBuf, thread, time};

fn log2_floor(mut num: i64) -> i64 {
    let mut res = -1;
    while num > 0 {
        res += 1;
        num >>= 1;
    }
    res
}

/// MMR Runner
pub struct Runner {
    /// MMR Storage
    pub path: PathBuf,
    store: Store,
    conn: SqliteConnection,
}

impl Default for Runner {
    fn default() -> Runner {
        let mut path = dirs::home_dir().unwrap_or_default();
        path.push(DEFAULT_RELATIVE_MMR_DB);
        trace!(
            "The database path of shadow service is {}",
            DEFAULT_RELATIVE_MMR_DB
        );
        let store = Store::new(&path);
        let conn = store.conn();

        Runner { path, store, conn }
    }
}

impl Runner {
    /// MMR size to last leaf `O(log2(log2(n)))`
    pub fn mmr_size_to_last_leaf(mmr_size: i64) -> i64 {
        if mmr_size == 0 {
            return 0;
        }

        let mut m = log2_floor(mmr_size);
        loop {
            match (2 * m - m.count_ones() as i64).cmp(&mmr_size) {
                Ordering::Equal => return m - 1,
                Ordering::Greater => m -= 1,
                Ordering::Less => m += 1,
            }
        }
    }

    /// Start the runner
    pub fn start(&mut self) -> Result<(), Error> {
        match self.mmr_count() {
            Ok(mut base) => {
                base = Runner::mmr_size_to_last_leaf(base);
                loop {
                    if let Err(e) = self.push(base) {
                        match e {
                            Error::Diesel(DieselError::NotFound) => {
                                warn!("Could not find block {:?} in cache", base)
                            }
                            _ => error!("Push block to mmr_store failed: {:?}", e),
                        }

                        trace!("MMR service restarting after 10s...");
                        thread::sleep(time::Duration::from_secs(10));
                        return self.start();
                    } else {
                        trace!("push eth block number {} into db succeed.", base);
                        base += 1;
                    }
                }
            }
            Err(e) => {
                error!("Get mmr count failed, {:?}", e);
                trace!("MMR service sleep for 3s...");
                thread::sleep(time::Duration::from_secs(3));
                self.start()
            }
        }
    }

    /// Get block hash by number
    pub fn get_hash(&mut self, block: i64) -> Result<String, Error> {
        let cache = eth_header_with_proof_caches
            .filter(number.eq(block))
            .first::<Cache>(&self.conn)?;

        Ok(cache.hash)
    }

    /// Push new header hash into storage
    pub fn push(&mut self, pnumber: i64) -> Result<(), Error> {
        let cache = eth_header_with_proof_caches
            .filter(number.eq(pnumber))
            .first::<Cache>(&self.conn)?;

        let mut mmr =
            MMR::<_, MergeHash, _>::new(self.mmr_count().unwrap_or(0) as u64, &self.store);
        if let Err(e) = mmr.push(H256::from(&cache.hash[2..])) {
            return Err(Error::MMR(e));
        }

        // eth_header_with_proof_caches
        let proot = mmr.get_root()?;
        diesel::update(eth_header_with_proof_caches.filter(number.eq(pnumber)))
            .set(root.eq(Some(H256::hex(&proot))))
            .execute(&self.conn)?;

        mmr.commit()?;
        Ok(())
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
}
