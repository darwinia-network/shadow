use cmmr::MMR;
use darwinia_shadow::{
    mmr::{MergeHash, H256},
    ShadowShared,
};

fn main() {
    let store = ShadowShared::new(None).store;
    let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(0), &store);
    println!("0x{}", H256::hex(&mmr.get_root().unwrap_or_default()));
}
