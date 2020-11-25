use crate::{result::Result};
use crate::mmr::{build_client, ClientType};

/// Count mmr
pub fn exec() -> Result<()> {
    let client = build_client(ClientType::Mysql)?;

    let count =
        match client.count()? {
            None => "None".to_string(),
            Some(count) => count.to_string()
        };

    println!(
        "Current best block: {}",
        count
    );

    Ok(())
}
