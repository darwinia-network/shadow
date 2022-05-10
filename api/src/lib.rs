//! The API server of Shadow
use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
pub use error::{Error, Result};
use shadow_types::rpc::EthereumRPC;

mod error;
pub mod ethereum;
mod root;

#[derive(Clone)]
pub struct AppData {
    pub(crate) eth: Arc<EthereumRPC>,
}

/// Run HTTP Server
pub async fn serve(port: u16, eth: &Arc<EthereumRPC>) -> Result<()> {
    let app_data = AppData { eth: eth.clone() };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(app_data.clone())
            .service(web::resource("/version").to(root::version))
            // .service(web::resource("/ethereum/parcel/{block}").to(ethereum::parcel))
            .service(web::resource("/ethereum/ethash_proof").to(ethereum::ethash_proof))
            .service(web::resource("/ethereum/receipt/{tx}").to(ethereum::receipt_proof))
    })
    .disable_signals()
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;
    Ok(())
}
