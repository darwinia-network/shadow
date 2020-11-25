//! Shdaow service mmr implementation
use crate::result::Result;
use mmr::{MmrClientTrait, MmrClientForMysql, MmrClientForRocksdb};
use mysql::Pool;
use rocksdb::DB;
use std::sync::Arc;

mod runner;

pub use runner::Runner;

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

/// convenient method to create mmr client
pub fn build_client(client_type: ClientType) -> Result<Box<dyn MmrClientTrait>> {
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
