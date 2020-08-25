//! The API server of Shadow
use actix_web::{web, App, HttpServer};

mod receipt;

/// Run HTTP Server
#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/receipt/{tx}").to(receipt::handle)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
