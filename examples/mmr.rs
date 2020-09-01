use cmmr::MMR;
use darwinia_shadow::{
    db::pool,
    mmr::{MergeHash, Store, H256},
};
use reqwest::Client;

fn main() {
    let conn = pool::conn(None);
    let store = Store::with(conn);
    let client = Client::new();

    let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(0), &store);
    println!("0x{}", H256::hex(&mmr.get_root().unwrap_or_default()));
}
