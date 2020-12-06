use crate::{result::Result};
use std::time::Duration;
use primitives::rpc::{RPC, EthereumRPC};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use anyhow::Context;
use crate::conf::{EPOCH_LOCK_FILE, EPOCH_BLOCK_FILE, EPOCH_INIT_BLOCK};

/// EpochRunner
pub struct EpochRunner {
    eth: Arc<EthereumRPC>,
}

impl EpochRunner {
    /// new
    pub fn new(eth: &Arc<EthereumRPC>) -> Self {
        EpochRunner { eth: eth.clone() }
    }

    pub async fn start(&mut self) {
        // remove lock if exist
        let lock_file = dirs::home_dir().unwrap().join(EPOCH_LOCK_FILE);
        if lock_file.exists() {
            tokio::fs::remove_file(lock_file).await.unwrap();
        }

        while let Err(err) = self.run().await {
            error!("{:?}", err);
            tokio::time::delay_for(Duration::from_millis(10)).await;
        }
    }

    async fn run(&mut self) -> Result<()> {
        let mut block = EpochRunner::read_from_file().await?;
        loop {
            let height = self.eth.block_number().await?;
            if block < height && EpochRunner::epoch(block).await? {
                block += 30000;
                EpochRunner::save_to_file(block).await?;
            }
        }
    }

    async fn read_from_file() -> Result<u64> {
        // read last block from file
        let block_file = dirs::home_dir().unwrap().join(EPOCH_BLOCK_FILE);
        if !block_file.exists() {
            Ok(EPOCH_INIT_BLOCK)
        } else {
            let mut file = File::open(block_file).await.context("Open block file failed")?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).await?;
            Ok(buffer.trim().parse().unwrap())
        }
    }

    async fn save_to_file(block: u64) -> Result<()> {
        let block_file = dirs::home_dir().unwrap().join(EPOCH_BLOCK_FILE);
        if !block_file.exists() {
            File::create(&block_file).await?;
        }

        tokio::fs::write(&block_file, block.to_string().as_bytes()).await?;
        Ok(())
    }

    async fn epoch(block: u64) -> Result<bool> {
        trace!("epoch {}", block/30000);

        let epoch_result: bool = tokio::task::spawn_blocking(move || {
            ffi::epoch(block)
        }).await?;

        Ok(epoch_result)
    }
}

