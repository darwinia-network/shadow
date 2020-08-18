//! MMR Errors
use cmmr::Error as MMRError;
use diesel::result::Error as DieselError;

/// MMR Errors
#[derive(Debug)]
pub enum Error {
    /// Diesel Error
    Diesel(DieselError),
    /// MMR Error
    MMR(MMRError),
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
