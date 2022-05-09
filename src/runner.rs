//! Runner
use crate::result::Result;
use primitives::rpc::{Rpc, EthereumRPC};
use std::time::Duration;
use std::sync::Arc;

/// Runner
pub struct Runner {
    eth: Arc<EthereumRPC>,
}

impl Runner {
    /// new
    pub fn new(eth: &Arc<EthereumRPC>) -> Self {
        Runner { eth: eth.clone() }
    }

    /// Start the runner
    pub async fn start(&self) {
        while let Err(err) = self.run().await {
            error!("{:?}", err);
            tokio::time::delay_for(Duration::from_secs(10)).await;
        }
    }

    /// Run
    pub async fn run(&self) -> Result<()> {
        // Using a cache rpc block number to optimize and reduce rpc call.
        let mut last_rpc_block_number = self.eth.block_number().await?;
        ffi::start(last_rpc_block_number/30_000);

        let mut epoched = last_rpc_block_number;

        loop {
            let block_number = self.eth.block_number().await?;

            // checking finalization, run too fast
            if block_number >= (last_rpc_block_number + 100u64) {
                last_rpc_block_number = block_number;

                if (last_rpc_block_number/30_000 * 30_000) as u64 + 30_000 > epoched && ffi::epoch(epoched) {
                    epoched += 30_000;

                    tokio::time::delay_for(Duration::from_secs(300)).await;
                }
            }

            tokio::time::delay_for(Duration::from_secs(10)).await;
        }
    }
}


