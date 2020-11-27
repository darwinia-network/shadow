use super::helper::WebResult;
use crate::{mmr::helper as mmr_helper, ShadowShared};
use actix_web::{error, web};
use primitives::chain::ethereum::{EthereumHeaderJson, MMRProofJson};

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
    /// Generate header
    /// mmr_root_height should be last confirmed block in relayt
    pub async fn new(
        shared: &ShadowShared,
        tx: &str,
        mmr_root_height: u64,
    ) -> super::helper::WebResult<ReceiptResp> {
        let receipt_proof: ReceiptProof = super::ffi::receipt(shared.eth.rpc(), tx).into();
        let header = super::helper::header_by_hash(&receipt_proof.header_hash, &shared).await?;
        let mmr_proof = if mmr_root_height > 0 {
            let (member_leaf_index, last_leaf_index) = (header.number, mmr_root_height - 1);
            MMRProofJson {
                member_leaf_index,
                last_leaf_index,
                proof: mmr_helper::gen_proof(&shared.store, member_leaf_index, last_leaf_index),
            }
        } else {
            return Err(error::ErrorInternalServerError(format!(
                "Get mmr proof of failed, member_leaf_index: {}, last_leaf_index: {}",
                header.number,
                mmr_root_height - 1
            )));
        };

        Ok(ReceiptResp {
            header,
            receipt_proof,
            mmr_proof,
        })
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
) -> WebResult<web::Json<ReceiptResp>> {
    Ok(web::Json(
        ReceiptResp::new(&shared, tx.0.as_str(), tx.1).await?,
    ))
}
