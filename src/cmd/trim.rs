use crate::{result::Result};
use crate::mmr::{client_type, build_client};

/// Trim mmrs
pub fn exec(leaf: u64, uri: Option<String>) -> Result<()> {
    let client = build_client(&client_type(uri)?)?;
    client.trim_from(leaf)?;
    println!(
        "Current best block: {:?}",
        client.get_last_leaf_index()?
    );
    Ok(())
}
