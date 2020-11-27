//! Ethereum MMR API
use crate::ShadowShared;
use actix_web::web;
use primitives::chain::ethereum::MMRRootJson;

/// Get target mmr
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::ethereum, ShadowShared};
///
/// // GET `/ethereum/parent_mmr_root/19`
/// ethereum::mmr_root(web::Path::from("19".to_string()), web::Data::new(ShadowShared::new(None)));
/// ```
pub async fn handle(
    block: web::Path<String>,
    shared: web::Data<ShadowShared>,
) -> super::helper::WebResult<web::Json<MMRRootJson>> {
    let num: u64 = block.to_string().parse().unwrap_or(0);
    Ok(web::Json(MMRRootJson {
        mmr_root: super::helper::parent_mmr_root(num, &shared)?,
    }))
}
