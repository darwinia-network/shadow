//! MMR Runner
use crate::{
    api::eth::epoch,
    mmr::{
        hash::{MergeHash, H256},
        helper,
    },
    result::Error,
    ShadowShared,
};
use cmmr::MMR;
use primitives::rpc::ethereum::EthHeaderRPCResp;
use rocksdb::IteratorMode;
use std::{env, thread, time};

/// MMR Runner
#[derive(Clone)]
pub struct Runner(ShadowShared);

impl AsRef<ShadowShared> for Runner {
    fn as_ref(&self) -> &ShadowShared {
        &self.0
    }
}

impl AsMut<ShadowShared> for Runner {
    fn as_mut(&mut self) -> &mut ShadowShared {
        &mut self.0
    }
}

impl From<ShadowShared> for Runner {
    fn from(s: ShadowShared) -> Self {
        Self(s)
    }
}

impl Runner {
    /// Async epoch
    pub fn epoch(block: u64) {
        if !epoch(block) {
            thread::sleep(time::Duration::from_secs(10));
            Self::epoch(block);
        }
    }

    /// Start the runner
    pub async fn start(&mut self) -> Result<(), Error> {
        let mut mmr_size = self.as_mut().db.iterator(IteratorMode::Start).count() as u64;
        let last_leaf = helper::mmr_size_to_last_leaf(mmr_size as i64);
        let mut ptr = if last_leaf == 0 { 0 } else { last_leaf + 1 };

        loop {
            // Note:
            //
            // This trigger is ungly, need better solution in the future
            if ptr % 30000 == 0 {
                thread::spawn(move || Self::epoch(ptr as u64))
                    .join()
                    .unwrap_or_default();
            }

            match self.push(ptr, mmr_size).await {
                Err(_e) => {
                    actix_rt::time::delay_for(time::Duration::from_secs(10)).await;
                }
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
        }
    }

    /// Push new header hash into storage
    pub async fn push(&mut self, number: i64, mmr_size: u64) -> Result<u64, Error> {
        let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &self.as_ref().store);
        let hash_from_ethereum = &EthHeaderRPCResp::get(&self.0.client, &self.0.eth, number as u64)
            .await?
            .result
            .hash;

        mmr.push(H256::from(hash_from_ethereum))?;
        let mmr_size_new = mmr.mmr_size();

        mmr.commit()?;
        Ok(mmr_size_new)
    }
}
