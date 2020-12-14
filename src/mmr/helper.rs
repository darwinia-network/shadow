//! Helper fns
use crate::mmr::{MergeHash, Store, H256};
use cmmr::MMR;
use std::cmp::Ordering;
use rocksdb::{DB};

fn log2_floor(mut num: i64) -> i64 {
    let mut res = -1;
    while num > 0 {
        res += 1;
        num >>= 1;
    }
    res
}

/// MMR size to last leaf `O(log2(log2(n)))`
pub fn mmr_size_to_last_leaf(mmr_size: i64) -> i64 {
    if mmr_size == 0 {
        return 0;
    }

    let mut m = log2_floor(mmr_size);
    loop {
        match (2 * m - m.count_ones() as i64).cmp(&mmr_size) {
            Ordering::Equal => return m - 1,
            Ordering::Greater => m -= 1,
            Ordering::Less => m += 1,
        }
    }
}

/// Generate proof by members and last_leaf
pub fn gen_proof(store: &Store, member: u64, last_leaf: u64) -> Vec<String> {
    match MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(last_leaf), store)
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

// Now we have only mmr data stored in the database. 
// We should add some prefix when some other data need to be saved.
/// Read mmr size from db, the key(block number) must be stored as big-endian byte ordering.
pub fn mmr_size_from_store(db: &DB) -> u64 {
    let mut iter = db.raw_iterator();
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

