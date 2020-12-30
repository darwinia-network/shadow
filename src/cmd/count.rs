use crate::{result::Result};
use crate::mmr::database;
use mmr::build_client;

/// Count mmr
pub fn exec(uri: Option<String>) -> Result<()> {
    // Build mmr client
    let client = build_client(&database(uri)?)?;

    println!(
        "Current leaf count: {}",
        client.get_leaf_count()?
    );

    Ok(())
}
