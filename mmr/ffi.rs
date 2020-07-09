//! C Bridge
use super::{
    hash::{MergeHash, H256},
    runner::Runner,
    store::Store,
};
use cmmr::MMR;
use std::{ffi::CString, slice};

/// Run the mmr service
#[no_mangle]
pub extern "C" fn run() -> i32 {
    env_logger::init();

    info!("starting mmr service...");
    if Runner::default().start().is_ok() {
        0
    } else {
        1
    }
}

/// Proof leaves
#[no_mangle]
pub unsafe extern "C" fn proof(leaves: *const u64, len: usize) -> CString {
    let leaves = Vec::from(slice::from_raw_parts(leaves, len));
    let store = Store::default();
    let mmr = MMR::<_, MergeHash, _>::new(
        cmmr::leaf_index_to_mmr_size(*leaves.iter().max().unwrap()),
        store,
    );
    if let Ok(proof) = mmr.gen_proof(leaves) {
        return CString::new(
            proof
                .proof_items()
                .iter()
                .map(|item| H256::hex(item))
                .collect::<Vec<String>>()
                .join(",")
                .as_bytes(),
        )
        .unwrap();
    }
    return CString::new("").unwrap();
}
