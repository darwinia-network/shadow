#[macro_use]
extern crate log;

use mysql::Pool;
use std::sync::Arc;
use rocksdb::DB;

mod error;
mod helper;
mod mysql_store;
mod rocksdb_store;
mod rocksdb_batchstore;
mod hash;
mod mmr_client_for_mysql;
mod mmr_client_trait;
mod mmr_client_for_rocksdb;

pub use self::{
    error::{Result, MMRError as Error},
    hash::{MergeHash, H256},
    rocksdb_store::RocksdbStore,
    rocksdb_batchstore::RocksBatchStore,
    mysql_store::MysqlStore,
    helper::mmr_size_to_last_leaf,
    mmr_client_trait::MmrClientTrait,
    mmr_client_for_mysql::MmrClientForMysql,
    mmr_client_for_rocksdb::MmrClientForRocksdb,
};

pub use cmmr::MerkleProof;
pub use cmmr::leaf_index_to_mmr_size;
pub use cmmr::leaf_index_to_pos;

/// Client type with diff mmr store
#[derive(Clone)]
pub enum Database {
    /// client with rocksdb store
    Rocksdb(Arc<DB>),
    /// client with mysql store
    Mysql(Pool),
}

/// convenient method to create mmr client
pub fn build_client(database: &Database) -> Result<Box<dyn MmrClientTrait>> {
    match database {
        Database::Mysql(pool) => {
            let client = MmrClientForMysql::new(pool.clone());
            Ok(Box::new(client))
        },
        Database::Rocksdb(db) => {
            let client = MmrClientForRocksdb::new(db.clone());
            Ok(Box::new(client))
        }
    }
}
