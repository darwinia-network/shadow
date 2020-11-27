//! MMR Errors
use actix_web::Error as ActixWeb;
use cmmr::Error as MMR;
use primitives::result::Error as Primitive;
use reqwest::Error as Reqwest;
use rocksdb::Error as Rocksdb;
use serde_json::Error as SerdeJson;
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as Io,
    result::Result as StdResult,
};

/// The custom shadow error
pub struct Shadow(String);

impl Display for Shadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

/// Error generator
macro_rules! error {
    ($($e:ident),*) => {
        /// Bridger Error
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            $($e(String),)+
        }

        impl Display for Error {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $(Error::$e(e) => e.fmt(f),)+
                }
            }
        }

        impl ErrorTrait for Error {}

        $(
            impl From<$e> for Error {
                fn from(e: $e) -> Error {
                    Error::$e(format!("{}", e))
                }
            }
        )+
    };
}

error! {Io, MMR, Reqwest, SerdeJson, Rocksdb, Shadow, Primitive, ActixWeb}

/// Sup Result
pub type Result<T> = StdResult<T, Error>;
