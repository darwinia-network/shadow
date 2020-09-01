use crate::{db::pool, mmr::Runner};
use actix_web::Responder;

/// Count the mmr of ethereum headers
///
/// ```
/// use darwinia_shadow::api::eth;
/// use actix_web::web;
///
/// // GET `/eth/proof/19`
/// eth::count();
/// ```
pub async fn handle() -> impl Responder {
    let conn = pool::conn(None);
    let runner = Runner::with(conn);
    format!("{}", runner.mmr_count().unwrap_or(0))
}
