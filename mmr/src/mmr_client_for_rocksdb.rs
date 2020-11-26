use cmmr::{MMR, MMRStore};
use rocksdb::{IteratorMode, DB};

use crate::{Result, MergeHash, H256, MmrClientTrait, mmr_size_to_last_leaf};
use crate::RocksdbStore;
use std::sync::Arc;
use rocksdb::backup::{BackupEngine, BackupEngineOptions, RestoreOptions};
use std::path::PathBuf;
use std::{io, fs, env};
use std::io::{Write, stdout};
use tar::Builder;
use std::fs::File;

pub struct MmrClientForRocksdb {
    db: Arc<DB>,
}

impl MmrClientForRocksdb {
    /// create a new client instance
    pub fn new(db: Arc<DB>) -> Self {
        MmrClientForRocksdb { db }
    }
}

impl MmrClientTrait for MmrClientForRocksdb {
    fn push(&mut self, elem: &str) -> Result<u64> {
        let store = RocksdbStore::with(self.db.clone());
        let mut mmr = MMR::<[u8; 32], MergeHash, _>::new(self.get_mmr_size()?, store);
        let elem = H256::from(elem)?;
        let position = mmr.push(elem)?;
        mmr.commit()?;
        Ok(position)
    }

    fn batch_push(&mut self, elems: &[&str]) -> Result<Vec<u64>> {
        let mut result = vec![];

        let store = RocksdbStore::with(self.db.clone());
        let mut mmr = MMR::<[u8; 32], MergeHash, _>::new(self.get_mmr_size()?, store);
        for &elem in elems {
            let elem = H256::from(elem)?;
            let position = mmr.push(elem)?;
            result.push(position);
        }
        mmr.commit()?;

        Ok(result)
    }

    fn get_mmr_size(&self) -> Result<u64> {
        let mmr_size = self.db.iterator(IteratorMode::Start).count() as u64;
        Ok(mmr_size)
    }

    fn get_last_leaf_index(&self) -> Result<Option<u64>> {
        let mmr_size = self.get_mmr_size().unwrap();
        if mmr_size == 0 {
            Ok(None)
        } else {
            let last_leaf_index = mmr_size_to_last_leaf(mmr_size as i64);
            Ok(Some(last_leaf_index as u64))
        }
    }

    fn get_elem(&self, pos: u64) -> Result<Option<String>> {
        let store = RocksdbStore::with(self.db.clone());
        let result = store.get_elem(pos)?;
        Ok(
            result.map(|hash| H256::hex(&hash))
        )
    }

    fn gen_proof(&self, member: u64, last_leaf: u64) -> Result<Vec<String>> {
        let store = RocksdbStore::with(self.db.clone());
        let mmr_size = cmmr::leaf_index_to_mmr_size(last_leaf);
        let mmr = MMR::<[u8; 32], MergeHash, _>::new(mmr_size, store);
        let proof = mmr.gen_proof(vec![cmmr::leaf_index_to_pos(member)])?;
        Ok(
            proof
                .proof_items()
                .iter()
                .map(|item| H256::hex(item))
                .collect::<Vec<String>>()
        )
    }

    fn backup(&self, dir: PathBuf) -> Result<()> {
        let mut rocks = dir.clone();
        rocks.push("shadow_mmr");

        let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &rocks)?;
        engine.create_new_backup_flush(&self.db.clone(), true)?;
        io::stderr().write_all(
            format!("Created backup at {} succeed!\n", &rocks.to_string_lossy()).as_bytes(),
        )?;

        // Tar backup
        io::stderr().write_all(b"Generateing tar package...\n")?;
        if atty::is(atty::Stream::Stdout) {
            let mut tar = dir.clone();
            if !&dir.exists() {
                fs::create_dir(dir)?;
            }
            tar.push("shadow_mmr.tar");

            let mut ar = Builder::new(File::create(&tar)?);
            ar.append_dir_all("shadow_mmr", &rocks)?;
            io::stderr()
                .write_all(format!("Export mmr at {} succeed!\n", &tar.to_string_lossy()).as_bytes())?;
        } else {
            let mut ar = Builder::new(stdout());
            ar.append_dir_all("shadow_mmr", &rocks)?;
            io::stderr().write_all(b"Export mmr succeed!")?;
        };

        // clean backup
        fs::remove_dir_all(&rocks)?;
        io::stderr()
            .write_all(format!("Clean backup at {} succeed!\n", &rocks.to_string_lossy()).as_bytes())?;
        Ok(())
    }

    fn trim_from(&self, leaf_index: u64) -> Result<()> {
        let mmr_size = self.get_mmr_size().unwrap();
        for i in cmmr::leaf_index_to_pos(leaf_index)..mmr_size {
            self.db.delete(i.to_le_bytes())?;
        }

        trace!("Trimed leaves greater and equal than {}", leaf_index);
        Ok(())
    }

    fn import_from_backup(&self, backup_file: PathBuf) -> Result<()> {
        // from
        tar::Archive::new(File::open(&backup_file)?).unpack(&env::temp_dir())?;
        let mut wal_dir = env::temp_dir();
        wal_dir.push("shadow_mmr");

        //
        let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &wal_dir)?;
        engine.restore_from_latest_backup(self.db.path(), wal_dir, &RestoreOptions::default())?;
        Ok(())
    }

    fn import_from_geth(&self, _geth_dir: PathBuf, _til_block: u64) -> Result<()> {
        // let from = self.count()?;
        // if from >= til_block {
        //     anyhow::bail!("The to position of mmr is {}, can not import mmr from {}, from must be less than to",
        //         to, from
        //     );
        // }
        //
        // // Get hashes
        // info!("Importing ethereum headers from {}...", geth_dir);
        // let hashes = ethereum::import(&geth_dir, from, til_block);
        // let hashes_vec = hashes.split(',').collect::<Vec<&str>>();
        //
        // // Check empty
        // info!("Imported {} hashes from ethereum node", hashes_vec.len());
        // if hashes_vec[0].is_empty() {
        //     anyhow::bail!("Importing hashes from {} failed", path);
        // }
        //
        // // Build mmr
        // let store = RocksdbStore::with(self.db.clone());
        // let mmr_size = cmmr::leaf_index_to_mmr_size(last_leaf);
        // info!("mmr_size: {}, from: {}", mmr_size, from);
        // let mut mmr = MMR::<[u8; 32], MergeHash, _>::new(mmr_size, store);
        //
        // let mut ptr = from;
        // for hash in &hashes_vec {
        //     if ptr % 1000 == 0 {
        //         trace!("Start to push hash into mmr for block {:?}/{}", ptr as usize, to);
        //     }
        //
        //     ptr += 1;
        //     mmr.push(H256::from(hash)?)?;
        // }
        //
        // // Commit mmr
        // mmr.commit()?;
        // info!("done.");
        // Ok(())
        unimplemented!()
    }

    fn get_leaf(&self, leaf_index: u64) -> Result<Option<String>> {
        self.get_elem(cmmr::leaf_index_to_pos(leaf_index))
    }

    fn get_mmr_root(&self, leaf_index: u64) -> Result<Option<String>> {
        if let Some(last_leaf_index) = self.get_last_leaf_index()? {
            if leaf_index > last_leaf_index {
                Ok(None)
            } else {
                let store = RocksdbStore::with(self.db.clone());
                let mmr_size = cmmr::leaf_index_to_mmr_size(leaf_index);
                let mmr = MMR::<[u8; 32], MergeHash, _>::new(mmr_size, store);
                let root = mmr.get_root()?;
                Ok(Some(H256::hex(&root)))
            }
        } else {
            Ok(None)
        }
    }
}