//! Shdaow service mmr implementation
use crate::result::Result;
use crate::conf::DEFAULT_ROCKSDB_FILE;
use mmr::Database;
use rocksdb::DB;
use mysql::Pool;
use std::sync::Arc;

mod runner;

pub use runner::Runner;
use mysql::prelude::Queryable;

/// Build mmr client type
pub fn database(uri: Option<String>) -> Result<Database> {
    if let Some(uri) = uri {
        if uri.starts_with("mysql://") {
            let pool = Pool::new(uri)?;
            create_table_if_not_exists(pool.clone())?;
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

fn create_table_if_not_exists(pool: Pool) -> Result<()> {
    let mut conn = pool.get_conn()?;
    let create_table_statement = "CREATE TABLE `mmr` (
                    `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                    `position` int(11) DEFAULT NULL,
                    `hash` char(64) DEFAULT NULL,
                    `leaf` tinyint(11) DEFAULT '0',
                    `leaf_index` int(11) DEFAULT NULL,
                    `root` char(64) DEFAULT NULL,
                    `height` int(11) DEFAULT NULL,
                    PRIMARY KEY (`id`),
                    UNIQUE KEY `position` (`position`),
                    UNIQUE KEY `hash` (`hash`)
                ) ENGINE=InnoDB AUTO_INCREMENT=96 DEFAULT CHARSET=utf8;";
    if let Err(err) = conn.query_first::<u64, _>("SELECT 1 FROM mmr") {
        if err.to_string().contains(".mmr' doesn't exist") {
            conn.query_drop(create_table_statement)?;
        } else {
            Err(err)?;
        }
    }
    Ok(())
}

