use crate::{result::Error, ShadowShared};
use rocksdb::backup::{BackupEngine, BackupEngineOptions};
use std::{
    env,
    fs::{self, File},
    io::stdout,
    path::PathBuf,
};
use tar::Builder;

/// Exec export command
pub fn exec(dist: Option<PathBuf>) -> Result<(), Error> {
    let tmp = env::temp_dir();

    // Create backup of rocksdb
    let dir = if let Some(p) = dist { p } else { tmp };
    let mut rocks = dir.clone();
    rocks.push("shadow_mmr");

    let shared = ShadowShared::new(None);
    let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &rocks)?;
    engine.create_new_backup_flush(&shared.db, true)?;

    // Tar backup
    if atty::is(atty::Stream::Stdout) {
        let mut tar = dir;
        tar.push("shadow_mmr.tar");

        let mut ar = Builder::new(File::create(tar)?);
        ar.append_dir_all("shadow_mmr", &rocks)?;
    } else {
        let mut ar = Builder::new(stdout());
        ar.append_dir_all("shadow_mmr", &rocks)?;
    };

    // clean backup
    fs::remove_dir_all(rocks)?;
    Ok(())
}
