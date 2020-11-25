#[macro_use]
extern crate log;

mod error;
mod helper;
mod mysql_store;
mod rocksdb_store;
mod hash;
mod mmr_client_for_mysql;
mod mmr_client_trait;
mod mmr_client_for_rocksdb;

pub use self::{
    error::{Result, MMRError},
    hash::{MergeHash, H256},
    rocksdb_store::RocksdbStore,
    mysql_store::MysqlStore,
    helper::mmr_size_to_last_leaf,
    mmr_client_trait::MmrClientTrait,
    mmr_client_for_mysql::MmrClientForMysql,
    mmr_client_for_rocksdb::MmrClientForRocksdb,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
