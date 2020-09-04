use crate::{mmr::helper, result::Error, ShadowShared};
use rocksdb::{IteratorMode, DB};

fn count(db: &DB) -> i64 {
    helper::mmr_size_to_last_leaf(db.iterator(IteratorMode::Start).count() as i64)
}

/// Trim mmrs
pub fn exec(leaf: u64) -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    shared
        .db
        .delete_file_in_range(leaf.to_le_bytes(), (count(&shared.db) + 1).to_le_bytes())?;
    println!("Trimed leaves greater and equal than {}", leaf);
    println!(
        "Current best block: {}",
        helper::mmr_size_to_last_leaf(count(&shared.db) as i64)
    );

    Ok(())
}
