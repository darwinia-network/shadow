use crate::{
    chain::eth::{EthHeader, EthHeaderJson},
    ShadowShared, mmr::{MergeHash, Store, H256},
};
use actix_web::{web, Responder};
use reqwest::Client;
use cmmr::MMR;

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
    header: EthHeaderJson,
    receipt_proof: ReceiptProof,
    mmr_proof: Vec<String>,
}

impl ReceiptResp {
    /// Get Receipt
    pub fn receipt(tx: &str) -> ReceiptProof {
        super::ffi::receipt(tx).into()
    }
    /// Get ethereum header json
    pub async fn header(client: &Client, block: &str) -> EthHeaderJson {
        EthHeader::get_by_hash(client, block)
            .await
            .unwrap_or_default()
            .into()
    }

    /// Get mmr proof
    pub fn mmr_proof(store: &Store, member: u64, mmr_root_height: u64) -> Vec<String> {
        if member >= mmr_root_height {
            return vec![];
        }

        match MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(mmr_root_height - 1), store)
            .gen_proof(
                [member]
                    .iter()
                    .map(|l| cmmr::leaf_index_to_pos(*l))
                    .collect(),
            ) {
            Err(e) => {
                error!(
                    "Generate proof failed {:?}, mmr_root_height: {:?}, member leaves: {:?}",
                    e, mmr_root_height, member
                );
                vec![]
            }
            Ok(proof) => proof
                .proof_items()
                .iter()
                .map(|item| format!("0x{}", H256::hex(item)))
                .collect::<Vec<String>>(),
        }
    }

    /// Generate header
    /// mmr_root_height should be last confirmed block in relay
    pub async fn new(shared: &ShadowShared, tx: &str, mmr_root_height: u64) -> ReceiptResp {
        let client = Client::new();
        let receipt_proof = Self::receipt(tx);
        let header = Self::header(&client, &receipt_proof.header_hash).await;
        let mmr_proof = Self::mmr_proof(&shared.store, header.number, mmr_root_height);
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
/// use darwinia_shadow::{api::eth, ShadowShared};
///
/// // GET `/eth/receipt/0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a`
/// eth::receipt(web::Path::from((
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
