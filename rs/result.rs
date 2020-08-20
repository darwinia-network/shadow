//! MMR Errors
use cmmr::Error as MMRError;
use diesel::result::Error as DieselError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJSONError;

/// MMR Errors
#[derive(Debug)]
pub enum Error {
    /// Diesel Error
    Diesel(DieselError),
    /// MMR Error
    MMR(MMRError),
    /// Reqwest Error
    Reqwest(ReqwestError),
    /// Reqwest Error
    SerdeJSON(SerdeJSONError),
    /// Custom
    Custom(String),
}

impl From<MMRError> for Error {
    fn from(e: MMRError) -> Error {
        Error::MMR(e)
    }
}

impl From<DieselError> for Error {
    fn from(e: DieselError) -> Error {
        Error::Diesel(e)
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
