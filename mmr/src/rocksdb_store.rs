//! MMR store
use crate::{H256, MergeHash};
use cmmr::{MMR, Error, MMRStore, Result as MMRResult};
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

    /// Generate proof by member and last_leaf
    pub fn gen_proof(&self, member: u64, last_leaf: u64) -> Vec<String> {
        match MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(last_leaf), self)
            .gen_proof(vec![cmmr::leaf_index_to_pos(member)])
        {
            Err(e) => {
                error!(
                    "Generate proof failed {:?}, last_leaf: {:?}, leaves: {:?}",
                    e, last_leaf, member,
                );
                vec![]
            }
            Ok(proof) => proof
                .proof_items()
                .iter()
                .map(|item| format!("0x{}", H256::hex(item)))
                .collect::<Vec<String>>(),
        }
    }
}

impl MMRStore<[u8; 32]> for &RocksdbStore
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
