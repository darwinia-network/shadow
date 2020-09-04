use crate::{
    api::eth,
    mmr::{helper, MergeHash, H256},
    result::Error,
    ShadowShared,
};
use cmmr::MMR;
use rocksdb::IteratorMode;

/// Import headers from geth
pub fn exec(path: String, limit: i32) -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info,darwinia_shadow");
    env_logger::init();

    // Get hashes
    info!("Importing ethereum headers from {}...", &path);
    let hashes = eth::import(&path, limit);
    let hashes_vec = hashes.split(',').collect::<Vec<&str>>();

    // Generate mmr store
    info!("Got {} header hashes", hashes_vec.len());
    let shared = ShadowShared::new(None);
    let mmr_size = shared.db.iterator(IteratorMode::Start).count() as u64;
    let last_leaf = helper::mmr_size_to_last_leaf(mmr_size as i64) as usize;

    // Build mmr
    info!("mmr_size: {}, last_leaf: {}", mmr_size, last_leaf);
    let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &shared.store);
    if hashes_vec.len() > last_leaf {
        let mut ptr = last_leaf;
        for i in &hashes_vec[last_leaf..] {
            trace!("Calculating {:?}/{}", ptr + 1, limit);
            mmr.push(H256::from(i))?;
            ptr += 1;
        }
    }

    // Commit mmr
    mmr.commit()?;
    info!("done.");
    Ok(())
}
