use crate::{mmr::helper, ShadowShared};
use actix_web::{web, Responder};
use rocksdb::IteratorMode;

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
        helper::mmr_size_to_last_leaf(shared.db.iterator(IteratorMode::Start).count() as i64)
    )
}
