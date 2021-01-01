use crate::mmr::{ Store, BatchStore };
use primitives::rpc::EthereumRPC;
use reqwest::Client;
use rocksdb::DB;
use std::{env, path::PathBuf, sync::Arc, thread, time};
use std::time::Duration;

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

/// Shadow db data, unsafe
pub struct ShadowUnsafe {
    /// MMR batch store
    pub bstore: BatchStore,
    /// RocksDB
    pub db: Arc<DB>,
}

fn ethereum_rpc(http: Client) -> EthereumRPC {
    let rpcs = env::var("ETHEREUM_RPC")
        .unwrap_or_else(|_| {
            "http://localhost:8545".into()
        })
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    info!("Avaiable ethereum rpc urls: \n{:#?}", rpcs);
    EthereumRPC::new(http, rpcs)
}

fn db_path(p: &Option<PathBuf>) -> PathBuf {
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
    path
}

impl ShadowShared {
    /// New shared data
    pub fn new(p: Option<PathBuf>) -> ShadowShared {
        let path = db_path(&p);
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
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(30))
                    .build().unwrap();
                ShadowShared {
                    db: db.clone(),
                    store: Store::with(db),
                    eth: Arc::new(ethereum_rpc(client)),
                }
            }
        }
    }
}

impl ShadowUnsafe {
    /// New unsafe shadow data
    pub fn new(p: Option<PathBuf>) -> ShadowUnsafe {
        let path = db_path(&p);
        match DB::open_default(path.to_owned()) {
            Err(e) => {
                panic!("open db failed! dir {:?} {:?}", &path, e);
            }
            Ok(rocks) => {
                let db = Arc::new(rocks);
                ShadowUnsafe {
                    db: db.clone(),
                    bstore: BatchStore::with(db),
                }
            }
        }
    }
}

