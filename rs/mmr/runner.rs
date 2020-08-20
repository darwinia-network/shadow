//! MMR Runner
use crate::{
    hash::{MergeHash, H256},
    helper,
    model::Cache,
    pool::{ConnPool, PooledConn},
    result::Error,
    schema::{eth_header_with_proof_caches::dsl::*, mmr_store::dsl::*},
    store::Store,
};
use cmmr::{Error as StoreError, MMR};
use diesel::{dsl::count, prelude::*, result::Error as DieselError};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread, time,
};

/// MMR Runner
#[derive(Clone)]
pub struct Runner {
    store: Store,
    pool: ConnPool,
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
        Runner { store, pool }
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
            trace!("push mmr of eth block {} into db succeed.", cur);
            cur + 1
        }
    }

    /// Start the runner
    pub fn start(&mut self, thread: i64, limits: u32) -> Result<(), Error> {
        let mut ptr = {
            let last_leaf = helper::mmr_size_to_last_leaf(self.mmr_count()?);
            if last_leaf == 0 {
                0
            } else {
                last_leaf + 1
            }
        };

        let gap: time::Duration = time::Duration::from_secs(1) / limits;
        loop {
            let next = Arc::new(Mutex::new(ptr));
            let (tx, rx) = mpsc::channel();
            for _ in 0..thread {
                let (next, tx) = (Arc::clone(&next), tx.clone());
                let mut this = self.clone();
                thread::spawn(move || {
                    let mut cur = next.lock().unwrap();
                    *cur = this.check_push(*cur);
                    thread::sleep(gap);
                    if *cur == thread + ptr {
                        tx.send(*cur).unwrap();
                    }
                });
            }

            ptr = rx.recv().unwrap();
        }
    }

    /// Get block hash by number
    pub fn get_hash(&mut self, block: i64) -> Result<String, Error> {
        let cache = eth_header_with_proof_caches
            .filter(number.eq(block))
            .first::<Cache>(&self.conn()?)?;

        Ok(cache.hash)
    }

    /// Push new header hash into storage
    pub fn push(&mut self, pnumber: i64) -> Result<(), Error> {
        let conn = self.conn()?;
        let cache = eth_header_with_proof_caches
            .filter(number.eq(pnumber))
            .first::<Cache>(&conn)?;

        let mut mmr =
            MMR::<_, MergeHash, _>::new(self.mmr_count().unwrap_or(0) as u64, self.store.clone());
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

    /// Get the count of mmr store
    fn mmr_count(&self) -> Result<i64, Error> {
        let conn = self.conn()?;
        let res = mmr_store.select(count(elem)).first::<i64>(&conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }
}
