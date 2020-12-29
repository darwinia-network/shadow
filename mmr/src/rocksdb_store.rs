//! MMR store
use crate::H256;
use cmmr::{Error, MMRStore, Result as MMRResult};
use rocksdb::DB;
use std::sync::Arc;

/// MMR Store
#[derive(Clone)]
pub struct RocksdbStore {
    /// Connection Pool
    pub db: Arc<DB>,
}

impl RocksdbStore {
    /// New store with database
    pub fn with(db: Arc<DB>) -> RocksdbStore {
        RocksdbStore { db }
    }

    // Now we have only mmr data stored in the database. 
    // We should add some prefix when some other data need to be saved.
    /// Read mmr size from db, the key(block number) must be stored as big-endian byte ordering.
    pub fn get_mmr_size(&self) -> u64 {
        let mut iter = self.db.raw_iterator();
        iter.seek_to_last();
        if iter.valid() {
            if let Some(key) = iter.key() {
                let mut pos = [0; 8];
                pos.copy_from_slice(&key);
                return u64::from_be_bytes(pos) + 1;
            }
        }
        0
    }
}

impl MMRStore<[u8; 32]> for RocksdbStore
{
    fn get_elem(&self, pos: u64) -> MMRResult<Option<[u8; 32]>> {
        if let Ok(Some(elem)) = self.db.get(pos.to_be_bytes()) {
            Ok(Some(H256::from_bytes(&elem).unwrap()))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, pos: u64, elems: Vec<[u8; 32]>) -> MMRResult<()> {
        if cfg!(debug_assertions) {
            let mmr_size = self.get_mmr_size();
            if pos != mmr_size {
                return Err(Error::InconsistentStore);
            }
        }

        // Insert into database
        for (i, elem) in elems.into_iter().enumerate() {
            if let Err(e) = self.db.put((pos as usize + i).to_be_bytes(), elem) {
                return Err(Error::StoreError(format!(
                    "Insert mmr of pos {} into database failed, {:?}",
                    pos as i64 + i as i64,
                    e,
                )));
            }
        }

        Ok(())
    }
}

