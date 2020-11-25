//! MMR store
use crate::H256;
use cmmr::{Error, MMRStore, Result as MMRResult};
use rocksdb::{IteratorMode, DB};
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
}

impl MMRStore<[u8; 32]> for RocksdbStore
{
    fn get_elem(&self, pos: u64) -> MMRResult<Option<[u8; 32]>> {
        if let Ok(Some(elem)) = self.db.get(pos.to_le_bytes()) {
            Ok(Some(H256::from_bytes(&elem).unwrap()))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, pos: u64, elems: Vec<[u8; 32]>) -> MMRResult<()> {
        if cfg!(debug_assertions) {
            let mmr_size = self.db.iterator(IteratorMode::Start).count();
            if (pos as usize) != mmr_size {
                return Err(Error::InconsistentStore);
            }
        }

        // Insert into database
        for (i, elem) in elems.into_iter().enumerate() {
            if let Err(e) = self.db.put((pos as usize + i).to_le_bytes(), elem) {
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
