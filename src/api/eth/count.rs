use crate::{mmr::helper, ShadowShared};
use actix_web::{web, Responder};
use rocksdb::IteratorMode;

/// Count the mmr of ethereum headers
///
/// ```
/// use darwinia_shadow::api::eth;
/// use actix_web::web;
///
/// // GET `/eth/count`
/// eth::count();
/// ```
pub async fn handle(shared: web::Data<ShadowShared>) -> impl Responder {
    format!(
        "{}",
        helper::mmr_size_to_last_leaf(shared.db.iterator(IteratorMode::Start).count() as i64)
    )
}
