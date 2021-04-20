//! MMR Errors
use thiserror::Error as ThisError;
use anyhow::Result as AnyResult;
#[allow(missing_docs)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    RocksdbError(#[from] rocksdb::Error),

    #[error(transparent)]
    MysqlError(#[from] mysql::Error),

    #[error(transparent)]
    MmrError(#[from] mmr::Error),

    #[error(transparent)]
    PrimitivesError(#[from] primitives::result::Error),

    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    ApiError(#[from] api::Error),

    #[error("{0}")]
    Shadow(String),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

/// Sup Result
pub type Result<T> = AnyResult<T, Error>;
