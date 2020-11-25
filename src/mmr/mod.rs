//! Shdaow service mmr implementation
use crate::result::Result;
use mmr::{MmrClientTrait, MmrClientForMysql, MmrClientForRocksdb};
use rocksdb::DB;
use mysql::Pool;
use std::sync::Arc;

mod runner;

pub use runner::Runner;
pub use runner::ClientType;

/// Default uris
const DEFAULT_ROCKSDB_FILE: &str = ".shadow/cache/mmr";
// const DEFAULT_MYSQL_URI: &str = "mysql://root:@localhost:3306/mmr_store";

/// Build mmr client type
pub fn client_type(uri: Option<String>) -> Result<ClientType> {
    if let Some(uri) = uri {
        if uri.starts_with("mysql://") {
            let pool = Pool::new(uri)?;
            Ok(ClientType::Mysql(pool))
        } else {
            let db = DB::open_default(uri)?;
            Ok(ClientType::Rocksdb(Arc::new(db)))
        }
    } else {
        let path_buf = dirs::home_dir().unwrap().join(DEFAULT_ROCKSDB_FILE);
        let path = path_buf.to_str().unwrap().to_string();
        let db = DB::open_default(path)?;
        Ok(ClientType::Rocksdb(Arc::new(db)))
    }
}

/// convenient method to create mmr client
pub fn build_client(client_type: &ClientType) -> Result<Box<dyn MmrClientTrait>> {
    match client_type {
        ClientType::Mysql(pool) => {
            let client = MmrClientForMysql::new(pool.clone());
            Ok(Box::new(client))
        },
        ClientType::Rocksdb(db) => {
            let client = MmrClientForRocksdb::new(db.clone());
            Ok(Box::new(client))
        }
    }
}