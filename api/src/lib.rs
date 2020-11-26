//! The API server of Shadow
use actix_web::{middleware, web, App, HttpServer};

pub mod ethereum;
mod root;
mod error;

pub use error::{Result, Error};
use primitives::rpc::EthereumRPC;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppData {
    pub(crate) mmr_db: mmr::Database,
    pub(crate) eth: Arc<EthereumRPC>,
}

/// Run HTTP Server
pub async fn serve(port: u16, mmr_db: &mmr::Database, eth: &Arc<EthereumRPC>) -> Result<()> {
    let app_data = AppData {
        mmr_db: mmr_db.clone(),
        eth: eth.clone()
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(app_data.clone())
            .service(web::resource("/version").to(root::version))
            .service(web::resource("/ethereum/count").route(web::get().to(ethereum::count)))
            .service(
                web::resource("/ethereum/mmr_root/{block}")
                    .route(web::get().to(ethereum::mmr_root)),
            )
            .service(
                web::resource("/ethereum/mmr_leaf/{block}")
                    .route(web::get().to(ethereum::mmr_leaf)),
            )
            // .service(web::resource("/ethereum/parcel/{block}").to(ethereum::parcel))
            .service(web::resource("/ethereum/proof").to(ethereum::proof))
            .service(web::resource("/ethereum/receipt/{tx}/{last}").to(ethereum::receipt))
    })
        .disable_signals()
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await?;
    Ok(())
}