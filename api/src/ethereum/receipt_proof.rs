use primitives::rpc::EthereumRPC;
use actix_web::{
    web::{Data, Path, Json},
    Responder
};
use primitives::{
    chain::ethereum::EthereumHeaderJson,
    rpc::Rpc,
};
use crate::{Result, AppData};
use serde::Serialize;
use crate::error::ErrorJson;

/// Receipt result
#[derive(Serialize)]
#[serde(untagged)]
pub enum ReceiptResult {
    ReceiptResp(ReceiptResp),
    Error(ErrorJson)
}

/// Receipt proof
#[derive(Serialize)]
pub struct ReceiptProof {
    index: String,
    proof: String,
    header_hash: String,
}

impl From<(String, String, String)> for ReceiptProof {
    fn from(t: (String, String, String)) -> ReceiptProof {
        ReceiptProof {
            index: t.0,
            proof: t.1,
            header_hash: t.2,
        }
    }
}

/// Receipt response
#[derive(Serialize)]
pub struct ReceiptResp {
    header: EthereumHeaderJson,
    receipt_proof: ReceiptProof,
}

impl ReceiptResp {
    /// Get Receipt
    pub fn receipt_proof(api: &str, tx: &str) -> ReceiptProof {
        ffi::receipt(api, tx).into()
    }

    /// Get ethereum header json
    pub async fn header(eth: &EthereumRPC, block: &str) -> Result<EthereumHeaderJson> {
        let result = eth.get_header_by_hash(block).await?.into();
        Ok(result)
    }

    /// Generate header
    /// mmr_root_height should be last confirmed block in relayt
    pub async fn new(eth: &EthereumRPC,
                     tx: &str
    ) -> Result<ReceiptResp> {
        let receipt_proof = Self::receipt_proof(eth.rpc(), tx);
        let header = Self::header(eth, &receipt_proof.header_hash).await?;

        Ok(ReceiptResp {
            header,
            receipt_proof,
        })
    }
}

/// Receipt Handler
pub async fn handle(tx: Path<String>, app_data: Data<AppData>) -> impl Responder {
    let tx_hash = tx.as_str();

    match ReceiptResp::new(&app_data.eth, tx_hash).await {
        Ok(result) => Json(ReceiptResult::ReceiptResp(result)),
        Err(err) => Json(ReceiptResult::Error(err.to_json()))
    }
}

