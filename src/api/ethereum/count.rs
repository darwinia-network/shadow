use crate::{mmr::helper, ShadowShared};
use actix_web::{web, Responder};

/// Count the mmr of ethereum headers
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::ethereum, ShadowShared};
///
/// // GET `/ethereum/count`
/// ethereum::count(web::Data::new(ShadowShared::new(None)));
/// ```
pub async fn handle(shared: web::Data<ShadowShared>) -> impl Responder {
    format!(
        "{}",
        helper::mmr_size_to_last_leaf(helper::mmr_size_from_store(&shared.db) as i64)
    )
}
