//! Ethereum MMR API
use crate::{
    ShadowShared,
};
use mmr::{MergeHash, H256};
use actix_web::{web, Responder};
use cmmr::MMR;
use primitives::chain::ethereum::MMRRootJson;

/// Get target mmr
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::ethereum, ShadowShared};
///
/// // GET `/ethereum/mmr_root/19`
/// ethereum::mmr_root(web::Path::from("19".to_string()), web::Data::new(ShadowShared::new(None)));
/// ```
#[allow(clippy::eval_order_dependence)]
pub async fn handle(block: web::Path<String>, shared: web::Data<ShadowShared>) -> impl Responder {
    let num: u64 = block.to_string().parse().unwrap_or(0);
    let root = if num == 0 {
        "0000000000000000000000000000000000000000000000000000000000000000".to_string()
    } else {
        H256::hex(
            &MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(num - 1), &shared.store)
                .get_root()
                .unwrap_or_default(),
        )
    };

    web::Json(MMRRootJson {
        mmr_root: format!("0x{}", root),
    })
}
