//! MMR Runner
use crate::result::Result;
use mmr::{build_client, Database, MmrClientTrait, H256};
use primitives::rpc::{EthereumRPC, RPC};
use std::sync::Arc;
use std::time::Duration;

/// MMR Runner
pub struct Runner {
    eth: Arc<EthereumRPC>,
    database: Database,
}

impl Runner {
    /// new
    pub fn new(eth: &Arc<EthereumRPC>, database: &Database) -> Self {
        Runner {
            eth: eth.clone(),
            database: database.clone(),
        }
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
        let client = build_client(&self.database)?;

        // check network
        let network = self.check_network(client.as_ref()).await?;
        let mut epoch_length = 30_000;
        let delay_blocks = {
            if &network == "Mainnet" {
                12u64
            } else if network.starts_with("Heco") || network.starts_with("BSC") {
                epoch_length = 200;
                11u64
            } else {
                100u64
            }
        };

        // Leaf index to push into mmr store
        let mut ptr: u64 = client.get_leaf_count()?;
        info!("Start from leaf index: {}", ptr);

        // Using cached rpc block number to optimize and reduce rpc call.
        let mut last_rpc_block_number = self.eth.block_number().await?;
        if !network.starts_with("Heco") && !network.starts_with("BSC") {
            ffi::start(ptr / epoch_length);
        }
        let mut epoched = ptr;

        loop {
            // checking finalization, run too fast
            if last_rpc_block_number < (ptr + delay_blocks) {
                trace!("Pause 10s due to finalization checking, prepare to push block {}, last block number from rpc is {}", ptr, last_rpc_block_number);
                tokio::time::delay_for(Duration::from_secs(10)).await;
                last_rpc_block_number = self.eth.block_number().await?;
                continue;
            }

            if (ptr / epoch_length * epoch_length) as u64 + epoch_length > epoched {
                // notify epoch change
                if &network == "Mainnet" {
                    ffi::epoch(epoched);
                }
                epoched += epoch_length;
            }
            let hash_from_ethereum = self.eth.get_header_by_number(ptr).await?.hash;

            if let Some(hash) = hash_from_ethereum {
                let client_type = self.database.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let mut client = build_client(&client_type)?;
                    client.push(&hash)
                })
                .await?;

                let leaf = H256::hex(&hash);
                match result {
                    Ok(position) => {
                        trace!("Pushed leaf {}: {} at position {}", ptr, leaf, position);
                        ptr += 1;
                    }
                    Err(err) => error!("Failed to push {}: {:?}", leaf, err),
                }
            } else {
                warn!("Ethereum block hash of {} is none", ptr);
                tokio::time::delay_for(Duration::from_secs(10)).await;
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
                return Err(anyhow::anyhow!(
                    "RPC network should be {} but {}",
                    network_of_mmr,
                    network_of_rpc
                )
                .into());
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
            "5751d1772ebc82d52d19d96157bb3f13ca8417217e3c0913adf15f04eb4cb144" => "HecoMainnet",
            "b24b1124276b1250ad3b2c02623677bce3e76c1539f76dcdfe4c27ab991c1dad" => "HecoTestnet",
            "0d21840abff46b96c84b2ac9e10e4f5cdaeb5693cb665db62a2f3b02d2d57b5b" => "BSCMainnet",
            "6d3c66c5357ec91d5c43af47e234a939b22557cbb552dc45bebbceeed90fbe34" => "BSCTestnet",
            _ => "unknown",
        }
    }
}
