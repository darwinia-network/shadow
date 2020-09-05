use actix_web::Responder;
use std::env;

/// Get version of shadow
pub async fn version() -> impl Responder {
    format!("shadow {}", env!("CARGO_PKG_VERSION"))
}
