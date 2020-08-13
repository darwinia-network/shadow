//! C Bridge
use crate::{
    hash::{MergeHash, H256},
    runner::Runner,
    store::{self, Store},
};
use cmmr::MMR;
use std::ffi::CString;

/// Run the mmr service
#[no_mangle]
pub extern "C" fn run() -> i32 {
    env_logger::init();
    info!("starting mmr service...");
    let conn = store::default_conn();
    if Runner::with(&conn).start().is_ok() {
        0
    } else {
        error!("mmr service start failed");
        1
    }
}

/// Proof leaves
///
/// # Safe
///
/// Concatenate strings
#[no_mangle]
pub unsafe extern "C" fn proof(last_leaf: u64, member: u64) -> CString {
    let conn = store::default_conn();
    let store = Store::with(&conn);
    let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(last_leaf), &store);
    match mmr.gen_proof(vec![cmmr::leaf_index_to_pos(member)]) {
        Err(e) => {
            error!(
                "Generate proof failed {:?}, last_leaf: {:?}, member: {:?}",
                e, last_leaf, member
            );
            CString::new("").unwrap()
        }
        Ok(proof) => CString::new(
            proof
                .proof_items()
                .iter()
                .map(|item| H256::hex(item))
                .collect::<Vec<String>>()
                .join(",")
                .as_bytes(),
        )
        .unwrap(),
    }
}
