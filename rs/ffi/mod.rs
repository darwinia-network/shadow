//! C Bridge
use crate::{
    hash::{MergeHash, H256},
    pool,
    store::Store,
    Runner,
};
use cmmr::MMR;
use std::ffi::CString;

/// Run the mmr service
#[no_mangle]
pub extern "C" fn run(t: i64, g: u32) -> i32 {
    env_logger::init();
    info!("starting mmr service...");
    let conn = pool::conn(None);
    if Runner::with(conn).start(t, g).is_ok() {
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
    let conn = pool::conn(None);
    let store = Store::with(conn);
    let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(last_leaf), store);
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
