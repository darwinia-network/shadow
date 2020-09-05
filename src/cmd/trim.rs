use crate::{mmr::helper, result::Error, ShadowShared};
use rocksdb::{IteratorMode, DB};

fn count(db: &DB) -> i64 {
    db.iterator(IteratorMode::Start).count() as i64
}

/// Trim mmrs
pub fn exec(leaf: u64) -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    let total = count(&shared.db) as u64 + 1;
    for i in cmmr::leaf_index_to_pos(leaf + 1)..total {
        shared.db.delete(i.to_le_bytes())?;
    }

    println!("Trimed leaves greater and equal than {}", leaf);
    println!(
        "Current best block: {}",
        helper::mmr_size_to_last_leaf(count(&shared.db))
    );
    Ok(())
}
