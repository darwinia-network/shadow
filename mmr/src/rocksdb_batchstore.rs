//! MMR batchstore
use crate::H256;
use cmmr::{Error, MMRStore, Result as MMRResult};
use rocksdb::{DB, WriteBatch};
use std::sync::Arc;
use std::cell::RefCell;

/// MMR Batch Store
pub struct RocksBatchStore {
    /// DB for store
    pub db: Arc<DB>,
    /// batch cache
    pub batch: RefCell<WriteBatch>,
}

impl RocksBatchStore {
    /// New batchstore with database
    pub fn with(db: Arc<DB>) -> RocksBatchStore {
        RocksBatchStore {
            db,
            batch: RefCell::new(WriteBatch::default()),
        }
    }
    /// Start batch, clear cache
    pub fn start_batch(&self) {
        self.batch.borrow_mut().clear();
    }
    /// Commit batch to database
    pub fn commit_batch(self) -> MMRResult<()> {
        if let Err(e) = self.db.write(self.batch.into_inner()) {
            return Err(Error::StoreError(format!("batch write db failed {:?}", e)))
        }
        Ok(())
    }
}

impl MMRStore<[u8; 32]> for &RocksBatchStore
{
    fn get_elem(&self, pos: u64) -> MMRResult<Option<[u8; 32]>> {
        if let Ok(Some(elem)) = self.db.get(pos.to_be_bytes()) {
            Ok(Some(H256::from_bytes(&elem).unwrap()))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, pos: u64, elems: Vec<[u8; 32]>) -> MMRResult<()> {
        for (i, elem) in elems.into_iter().enumerate() {
            self.batch.borrow_mut().put((pos as usize + i).to_be_bytes(), elem);
        }
        Ok(())
    }
}

