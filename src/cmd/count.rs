use crate::{mmr::helper, result::Error, ShadowShared};
use rocksdb::IteratorMode;

/// Count mmr
pub fn exec() -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    println!(
        "Current best block: {}",
        helper::mmr_size_to_last_leaf(shared.db.iterator(IteratorMode::Start).count() as i64)
    );

    Ok(())
}
