use anyhow::Result as AnyResult;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    PrimitivesError(#[from] shadow_types::result::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ScaleCodecError(#[from] codec::Error),

    #[error("ArrayBytes: {0}")]
    ArrayBytesError(String),

    #[error("Leaf of index {0} is not found")]
    LeafNotFound(u64),

    #[error("Mmr root of leaf {0} is not found")]
    MmrRootNotFound(u64),
}

impl From<array_bytes::Error> for Error {
    fn from(e: array_bytes::Error) -> Self {
        Self::ArrayBytesError(format!("{:?}", e))
    }
}

/// Error Json
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorJson {
    /// MMR leaf string
    pub error: String,
}

impl Error {
    pub fn to_json(&self) -> ErrorJson {
        ErrorJson {
            error: self.to_string(),
        }
    }
}

pub type Result<T> = AnyResult<T, Error>;
