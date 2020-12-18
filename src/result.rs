//! MMR Errors
use thiserror::Error as ThisError;
use anyhow::Result as AnyResult;

#[allow(missing_docs)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Failed to connect to darwinia node {url}")]
    FailToConnectDarwinia {
        url: String,
        source: jsonrpsee::transport::ws::WsNewDnsError,
    },

    #[error("{0}")]
    Shadow(String),
}

/// Sup Result
pub type Result<T> = AnyResult<T>;
