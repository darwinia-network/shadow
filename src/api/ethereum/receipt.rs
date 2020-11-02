use crate::{mmr::helper, ShadowShared};
use actix_web::{web, Responder};
use primitives::{
    chain::ethereum::{EthereumHeaderJson, MMRProofJson},
    rpc::RPC,
};

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
        super::ffi::receipt(api, tx).into()
    }
    /// Get ethereum header json
    pub async fn header(shared: &ShadowShared, block: &str) -> EthereumHeaderJson {
        shared
            .eth
            .get_header_by_hash(block)
            .await
            .unwrap_or_default()
            .into()
    }

    /// Generate header
    /// mmr_root_height should be last confirmed block in relayt
    pub async fn new(shared: &ShadowShared, tx: &str, mmr_root_height: u64) -> ReceiptResp {
        let receipt_proof = Self::receipt(shared.eth.rpc(), tx);
        let header = Self::header(&shared, &receipt_proof.header_hash).await;
        let mmr_proof = if mmr_root_height > 0 {
            let (member_leaf_index, last_leaf_index) = (header.number, mmr_root_height - 1);
            MMRProofJson {
                member_leaf_index,
                last_leaf_index,
                proof: helper::gen_proof(&shared.store, member_leaf_index, last_leaf_index),
            }
        } else {
            MMRProofJson::default()
        };
        ReceiptResp {
            header,
            receipt_proof,
            mmr_proof,
        }
    }
}

/// Receipt Handler
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::ethereum, ShadowShared};
///
/// // GET `/ethereum/receipt/0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a/66666`
/// ethereum::receipt(web::Path::from((
///     "0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a".to_string(),
///      0 as u64,
/// )), web::Data::new(ShadowShared::new(None)));
/// ```
pub async fn handle(
    tx: web::Path<(String, u64)>,
    shared: web::Data<ShadowShared>,
) -> impl Responder {
    web::Json(ReceiptResp::new(&shared, tx.0.as_str(), tx.1).await)
}
