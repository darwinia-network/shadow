use crate::{
    api::eth,
    mmr::{helper, MergeHash, H256},
    result::Error,
    ShadowShared,
};
use cmmr::MMR;
use rocksdb::IteratorMode;

/// Import headers from geth
pub fn exec(path: String, from: i32, to: i32) -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info,darwinia_shadow");
    std::env::set_var("GO_LOG", "ALL");
    env_logger::init();

    // Get hashes
    info!("Importing ethereum headers from {}...", &path);
    let hashes = eth::import(&path, from, to);
    let hashes_vec = hashes.split(',').collect::<Vec<&str>>();

    // Check empty
    info!("Imported {} hashes", hashes_vec.len());
    if hashes_vec[0].is_empty() {
        error!("Importing hashes from {} failed", path);
        return Ok(());
    }

    // Generate mmr store
    info!("Got {} header hashes", hashes_vec.len());
    let shared = ShadowShared::new(None);
    let mmr_size = shared.db.iterator(IteratorMode::Start).count() as u64;
    let last_leaf = helper::mmr_size_to_last_leaf(mmr_size as i64) as usize;
    if last_leaf < from as usize {
        error!(
            "The last leaf of mmr is {}, can not import mmr from {}",
            last_leaf, from
        );
    }
    let rect = last_leaf - from as usize;

    // Build mmr
    info!("mmr_size: {}, last_leaf: {}", mmr_size, last_leaf);
    let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &shared.store);
    if hashes_vec.len() > rect {
        let mut ptr = rect;
        for i in &hashes_vec[ptr..] {
            ptr += 1;
            if ptr % 1000 == 0 {
                trace!("Calculating {:?}/{}", ptr + from as usize, to);
            }
            mmr.push(H256::from(i))?;
        }
    }

    // Commit mmr
    mmr.commit()?;
    info!("done.");
    Ok(())
}
