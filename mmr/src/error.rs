use thiserror::Error;
use anyhow::Result as AnyResult;

#[derive(Error, Debug)]
pub enum MMRError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    CMMRError(#[from] cmmr::Error),

    #[error(transparent)]
    MysqlError(#[from] mysql::Error),

    #[error(transparent)]
    RocksdbError(#[from] rocksdb::Error),

    #[error(transparent)]
    ArrayBytesError(#[from] array_bytes::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

}

pub type Result<T> = AnyResult<T, MMRError>;