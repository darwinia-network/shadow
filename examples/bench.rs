#[macro_use]
extern crate log;
use cmmr::MMR;
use darwinia_shadow::{
    chain::eth::EthHeaderRPCResp,
    mmr::{helper, MergeHash, Store, H256},
    ShadowShared,
};
use reqwest::Client;
use rocksdb::IteratorMode;
use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

#[actix_rt::main]
pub async fn main() {
    std::env::set_var("RUST_LOG", "info,darwinia_shadow");
    env_logger::init();

    let shared = ShadowShared::new(None);
    let mut mmr_size = shared.db.iterator(IteratorMode::Start).count() as u64;
    let last_leaf = helper::mmr_size_to_last_leaf(mmr_size as i64);
    let mut ptr = if last_leaf == 0 { 0 } else { last_leaf + 1 };

    loop {
        debug!("-{}-{}------------", ptr, mmr_size);
        let a = now();
        let mmr_size_new = push(&shared.store, &shared.client, ptr, mmr_size).await;

        mmr_size = mmr_size_new;
        ptr += 1;
        debug!("total: {}", now() - a);
    }
}

/// Push new header hash into storage
pub async fn push(store: &Store, client: &Client, number: i64, mmr_size: u64) -> u64 {
    let a = now();
    let mut mmr = MMR::<_, MergeHash, _>::new(mmr_size, store);

    let b = now();
    debug!("mmr create  : {}", b - a);
    let hash_from_ethereum = &EthHeaderRPCResp::get(&client, number as u64)
        .await
        .unwrap()
        .result
        .hash;

    let c = now();
    debug!("rpc call    : {}", c - b);
    mmr.push(H256::from(hash_from_ethereum)).unwrap();

    let d = now();
    debug!("push to mmr : {}", d - c);

    let mmr_size_new = mmr.mmr_size();
    let e = now();
    debug!("get new size: {}", e - d);
    mmr.commit().expect("commit failed");

    let f = now();
    debug!("commit      : {}", f - e);
    mmr_size_new
}
