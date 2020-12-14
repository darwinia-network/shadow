use crate::{mmr::helper, result::Error, ShadowShared};

/// Trim mmrs
pub fn exec(leaf: u64) -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    let total = helper::mmr_size_from_store(&shared.db) + 1;
    for i in cmmr::leaf_index_to_pos(leaf + 1)..total {
        shared.db.delete(i.to_be_bytes())?;
    }

    println!("Trimed leaves greater and equal than {}", leaf);
    println!(
        "Current best block: {}",
        helper::mmr_size_to_last_leaf(helper::mmr_size_from_store(&shared.db) as i64)
    );
    Ok(())
}
