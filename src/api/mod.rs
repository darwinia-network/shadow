//! The API server of Shadow
use crate::ShadowShared;
use actix_web::{middleware, web, App, HttpServer};

pub mod ethereum;
mod root;

/// Run HTTP Server
pub async fn serve(port: u16, shared: ShadowShared) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(shared.clone())
            .service(web::resource("/version").to(root::version))
            .service(web::resource("/ethereum/count").route(web::get().to(ethereum::count)))
            .service(web::resource("/ethereum/proposal").to(ethereum::proposal))
            .service(web::resource("/ethereum/receipt/{tx}/{last}").to(ethereum::receipt))
            .service(web::resource("/ethereum/header/{block}").to(ethereum::header))
    })
    .disable_signals()
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
