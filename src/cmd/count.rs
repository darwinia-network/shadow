use crate::{mmr::helper, result::Error, ShadowShared};

/// Count mmr
pub fn exec() -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    println!(
        "Current best block: {}",
        helper::mmr_size_to_last_leaf(helper::mmr_size_from_store(&shared.db) as i64)
    );

    Ok(())
}
