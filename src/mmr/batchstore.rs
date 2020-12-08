//! MMR batchstore
use crate::mmr::hash::H256;
use cmmr::{Error, MMRStore, Result as MMRResult};
use rocksdb::{DB, WriteBatch};
use std::sync::Arc;
use std::cell::RefCell;

/// MMR Batch Store
pub struct BatchStore {
    /// DB for store
    pub db: Arc<DB>,
    /// batch cache
    pub batch: RefCell<WriteBatch>,
}

impl BatchStore {
    /// New batchstore with database
    pub fn with(db: Arc<DB>) -> BatchStore {
        BatchStore {
            db: db,
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

impl<H> MMRStore<H> for &BatchStore
where
    H: H256 + AsRef<[u8]>,
{
    fn get_elem(&self, pos: u64) -> MMRResult<Option<H>> {
        self.db
            .get(pos.to_le_bytes())
            .map_err(|err| {
                cmmr::Error::StoreError(err.to_string())
            })
            .map(|elem| {
                elem.map(|e| H::from_bytes(&e))
            })
    }

    fn append(&mut self, pos: u64, elems: Vec<H>) -> MMRResult<()> {
        for (i, elem) in elems.into_iter().enumerate() {
            self.batch.borrow_mut().put((pos as usize + i).to_le_bytes(), elem);
        }
        Ok(())
    }
}

