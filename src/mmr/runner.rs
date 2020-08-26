//! MMR Runner
use crate::{
    chain::eth::EthHeaderRPCResp,
    hash::{MergeHash, H256},
    helper,
    pool::{ConnPool, PooledConn},
    result::Error,
    schema::mmr_store::dsl::*,
    store::Store,
};
use cmmr::{Error as StoreError, MMR};
use diesel::{dsl::count, prelude::*, result::Error as DieselError};
use reqwest::blocking::Client;
use std::{thread, time};

/// MMR Runner
#[derive(Clone)]
pub struct Runner {
    store: Store,
    pool: ConnPool,
    client: Client,
}

impl Runner {
    /// Get Pooled connection
    pub fn conn(&self) -> Result<PooledConn, Error> {
        let cfp = self.pool.get();
        if cfp.is_err() {
            return Err(Error::Custom("Connect to database failed".into()));
        }

        Ok(cfp.unwrap())
    }

    /// Start with sqlite3 conn
    pub fn with(pool: ConnPool) -> Runner {
        let store = Store::with(pool.clone());
        Runner {
            store,
            pool,
            client: Client::new(),
        }
    }

    fn check_push(&mut self, cur: i64) -> i64 {
        if let Err(e) = self.push(cur) {
            let mut locked = |m: &str| {
                if m.contains("database is locked") {
                    warn!("Database if locked, retry after 100ms...");
                    thread::sleep(time::Duration::from_millis(100));
                    Some(self.check_push(cur))
                } else {
                    error!("{:?}", m);
                    None
                }
            };
            match e {
                Error::Diesel(DieselError::NotFound) => {
                    trace!("Could not find block {:?} in cache", cur)
                }
                Error::Diesel(DieselError::DatabaseError(_, e)) => {
                    if let Some(r) = locked(e.message()) {
                        return r;
                    }
                }
                Error::MMR(StoreError::StoreError(e)) => {
                    if let Some(r) = locked(&e) {
                        return r;
                    }
                }
                _ => error!("Push block to mmr_store failed: {:?}", e),
            }

            trace!("MMR service restarting after 10s...");
            thread::sleep(time::Duration::from_secs(10));
            self.check_push(cur)
        } else {
            if cur % 10000 == 0 {
                trace!("current mmr height: {}", cur);
            }

            cur + 1
        }
    }

    /// Start the runner
    pub fn start(&mut self) -> Result<(), Error> {
        let mut ptr = {
            let last_leaf = helper::mmr_size_to_last_leaf(self.mmr_count()?);
            if last_leaf == 0 {
                0
            } else {
                last_leaf + 1
            }
        };

        loop {
            self.check_push(ptr);
            ptr += 1;
        }
    }

    /// Get block hash by number
    pub fn get_hash(&mut self, block: i64) -> Result<String, Error> {
        Ok(EthHeaderRPCResp::get(&self.client, block as u64)?
            .result
            .hash)
    }

    /// Trim mmr
    pub fn trim(&mut self, leaf: u64) -> Result<(), Error> {
        let mpos = cmmr::leaf_index_to_pos(leaf);
        let conn = self.conn()?;
        diesel::delete(mmr_store.filter(pos.ge(mpos as i64))).execute(&conn)?;
        Ok(())
    }

    /// Push new header hash into storage
    pub fn push(&mut self, pnumber: i64) -> Result<(), Error> {
        let mut mmr = MMR::<_, MergeHash, _>::new(
            cmmr::leaf_index_to_mmr_size((pnumber - 1) as u64),
            &self.store,
        );
        if let Err(e) = mmr.push(H256::from(
            &EthHeaderRPCResp::get(&self.client, pnumber as u64)?
                .result
                .hash,
        )) {
            return Err(Error::MMR(e));
        }

        mmr.commit()?;
        Ok(())
    }

    /// Get the count of mmr store
    pub fn mmr_count(&self) -> Result<i64, Error> {
        let conn = self.conn()?;
        let res = mmr_store.select(count(elem)).first::<i64>(&conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }
}
