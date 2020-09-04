//! MMR store
use crate::mmr::hash::H256;
use cmmr::{Error, MMRStore, Result as MMRResult};
use rocksdb::{IteratorMode, DB};
use std::sync::Arc;

/// MMR Store
#[derive(Clone)]
pub struct Store {
    /// Connection Pool
    pub db: Arc<DB>,
}

impl Store {
    /// New store with database
    pub fn with(db: Arc<DB>) -> Store {
        Store { db }
    }
}

impl<H> MMRStore<H> for &Store
where
    H: H256 + AsRef<[u8]>,
{
    fn get_elem(&self, pos: u64) -> MMRResult<Option<H>> {
        if let Ok(Some(elem)) = self.db.get(pos.to_le_bytes()) {
            Ok(Some(H::from_bytes(&elem)))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, pos: u64, elems: Vec<H>) -> MMRResult<()> {
        if cfg!(debug_assertions) {
            let count = self.db.iterator(IteratorMode::Start).count();
            if (pos as usize) != count {
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
