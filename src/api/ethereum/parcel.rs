use crate::{
    mmr::{MergeHash, H256},
    ShadowShared,
};
use actix_web::{error, web, Responder};
use cmmr::MMR;
use primitives::{
    chain::ethereum::{EthereumHeader, EthereumRelayHeaderParcelJson},
    rpc::RPC,
};

/// Proof target header
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::ethereum, ShadowShared};
///
/// // GET `/ethereum/parcel/19`
/// ethereum::parcel(web::Path::from("19".to_string()), web::Data::new(ShadowShared::new(None)));
/// ```
#[allow(clippy::eval_order_dependence)]
pub async fn handle(block: web::Path<String>, shared: web::Data<ShadowShared>) -> impl Responder {
    let header: EthereumHeader;
    let num: u64 = block.to_string().parse().unwrap_or(0);
    if num == 0 {
        return Err(error::ErrorBadRequest("Requesting mmr of block -1"));
    }

    if let Ok(h) = shared.eth.get_header_by_number(num).await {
        header = h;
    } else {
        return Err(error::ErrorInternalServerError(format!(
            "Get block header {} failed",
            num
        )));
    }

    // Gen response
    if let Ok(hash_bytes) =
        MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(num - 1), &shared.store).get_root()
    {
        Ok(web::Json(EthereumRelayHeaderParcelJson {
            header: header.into(),
            mmr_root: format!("0x{}", H256::hex(&hash_bytes)),
        }))
    } else {
        Err(error::ErrorInternalServerError(format!(
            "Get mmr root of block {} failed",
            num
        )))
    }
}
