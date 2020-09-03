use crate::mmr::Store;
use reqwest::Client;
use rocksdb::DB;
use std::{path::PathBuf, sync::Arc};

/// Constants
const DEFAULT_RELATIVE_MMR_DB: &str = ".darwinia/cache/mmr";

/// Shadow shared data
#[derive(Clone)]
pub struct ShadowShared {
    /// MMR Store
    pub store: Store,
    /// HTTP client
    pub client: Client,
    /// RocksDB
    pub db: Arc<DB>,
}

impl ShadowShared {
    /// New shared data
    pub fn new(p: Option<PathBuf>) -> ShadowShared {
        let path = p.unwrap_or_else(|| dirs::home_dir().unwrap().join(DEFAULT_RELATIVE_MMR_DB));
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

        let res = DB::open_default(path.to_owned());
        if res.is_err() {
            panic!("Could not open dir {:?}", &path);
        }

        let db = Arc::new(res.unwrap());
        ShadowShared {
            db: db.clone(),
            store: Store::with(db),
            client: Client::new(),
        }
    }
}
