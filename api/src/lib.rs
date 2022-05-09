//! The API server of Shadow
use actix_web::{middleware, web, App, HttpServer};

mod error;
pub mod ethereum;
mod root;

pub use error::{Error, Result};
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
        eth: eth.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(app_data.clone())
            .service(web::resource("/version").to(root::version))
            .service(web::resource("/ethereum/count").route(web::get().to(ethereum::count)))
            .service(
                web::resource("/ethereum/parent_mmr_root/{block}")
                    .route(web::get().to(ethereum::parent_mmr_root)),
            )
            .service(
                web::resource("/ethereum/mmr_leaf/{block}")
                    .route(web::get().to(ethereum::mmr_leaf)),
            )
            // .service(web::resource("/ethereum/parcel/{block}").to(ethereum::parcel))
            .service(web::resource("/ethereum/proof").to(ethereum::proof))
            .service(
                web::resource("/ethereum/receipt/{tx}/{last}").to(ethereum::receipt_with_mmr_proof),
            )
            .service(web::resource("/ethereum/only-receipt/{tx}").to(ethereum::only_receipt))
    })
    .disable_signals()
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;
    Ok(())
}
