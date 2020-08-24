//! Mock the uncle block
use cmmr::{leaf_index_to_mmr_size, MMR};
use shadow::{
    hash::{MergeHash, H256},
    pool,
    store::Store,
};

fn main() {
    let conn = pool::conn(None);
    let store = Store::with(conn);

    let mmr = MMR::<_, MergeHash, _>::new(leaf_index_to_mmr_size(1), &store);
    let leaf_root = mmr.get_root().expect("get root failed");

    // let proofs = mmr
    //     .gen_proof(vec![leaf_index_to_pos(19)])
    //     .unwrap()
    //     .proof_items()
    //     .iter()
    //     .map(|item| H256::hex(item))
    //     .collect::<Vec<String>>();
    //
    println!("leaf root: {:?}", H256::hex(&leaf_root));
    // println!("{:?}", proofs);
    // return;
    // let proofs = mmr
    //     .gen_proof(vec![pos[19]])
    //     .unwrap()
    //     .proof_items()
    //     .iter()
    //     .map(|item| H256::hex(item))
    //     .collect::<Vec<String>>();
    //
    // println!("{:#?}, {:#?}, {:#?}", HASHES, roots, proofs);
}
