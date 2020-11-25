use cmmr::{MMR, MMRStore};
use rocksdb::{IteratorMode, DB};

use crate::{Result, MergeHash, H256, MMRError, MmrClientTrait, mmr_size_to_last_leaf};
use crate::RocksdbStore;
use std::sync::Arc;

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

    fn get_elem(&self, pos: u64) -> Result<String> {
        let store = RocksdbStore::with(self.db.clone());
        let result = store.get_elem(pos)?;

        if let Some(hash) = result {
            Ok(H256::hex(&hash))
        } else {
            Err(MMRError::ElementNotFound(pos))?
        }
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
}