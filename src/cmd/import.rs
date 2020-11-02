use crate::{
    api::ethereum,
    mmr::{helper, MergeHash, H256},
    result::Error,
    ShadowShared,
};
use cmmr::MMR;
use rocksdb::{
    backup::{BackupEngine, BackupEngineOptions, RestoreOptions},
    IteratorMode,
};
use std::{env, fs::File};

/// Import headers from backup or geth
pub fn exec(path: String, from: i32, to: i32) -> Result<(), Error> {
    if path.ends_with("tar") {
        backup(path)
    } else {
        geth(path, from, to)
    }
}

/// Import headers from backup
fn backup(path: String) -> Result<(), Error> {
    let db_dir = dirs::home_dir().unwrap().join(".darwinia/cache/mmr");
    let mut wal_dir = env::temp_dir();
    wal_dir.push("shadow_mmr");

    // extract tar file
    tar::Archive::new(File::open(&path)?).unpack(&env::temp_dir())?;
    let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &wal_dir)?;
    engine.restore_from_latest_backup(db_dir, wal_dir, &RestoreOptions::default())?;
    Ok(())
}

/// Import headers from geth
fn geth(path: String, from: i32, to: i32) -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info,darwinia_shadow");
    std::env::set_var("GO_LOG", "ALL");
    env_logger::init();

    // Get hashes
    info!("Importing ethereum headers from {}...", &path);
    let hashes = ethereum::import(&path, from, to);
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
    if last_leaf + 1 != from as usize {
        error!(
            "The last leaf of mmr is {}, can not import mmr from {}, from must be last_leaf + 1",
            last_leaf, from
        );
    }

    // Build mmr
    info!("mmr_size: {}, last_leaf: {}", mmr_size, last_leaf);
    let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &shared.store);

    let mut ptr = from;
    for hash in &hashes_vec {
        if ptr % 1000 == 0 {
            trace!("Calculating {:?}/{}", ptr as usize, to);
        }

        ptr += 1;
        mmr.push(H256::from(hash))?;
    }

    // Commit mmr
    mmr.commit()?;
    info!("done.");
    Ok(())
}
