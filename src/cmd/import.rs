use crate::result::Result;

use crate::mmr::database;
use mmr::build_client;
use std::path::PathBuf;

/// Import headers from backup or geth
pub fn exec(path: String, to: u64, uri: Option<String>) -> Result<()> {
    std::env::set_var("RUST_LOG", "info");

    // Build mmr client
    let mut client = build_client(&database(uri)?)?;
    if path.ends_with("tar") {
        client.import_from_backup(PathBuf::from(path))?;
    } else {
        client.import_from_geth(PathBuf::from(path), to)?;
    }

    Ok(())
}

