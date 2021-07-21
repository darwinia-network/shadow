use mmr::{Database, build_client};
use primitives::rpc::EthereumRPC;
use actix_web::{
    web::{Data, Path, Json},
    Responder
};
use primitives::{
    chain::ethereum::{EthereumHeaderJson, MMRProofJson},
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
    mmr_proof: MMRProofJson,
}

impl ReceiptResp {
    /// Get Receipt
    pub fn receipt(api: &str, tx: &str) -> ReceiptProof {
        ffi::receipt(api, tx).into()
    }

    /// Get ethereum header json
    pub async fn header(eth: &EthereumRPC, block: &str) -> Result<EthereumHeaderJson> {
        let result = eth.get_header_by_hash(block).await?.into();
        Ok(result)
    }

    /// Generate header
    /// mmr_root_height should be last confirmed block in relayt
    pub async fn new(mmr_db: &Database,
                     eth: &EthereumRPC,
                     tx: &str,
                     mmr_root_height: u64
    ) -> Result<ReceiptResp> {
        let receipt_proof = Self::receipt(eth.rpc(), tx);
        let header = Self::header(eth, &receipt_proof.header_hash).await?;

        let client = build_client(mmr_db)?;
        let (member_leaf_index, last_leaf_index) = (header.number, mmr_root_height - 1);
        let mmr_proof = MMRProofJson {
            member_leaf_index,
            last_leaf_index,
            proof: client.gen_proof(member_leaf_index, last_leaf_index)?,
        };

        Ok(ReceiptResp {
            header,
            receipt_proof,
            mmr_proof,
        })
    }
}

/// Receipt Handler
pub async fn handle(tx: Path<(String, u64)>, app_data: Data<AppData>) -> impl Responder {
    let tx_hash = tx.0.as_str();
    let mmr_root_height = tx.1;

    match ReceiptResp::new(&app_data.mmr_db, &app_data.eth, tx_hash, mmr_root_height).await {
        Ok(result) => Json(ReceiptResult::ReceiptResp(result)),
        Err(err) => Json(ReceiptResult::Error(err.to_json()))
    }
}

//#[test]
//fn header_before_london() {
    //let eth = EthereumRPC::new(reqwest::Client::new(), "https://ropsten.geth.darwinia.network");
    //// block 10499400
    //let header = ReceiptResp::header(&eth, "0xeafc2fd5df033e82a69943eb7d53a1cc4978047dc6557ab8fb5ee8c414ec3282");
    //assert_eq!(header.is_ok(), true);
    //header.unwrap().base_fee_per_gas, None
//}

//#[test]
//fn receipt_after_london() {
//}

