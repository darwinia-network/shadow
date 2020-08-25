use actix_web::{web, Responder};

/// Receipt Handler
pub async fn handle(tx: web::Path<String>) -> impl Responder {
    format!("Hello {}!", tx)
}
