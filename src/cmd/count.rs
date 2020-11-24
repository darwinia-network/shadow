use crate::{result::Result, ShadowShared};
use mmr::mmr_size_to_last_leaf;
use rocksdb::IteratorMode;

/// Count mmr
pub fn exec() -> Result<()> {
    let shared = ShadowShared::new(None);
    println!(
        "Current best block: {}",
        mmr_size_to_last_leaf(shared.db.iterator(IteratorMode::Start).count() as i64)
    );

    Ok(())
}
