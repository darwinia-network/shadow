//! MMR Runner
use super::{
    hash::{MergeHash, H256},
    helper,
    model::Cache,
    result::Error,
    schema::{eth_header_with_proof_caches::dsl::*, mmr_store::dsl::*},
    store::{Store, DEFAULT_RELATIVE_MMR_DB},
};
use cmmr::MMR;
use diesel::{dsl::count, prelude::*, result::Error as DieselError};
use std::{thread, time};

/// MMR Runner
pub struct Runner<'r> {
    store: Store<'r>,
    conn: &'r SqliteConnection,
}

impl<'r> Runner<'r> {
    /// Start with sqlite3 conn
    pub fn with(conn: &'r SqliteConnection) -> Runner<'r> {
        let mut path = dirs::home_dir().unwrap_or_default();
        path.push(DEFAULT_RELATIVE_MMR_DB);
        let store = Store::with(conn);
        Runner { store, conn }
    }

    /// Start the runner
    pub fn start(&mut self) -> Result<(), Error> {
        match self.mmr_count() {
            Ok(mmr_count) => {
                let mut next = {
                    let last_leaf = helper::mmr_size_to_last_leaf(mmr_count);
                    if last_leaf == 0 {
                        0
                    } else {
                        last_leaf + 1
                    }
                };

                loop {
                    if let Err(e) = self.push(next) {
                        match e {
                            Error::Diesel(DieselError::NotFound) => {
                                trace!("Could not find block {:?} in cache", next)
                            }
                            _ => error!("Push block to mmr_store failed: {:?}", e),
                        }

                        trace!("MMR service restarting after 10s...");
                        thread::sleep(time::Duration::from_secs(10));
                        return self.start();
                    } else {
                        trace!("push eth block number {} into db succeed.", next);
                        next += 1;
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
            .first::<Cache>(self.conn)?;

        Ok(cache.hash)
    }

    /// Push new header hash into storage
    pub fn push(&mut self, pnumber: i64) -> Result<(), Error> {
        let cache = eth_header_with_proof_caches
            .filter(number.eq(pnumber))
            .first::<Cache>(self.conn)?;

        let mut mmr =
            MMR::<_, MergeHash, _>::new(self.mmr_count().unwrap_or(0) as u64, &self.store);
        if let Err(e) = mmr.push(H256::from(&cache.hash[2..])) {
            return Err(Error::MMR(e));
        }

        // eth_header_with_proof_caches
        let proot = mmr.get_root()?;
        diesel::update(eth_header_with_proof_caches.filter(number.eq(pnumber)))
            .set(root.eq(Some(H256::hex(&proot))))
            .execute(self.conn)?;

        mmr.commit()?;
        Ok(())
    }

    /// Get the count of mmr store
    fn mmr_count(&self) -> Result<i64, Error> {
        let res = mmr_store.select(count(elem)).first::<i64>(self.conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }
}
