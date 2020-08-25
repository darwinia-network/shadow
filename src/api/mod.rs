//! The API server of Shadow
use actix_web::{middleware, web, App, HttpServer};

mod proof;
mod receipt;

/// Run HTTP Server
#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/receipt/{tx}").to(receipt::handle))
            .service(web::resource("/proposal").to(proof::handle))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
