//! Shdaow service mmr implementation
use crate::result::Result;
use crate::conf::DEFAULT_ROCKSDB_FILE;
use mmr::Database;
use rocksdb::DB;
use mysql::Pool;
use std::sync::Arc;

mod runner;

pub use runner::Runner;

/// Build mmr client type
pub fn database(uri: Option<String>) -> Result<Database> {
    if let Some(uri) = uri {
        if uri.starts_with("mysql://") {
            let pool = Pool::new(uri)?;
            Ok(Database::Mysql(pool))
        } else {
            let db = DB::open_default(uri)?;
            Ok(Database::Rocksdb(Arc::new(db)))
        }
    } else {
        let path_buf = dirs::home_dir().unwrap().join(DEFAULT_ROCKSDB_FILE);
        let path = path_buf.to_str().unwrap().to_string();
        let db = DB::open_default(path)?;
        Ok(Database::Rocksdb(Arc::new(db)))
    }
}