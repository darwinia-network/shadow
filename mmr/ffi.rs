//! C Bridge
use super::{
    hash::{MergeHash, H256},
    runner::Runner,
    store::Store,
};
use cmmr::MMR;

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

/// Run the mmr service
#[no_mangle]
pub extern "C" fn proof(leaves: Vec<u64>) -> String {
    let store = Store::default();
    let db = Runner::default();
    if let Ok(len) = db.mmr_count() {
        let mmr = MMR::<_, MergeHash, _>::new(len as u64, store);
        if let Ok(proof) = mmr.gen_proof(leaves) {
            return proof
                .proof_items()
                .iter()
                .map(|item| H256::hex(item))
                .collect::<Vec<String>>()
                .join(",");
        }
        return String::new();
    }
    String::new()
}
