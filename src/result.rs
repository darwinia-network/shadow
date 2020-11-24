//! MMR Errors
use thiserror::Error as ThisError;
use anyhow::Result as AnyResult;

#[allow(missing_docs)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Primitive(#[from] primitives::result::Error),

    #[error("{0}")]
    Shadow(String),
}

/// Sup Result
pub type Result<T> = AnyResult<T>;
