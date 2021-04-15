use crate::mmr::database;
use crate::result::Result;
use mmr::build_client;
use std::env;
use std::path::PathBuf;

/// Exec export command
pub fn exec(dist: Option<PathBuf>, uri: Option<String>) -> Result<()> {
    // Build mmr client
    let client = build_client(&database(uri)?)?;
    let dir = if let Some(p) = dist {
        p
    } else {
        env::temp_dir()
    };
    client.backup(dir)?;
    Ok(())
}
