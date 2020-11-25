use crate::{result::Result};
use crate::mmr::{build_client, ClientType};
use std::path::PathBuf;
use std::env;

/// Exec export command
pub fn exec(dist: Option<PathBuf>) -> Result<()> {
    let client = build_client(ClientType::Rocksdb)?;
    let dir = if let Some(p) = dist { p } else { env::temp_dir() };
    client.backup(&dir)?;
    Ok(())
}
