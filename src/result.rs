//! MMR Errors
use cmmr::Error as MMRError;
use reqwest::Error as ReqwestError;
use rocksdb::Error as RocksdbError;
use serde_json::Error as SerdeJSONError;
use std::io::Error as IoError;

/// MMR Errors
#[derive(Debug)]
pub enum Error {
    /// Io Error
    Io(IoError),
    /// MMR Error
    MMR(MMRError),
    /// Reqwest Error
    Reqwest(ReqwestError),
    /// Reqwest Error
    SerdeJSON(SerdeJSONError),
    /// Reqwest Error
    RocksdbError(RocksdbError),
    /// Custom
    Custom(String),
}

impl From<MMRError> for Error {
    fn from(e: MMRError) -> Error {
        Error::MMR(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Error {
        Error::Reqwest(e)
    }
}

impl From<SerdeJSONError> for Error {
    fn from(e: SerdeJSONError) -> Error {
        Error::SerdeJSON(e)
    }
}

impl From<RocksdbError> for Error {
    fn from(e: RocksdbError) -> Error {
        Error::RocksdbError(e)
    }
}
