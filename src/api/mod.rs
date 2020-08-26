//! The API server of Shadow
use actix_web::{middleware, web, App, HttpServer};

mod proof;
mod receipt;

/// Run HTTP Server
pub async fn serve(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/receipt/{tx}").to(receipt::handle))
            .service(web::resource("/proposal").to(proof::handle))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
