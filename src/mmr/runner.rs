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
        let mut ptr = {
            let last_leaf =
                helper::mmr_size_to_last_leaf(self.db.iterator(IteratorMode::Start).count() as i64);
            if last_leaf == 0 {
                0
            } else {
                last_leaf + 1
            }
        };

        loop {
            if let Err(e) = self.push(ptr).await {
                trace!("Push block to mmr_store failed: {:?}", e);
                trace!("MMR service restarting after 10s...");
                actix_rt::time::delay_for(time::Duration::from_secs(10)).await;
            } else {
                if ptr
                    % env::var("MMR_LOG")
                        .unwrap_or_else(|_| "10000".to_string())
                        .parse::<i64>()
                        .unwrap_or(10000)
                    == 0
                {
                    trace!("Pushed mmr {} into database", ptr);
                }

                ptr += 1;
            }
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
    pub async fn push(&mut self, number: i64) -> Result<(), Error> {
        let mmr_size = if number == 0 {
            0
        } else {
            cmmr::leaf_index_to_mmr_size((number - 1) as u64)
        } as u64;
        let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &self.store);
        mmr.push(H256::from(
            &EthHeaderRPCResp::get(&self.client, number as u64)
                .await?
                .result
                .hash,
        ))?;

        mmr.commit()?;
        Ok(())
    }

    /// Gen mmrs for tests
    pub async fn stops_at(&mut self, count: i64) -> Result<(), Error> {
        let mmr_size = self.db.iterator(IteratorMode::Start).count() as i64;
        let mut ptr = {
            let last_leaf = helper::mmr_size_to_last_leaf(mmr_size);
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
            self.push(ptr).await?;
            ptr += 1;
        }

        Ok(())
    }
}
