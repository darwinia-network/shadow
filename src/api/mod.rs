//! The API server of Shadow
use actix_web::{middleware, web, App, HttpServer};

pub mod eth;

/// Run HTTP Server
pub async fn serve(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/eth/count").route(web::get().to(eth::count)))
            .service(web::resource("/eth/proposal").to(eth::proposal))
            .service(web::resource("/eth/proposal/codec").to(eth::proposal_codec))
            .service(web::resource("/eth/receipt/{tx}").to(eth::receipt))
            .service(web::resource("/eth/header/{block}").to(eth::header))
    })
    .disable_signals()
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
