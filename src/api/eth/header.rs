use crate::{
    chain::eth::{EthHeader, EthHeaderJson},
    db::pool,
    mmr::{MergeHash, Store, H256},
};
use actix_web::{web, Responder};
use cmmr::MMR;
use reqwest::Client;

#[derive(Serialize)]
struct ProofResp {
    header: EthHeaderJson,
    mmr_root: String,
}

/// Proof target header
///
/// ```
/// use darwinia_shadow::api::eth;
/// use actix_web::web;
///
/// // GET `/eth/header/19`
/// eth::header(web::Path::from("19".to_string()));
/// ```
pub async fn handle(block: web::Path<String>) -> impl Responder {
    let conn = pool::conn(None);
    let store = Store::with(conn);
    let client = Client::new();

    let num: u64 = block.to_string().parse().unwrap_or(0);
    let root = if num == 0 {
        "0000000000000000000000000000000000000000000000000000000000000000".to_string()
    } else {
        H256::hex(&MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(num - 1), &store).get_root().unwrap_or_default())
    };

    web::Json(ProofResp {
        header: EthHeader::get(&client, num)
            .await
            .unwrap_or_default()
            .into(),
        mmr_root: format!("0x{}", root),
    })
}
