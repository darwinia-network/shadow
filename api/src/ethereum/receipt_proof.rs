use actix_web::{
    web::{Data, Json, Path},
    Responder,
};
use serde::Serialize;
use shadow_types::{
    chain::ethereum::{
        block::EthereumHeaderJson,
        receipt::{EthereumReceiptJson, ReceiptProof},
    },
    rpc::{EthereumRPC, Rpc},
};

use crate::error::ErrorJson;
use crate::{AppData, Result};

/// Receipt result
#[derive(Serialize)]
#[serde(untagged)]
pub enum ReceiptResult {
    ReceiptResp(EthereumReceiptJson),
    Error(ErrorJson),
}

async fn handle_receipt(eth: &EthereumRPC, tx: &str) -> Result<EthereumReceiptJson> {
    let receipt_proof: ReceiptProof = ffi::receipt(eth.rpc(), tx).into();
    let header: EthereumHeaderJson = eth
        .get_header_by_hash(&receipt_proof.header_hash)
        .await?
        .into()?;
    Ok(EthereumReceiptJson {
        header,
        receipt_proof,
    })
}

/// Receipt Handler
pub async fn handle(tx: Path<String>, app_data: Data<AppData>) -> impl Responder {
    let tx_hash = tx.as_str();

    match handle_receipt(&app_data.eth, tx_hash).await {
        Ok(result) => Json(ReceiptResult::ReceiptResp(result)),
        Err(err) => Json(ReceiptResult::Error(err.to_json())),
    }
}
