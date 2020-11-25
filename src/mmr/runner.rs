//! MMR Runner
use mmr::{
    MmrClientTrait, MmrClientForRocksdb, MmrClientForMysql, H256
};
use crate::result::Result;
use primitives::rpc::{RPC, EthereumRPC};
use std::time::Duration;
use mysql::Pool;
use std::sync::Arc;
use rocksdb::DB;

/// Constants
const DEFAULT_ROCKSDB_FILE: &str = ".darwinia/cache/mmr";
const DEFAULT_MYSQL_URI: &str = "mysql://root:@localhost:3306/mmr_store";

/// Client type with diff mmr store
#[derive(Clone)]
pub enum ClientType {
    /// client with rocksdb store
    Rocksdb,
    /// client with mysql store
    Mysql,
}

/// MMR Runner
pub struct Runner {
    eth: Arc<EthereumRPC>,
    client_type: ClientType,
}

impl Runner {
    /// new
    pub fn new(eth: Arc<EthereumRPC>, client_type: ClientType) -> Self {
        Runner { eth, client_type }
    }

    /// Start the runner
    pub async fn start(&self) -> Result<()> {
        let client = Runner::build_client(&self.client_type)?;
        let mmr_size = client.get_mmr_size().unwrap();

        // MMR variables
        info!("last mmr size {}", mmr_size);

        // Check if the correct ethereum node is connected
        if mmr_size > 0 {
            let valid_hash = client.get_elem(0)?;
            let hash_from_ethereum = self.eth.get_header_by_number(0).await?.hash;

            if let Some(hash) = hash_from_ethereum {
                let rpc_hash = H256::hex(&hash);
                assert_eq!(valid_hash, rpc_hash, "RPC network should be {} but {}", Runner::network_name(&valid_hash), Runner::network_name(&rpc_hash));
            } else {
                panic!("rpc request is unreachable");
            }
        }

        // Leaf index to push into mmr store
        let mut ptr: u64 =
            match client.get_last_leaf_index()? {
                Some(last_leaf_index) => last_leaf_index + 1,
                None => 0
            };

        // Using a cache rpc block number to optimize and reduce rpc call.
        let mut last_rpc_block_number = self.eth.block_number().await?;

        loop {
            // checking finalization, run too fast
            if last_rpc_block_number < (ptr as u64 + 12) {
                trace!("Pause 10s due to finalization checking, prepare to push block {}, last block number from rpc is {}", ptr, last_rpc_block_number);
                tokio::time::delay_for(Duration::from_millis(10)).await;
                last_rpc_block_number = self.eth.block_number().await?;
                continue;
            }

            let hash_from_ethereum = self.eth.get_header_by_number(ptr).await?.hash;

            if let Some(hash) = hash_from_ethereum {
                let leaf = H256::hex(&hash);
                let client_type = self.client_type.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let mut client = Runner::build_client(&client_type)?;
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

    fn build_client(client_type: &ClientType) -> Result<Box<dyn MmrClientTrait>> {
        match client_type {
            ClientType::Mysql => {
                let db = Pool::new(DEFAULT_MYSQL_URI.to_string())?;
                let client = MmrClientForMysql::new(db);
                Ok(Box::new(client))
            },
            ClientType::Rocksdb => {
                let db = DB::open_default(DEFAULT_ROCKSDB_FILE.to_string())?;
                let client = MmrClientForRocksdb::new(Arc::new(db));
                Ok(Box::new(client))
            }
        }
    }
}


