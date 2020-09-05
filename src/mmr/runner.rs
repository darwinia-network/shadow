//! MMR Runner
use crate::{
    chain::eth::EthHeaderRPCResp,
    mmr::{
        hash::{MergeHash, H256},
        helper,
        store::Store,
    },
    result::Error,
    ShadowShared,
};
use cmmr::MMR;
use reqwest::Client;
use rocksdb::{IteratorMode, DB};
use std::{env, sync::Arc, time};
use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

/// MMR Runner
#[derive(Clone)]
pub struct Runner {
    store: Store,
    db: Arc<DB>,
    client: Client,
}

impl From<ShadowShared> for Runner {
    fn from(s: ShadowShared) -> Self {
        Self {
            store: s.store,
            db: s.db,
            client: s.client,
        }
    }
}

impl Runner {
    /// Start the runner
    pub async fn start(&mut self) -> Result<(), Error> {
        let mut mmr_size = self.db.iterator(IteratorMode::Start).count() as u64;
        let last_leaf = helper::mmr_size_to_last_leaf(mmr_size as i64);
        let mut ptr =
            if last_leaf == 0 {
                0
            } else {
                last_leaf + 1
            };

        loop {
            debug!("-{}-{}------------", ptr, mmr_size);
            let a = now();
            match self.push(ptr, mmr_size).await {
                Err(e) => {
                    trace!("Push block to mmr_store failed: {:?}", e);
                    trace!("MMR service restarting after 10s...");
                    actix_rt::time::delay_for(time::Duration::from_secs(10)).await;
                },
                Ok(mmr_size_new) => {
                    if ptr
                        % env::var("MMR_LOG")
                        .unwrap_or_else(|_| "10000".to_string())
                        .parse::<i64>()
                        .unwrap_or(10000)
                        == 0
                    {
                        trace!("Pushed mmr {} into database", ptr);
                    }

                    mmr_size = mmr_size_new;
                    ptr += 1;
                }
            }
            debug!("total: {}", now() - a);
        }
    }

    /// Get block hash by number
    pub async fn get_hash(&mut self, block: i64) -> Result<String, Error> {
        Ok(EthHeaderRPCResp::get(&self.client, block as u64)
            .await?
            .result
            .hash)
    }

    /// Push new header hash into storage
    pub async fn push(&mut self, number: i64, mmr_size: u64) -> Result<u64, Error> {
        let a = now();
        let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &self.store);

        let b = now();
        debug!("mmr create  : {}", b - a);

        let hash_from_ethereum = &EthHeaderRPCResp::get(&self.client, number as u64)
            .await?
            .result
            .hash;

        let c = now();
        debug!("rpc call    : {}", c - b);

        mmr.push(H256::from(hash_from_ethereum))?;

        let d = now();
        debug!("push to mmr : {}", d - c);

        let mmr_size_new = mmr.mmr_size();

        let e = now();
        debug!("get new size: {}", e - d);

        mmr.commit()?;

        let f = now();
        debug!("commit      : {}", f - e);

        Ok(mmr_size_new)
    }

    /// Gen mmrs for tests
    pub async fn stops_at(&mut self, count: i64) -> Result<(), Error> {
        let mut mmr_size = self.db.iterator(IteratorMode::Start).count() as u64;
        let mut ptr = {
            let last_leaf = helper::mmr_size_to_last_leaf(mmr_size as i64);
            if last_leaf == 0 {
                0
            } else {
                last_leaf + 1
            }
        };

        loop {
            if ptr >= count {
                break;
            }
            if let Ok(mmr_size_new) = self.push(ptr, mmr_size).await {
                mmr_size = mmr_size_new;
                ptr += 1;
            }
        }

        Ok(())
    }
}
