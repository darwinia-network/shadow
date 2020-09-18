use crate::{result::Error, ShadowShared};
use rocksdb::backup::{BackupEngine, BackupEngineOptions};
use std::path::PathBuf;

/// Exec export command
pub fn exec(dist: PathBuf) -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    let mut engine = BackupEngine::open(&BackupEngineOptions::default(), dist)?;
    engine.create_new_backup_flush(&shared.db, true)?;
    Ok(())
}
