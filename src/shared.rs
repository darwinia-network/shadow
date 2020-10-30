use crate::mmr::Store;
use primitives::rpc::EthereumRPC;
use reqwest::Client;
use rocksdb::DB;
use std::{env, path::PathBuf, sync::Arc, thread, time};

/// Constants
const DEFAULT_RELATIVE_MMR_DB: &str = ".darwinia/cache/mmr";

/// Shadow shared data
#[derive(Clone)]
pub struct ShadowShared {
    /// MMR Store
    pub store: Store,
    /// RocksDB
    pub db: Arc<DB>,
    /// Ethereum rpc
    pub eth: Arc<EthereumRPC>,
}

fn ethereum_rpc(http: Client) -> EthereumRPC {
    let rpcs = env::var("ETHEREUM_RPC")
        .unwrap_or_else(|_| {
            if env::var("ETHEREUM_ROPSTEN").is_ok() {
                crate::conf::DEFAULT_ETHEREUM_ROPSTEN_RPC.into()
            } else {
                crate::conf::DEFAULT_ETHEREUM_RPC.into()
            }
        })
        .split(',')
        .map(|s| s.to_string())
        .collect();

    info!("Avaiable ethereum rpc urls: \n{:#?}", rpcs);
    EthereumRPC::new(http, rpcs)
}

impl ShadowShared {
    /// New shared data
    pub fn new(p: Option<PathBuf>) -> ShadowShared {
        let path = p
            .clone()
            .unwrap_or_else(|| dirs::home_dir().unwrap().join(DEFAULT_RELATIVE_MMR_DB));
        let op_dir = path.parent();
        if op_dir.is_none() {
            panic!("Wrong db path: {:?}", &path);
        }

        let dir = op_dir.unwrap();
        if !dir.exists() {
            let res = std::fs::create_dir_all(dir);
            if res.is_err() {
                panic!("Create dir failed: {:?}", res);
            }
        }

        match DB::open_default(path.to_owned()) {
            Err(e) => {
                let msg = e.into_string();
                if msg.contains("lock") {
                    thread::sleep(time::Duration::from_secs(1));
                    Self::new(p)
                } else {
                    panic!("Could not open dir {:?}, {:?}", &path, msg);
                }
            }
            Ok(rocks) => {
                let db = Arc::new(rocks);
                ShadowShared {
                    db: db.clone(),
                    store: Store::with(db),
                    eth: Arc::new(ethereum_rpc(Client::new())),
                }
            }
        }
    }
}
