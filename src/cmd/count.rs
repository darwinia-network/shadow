use crate::{result::Result};
use crate::mmr::{client_type, build_client};

/// Count mmr
pub fn exec(uri: Option<String>) -> Result<()> {
    // Build mmr client
    let client = build_client(&client_type(uri)?)?;

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
