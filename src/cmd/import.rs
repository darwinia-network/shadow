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
pub fn exec(path: String, to: i32) -> Result<(), Error> {
    if path.ends_with("tar") {
        backup(path)
    } else {
        geth(path, to)
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
fn geth(path: String, to: i32) -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info,darwinia_shadow");
    std::env::set_var("GO_LOG", "ALL");
    env_logger::init();

    let shared = ShadowShared::new(None);

    let mmr_size = shared.db.iterator(IteratorMode::Start).count() as u64;
    let from = if mmr_size == 0 {
        0
    } else {
        let last_leaf = helper::mmr_size_to_last_leaf(mmr_size as i64) as usize;
        last_leaf + 1
    };

    if from as i32 >= to {
        error!(
            "The to position of mmr is {}, can not import mmr from {}, from must be less than to",
            to, from
        );
    }

    // Get hashes
    info!("Importing ethereum headers from {}...", &path);
    let hashes = ethereum::import(&path, from as i32, to);
    let hashes_vec = hashes.split(',').collect::<Vec<&str>>();

    // Check empty
    info!("Imported {} hashes from ethereum node", hashes_vec.len());
    if hashes_vec[0].is_empty() {
        error!("Importing hashes from {} failed", path);
        return Ok(());
    }

    // Build mmr
    info!("mmr_size: {}, from: {}", mmr_size, from);
    let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &shared.store);

    let mut ptr = from;
    for hash in &hashes_vec {
        if ptr % 1000 == 0 {
            trace!("Start to push hash into mmr for block {:?}/{}", ptr as usize, to);
        }

        ptr += 1;
        mmr.push(H256::from(hash))?;
    }

    // Commit mmr
    mmr.commit()?;
    info!("done.");
    Ok(())
}
