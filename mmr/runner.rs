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
        trace!(
            "The database path of shadow service is: {}",
            DEFAULT_RELATIVE_MMR_DB
        );

        Runner { path }
    }
}

impl Runner {
    /// Start the runner
    pub fn start(&mut self) -> Result<(), Error> {
        match self.mmr_count() {
            Ok(mut base) => loop {
                if let Err(e) = self.push(base) {
                    match e {
                        Error::Diesel(DieselError::NotFound) => {
                            warn!("Could not find block: {:?} in cache", base)
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
            },
            Err(e) => {
                error!("Get mmr count failed, {:?}", e);
                trace!("MMR service sleep for 3s...");
                thread::sleep(time::Duration::from_secs(3));
                self.start()
            }
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

    /// Push new header hash into storage
    fn push(&mut self, pnumber: i64) -> Result<(), Error> {
        let store = Store::new(&self.path);
        // Get Hash
        let conn = store.conn();
        let cache = eth_header_with_proof_caches
            .filter(number.eq(pnumber))
            .first::<Cache>(&store.conn)?;

        let mut mmr = MMR::<_, MergeHash, _>::new(self.mmr_count().unwrap_or(0) as u64, store);
        if let Err(e) = mmr.push(H256::from(&cache.hash[2..])) {
            return Err(Error::MMR(e));
        }

        // eth_header_with_proof_caches
        let proot = mmr.get_root()?;
        diesel::update(eth_header_with_proof_caches.filter(number.eq(pnumber)))
            .set(root.eq(Some(H256::hex(&proot))))
            .execute(&conn)?;

        mmr.commit()?;
        Ok(())
    }
}
