use crate::ShadowShared;
use actix_web::web;
use primitives::chain::ethereum::EthereumRelayHeaderParcelJson;

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
pub async fn handle(
    block: web::Path<String>,
    shared: web::Data<ShadowShared>,
) -> super::helper::WebResult<web::Json<EthereumRelayHeaderParcelJson>> {
    let num: u64 = block.to_string().parse().unwrap_or(0);

    // Gen response
    Ok(web::Json(EthereumRelayHeaderParcelJson {
        header: super::helper::header(num, &shared).await?,
        mmr_root: super::helper::mmr_root(num, &shared)?,
    }))
}
