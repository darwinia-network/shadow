use crate::{result::Result};
use crate::mmr::{build_client, ClientType};

/// Trim mmrs
pub fn exec(leaf: u64) -> Result<()> {
    let client = build_client(ClientType::Mysql)?;
    client.trim_from(leaf)?;
    println!(
        "Current best block: {:?}",
        client.get_last_leaf_index()?
    );
    Ok(())
}
