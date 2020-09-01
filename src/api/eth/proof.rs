use crate::{
    chain::eth::EthHeader,
    db::pool,
    hex,
    mmr::{MergeHash, Store, H256},
};
use actix_web::{web, Responder};
use cmmr::MMR;
use reqwest::Client;
use scale::Encode;

#[derive(Serialize)]
struct ProofResp {
    eth_header: String,
    mmr_root: String,
}

/// Proof target header
///
/// ```
/// use darwinia_shadow::api::eth;
/// use actix_web::web;
///
/// // GET `/eth/proof/19`
/// eth::proof(web::Path::from("19".to_string()));
/// ```
pub async fn handle(block: web::Path<String>) -> impl Responder {
    let conn = pool::conn(None);
    let store = Store::with(conn);
    let client = Client::new();

    let num: u64 = block.to_string().parse().unwrap_or(0);
    let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(num - 1), &store);

    web::Json(ProofResp {
        eth_header: hex!(EthHeader::get(&client, num)
            .await
            .unwrap_or_default()
            .encode()),
        mmr_root: format!("0x{}", H256::hex(&mmr.get_root().unwrap_or_default())),
    })
}
