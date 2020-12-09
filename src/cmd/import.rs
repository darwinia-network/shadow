use crate::{
    api::ethereum,
    mmr::{helper, MergeHash, H256, BatchStore},
    result::Error,
    ShadowUnsafe,
};
use cmmr::MMR;
use rocksdb::{
    backup::{BackupEngine, BackupEngineOptions, RestoreOptions},
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
    std::env::set_var("GO_LOG", "info");
    env_logger::init();

    let shadow_unsafe= ShadowUnsafe::new(None);

    let mut mmr_size = helper::mmr_size_from_store(&shadow_unsafe.db);
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
    info!("mmr_size: {}, from: {}", mmr_size, from);
    const BATCH: i32 = 10240;
    let ret = ethereum::import(&path, from as i32, to, BATCH, |hashes| {
        let hashes_vec = hashes.split(',').collect::<Vec<&str>>();
        let veclen = hashes_vec.len();
        trace!("push mmr size-start {}, batch-length {}", mmr_size, veclen);
        let bstore = BatchStore::with(shadow_unsafe.db.clone());
        let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, &bstore);
        for hash in &hashes_vec {
            if let Err(e) = mmr.push(H256::from(hash)) {
                error!("push mmr failed, hash {} exception {}", hash, e);
                return false
            }
        }
        mmr_size = mmr.mmr_size();

        bstore.start_batch();
        match mmr.commit() {
            Err(e) => {
                error!("commit mmr failed exception{}", e);
                false
            },
            _ => {
                if let Err(_e) = bstore.commit_batch() {
                    return false;
                }
                true
            }
        }
    });
    info!("done");
    if ret {
        Ok(())
    } else {
        Err(Error::Primitive(String::from("import geth failed")))
    }
}
