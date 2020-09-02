//! The API server of Shadow
use crate::{db::pool, mmr::Store};
use actix_web::{middleware, web, App, HttpServer};
use reqwest::Client;

pub mod eth;

/// Shadow shared data
pub struct ShadowShared {
    /// MMR Store
    pub store: Store,
    /// HTTP client
    pub client: Client,
}

/// Run HTTP Server
pub async fn serve(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(ShadowShared {
                store: Store::with(pool::conn(None)),
                client: Client::new(),
            })
            .service(web::resource("/eth/count").route(web::get().to(eth::count)))
            .service(web::resource("/eth/proposal").to(eth::proposal))
            .service(web::resource("/eth/receipt/{tx}").to(eth::receipt))
            .service(web::resource("/eth/header/{block}").to(eth::header))
    })
    .disable_signals()
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
