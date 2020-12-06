//! MMR Runner
use mmr::{H256, Database, build_client, MmrClientTrait};
use crate::result::Result;
use primitives::rpc::{RPC, EthereumRPC};
use std::time::Duration;
use std::sync::Arc;

/// MMR Runner
pub struct Runner {
    eth: Arc<EthereumRPC>,
    database: Database,
}

impl Runner {
    /// new
    pub fn new(eth: &Arc<EthereumRPC>, database: &Database) -> Self {
        Runner { eth: eth.clone(), database: database.clone() }
    }

    /// Start the runner
    pub async fn start(&self) {
        while let Err(err) = self.run().await {
            error!("{:?}", err);
            tokio::time::delay_for(Duration::from_millis(10)).await;
        }
    }

    /// Run
    pub async fn run(&self) -> Result<()> {
        let client = build_client(&self.database)?;

        // check network
        let network = self.check_network(client.as_ref()).await?;
        let delay_blocks = if &network == "Mainnet" { 12u64 } else { 100u64 };

        // Leaf index to push into mmr store
        let mut ptr: u64 = client.get_leaf_count()?;
        info!("Start from leaf index: {}", ptr);

        // Using a cache rpc block number to optimize and reduce rpc call.
        let mut last_rpc_block_number = self.eth.block_number().await?;

        loop {
            // checking finalization, run too fast
            if last_rpc_block_number < (ptr + delay_blocks) {
                trace!("Pause 10s due to finalization checking, prepare to push block {}, last block number from rpc is {}", ptr, last_rpc_block_number);
                tokio::time::delay_for(Duration::from_millis(10)).await;
                last_rpc_block_number = self.eth.block_number().await?;
                continue;
            }

            let hash_from_ethereum = self.eth.get_header_by_number(ptr).await?.hash;

            if let Some(hash) = hash_from_ethereum {
                let leaf = H256::hex(&hash);
                let client_type = self.database.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let mut client = build_client(&client_type)?;
                    client.push(&leaf)
                }).await?;

                let leaf = H256::hex(&hash);
                match result {
                    Ok(position) => {
                        trace!("Pushed leaf {}: {} at position {}", ptr, leaf, position);
                        ptr += 1;
                    },
                    Err(err) => error!("Failed to push {}: {:?}", leaf, err)
                }
            } else {
                warn!("Ethereum block hash of {} is none", ptr);
                tokio::time::delay_for(Duration::from_millis(10)).await;
            }
        }
    }

    async fn check_network(&self, client: &dyn MmrClientTrait) -> Result<String> {
        let hash_from_ethereum = self.eth.get_header_by_number(0).await?.hash.unwrap();
        let hash_of_rpc = H256::hex(&hash_from_ethereum);
        let network_of_rpc = Runner::network_name(&hash_of_rpc).to_string();

        if let Some(first_leaf) = client.get_leaf(0)? {
            if first_leaf != hash_of_rpc {
                let network_of_mmr = Runner::network_name(&first_leaf);
                return Err(anyhow::anyhow!("RPC network should be {} but {}", network_of_mmr, network_of_rpc).into());
            }
        };

        Ok(network_of_rpc)
    }

    /// translate from hash to network name
    fn network_name(h: &str) -> &str {
        match h {
            "41941023680923e0fe4d74a34bdac8141f2540e3ae90623718e47d66d1ca4a2d" => "Ropsten",
            "d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3" => "Mainnet",
            "6341fd3daf94b748c72ced5a5b26028f2474f5f00d824504e4fa37a75767e177" => "Rinkeby",
            "bf7e331f7f7c1dd2e05159666b3bf8bc7a8a3a9eb1d518969eab529dd9b88c1a" => "Goerli",
            _ => "unknown",
        }
    }


}


