//! Database Connection Pool
use diesel::{
    prelude::*,
    r2d2::{Builder, ConnectionManager, Pool, PooledConnection},
};
use std::path::PathBuf;

/// Connection Pool
pub type ConnPool = Pool<ConnectionManager<SqliteConnection>>;

/// Connection from Pool
pub type PooledConn = PooledConnection<ConnectionManager<SqliteConnection>>;

/// Constants
pub const DEFAULT_RELATIVE_MMR_DB: &str = ".darwinia/cache/shadow.db";

/// Connections
pub fn conn(p: Option<PathBuf>) -> ConnPool {
    let path = p.unwrap_or_else(|| dirs::home_dir().unwrap().join(DEFAULT_RELATIVE_MMR_DB));

    let op_dir = path.parent();
    if op_dir.is_none() {
        panic!("Wrong db path: {:?}", path);
    }

    let dir = op_dir.unwrap();
    if !dir.exists() {
        let res = std::fs::create_dir_all(dir);
        if res.is_err() {
            panic!("Create dir failed: {:?}", res);
        }
    }

    Builder::new()
        .max_size(1)
        .build(ConnectionManager::<SqliteConnection>::new(
            path.to_string_lossy(),
        ))
        .unwrap_or_else(|_| panic!("Error connecting to {:?}", &path))
}
