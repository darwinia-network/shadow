use super::helper::WebResult;
use crate::{mmr::helper as mmr_helper, ShadowShared};
use actix_web::{error, web};
use primitives::chain::ethereum::{EthereumHeaderJson, EthereumReceipt, MMRProofJson};
use waitgroup::WaitGroup;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use mpt::{trie::Trie, MerklePatriciaTrie, MemoryDB};
use std::rc::Rc;

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

async fn get_receipt_proof(txhash: &str, shared: &ShadowShared) -> super::helper::WebResult<ReceiptProof> {
    let receipt = super::helper::receipt(txhash, shared).await?;
    let index = u64::from_str_radix(&receipt.transaction_index.as_str()[2..], 16).unwrap_or_default();
    let header = super::helper::header_by_hash(&receipt.block_hash, shared).await?;

    let receipts_ref = Arc::new(Mutex::new(HashMap::new()));
    let wg = WaitGroup::new();
    for hash in &header.transactions {
        let w = wg.worker();
        let s = shared.clone();
        let h = hash.to_string();
        let receipts = receipts_ref.clone();
        actix_rt::spawn(async move {
            let r = super::helper::receipt(&h, &s).await;
            if let Ok(ret) = r {
                let mut receipt_map = receipts.lock().unwrap();
                let index = u64::from_str_radix(&ret.transaction_index.as_str()[2..], 16).unwrap_or_default();
                let ethereum_receipt: EthereumReceipt = ret.into();
                receipt_map.insert(index, ethereum_receipt);
            }
            drop(w);
        });
    }
    wg.wait().await;
    let receipts = receipts_ref.lock().unwrap();
    if receipts.len() != header.transactions.len() {
        return Err(error::ErrorInternalServerError(format!(
            "get receipts failed: {}, last_leaf_index: {}",
            receipts.len(),
            header.transactions.len()
        )));
    }

    let memdb = Rc::new(MemoryDB::new());
    let mut trie = MerklePatriciaTrie::new(memdb);
    for (index, r) in receipts.iter() {
        let path = rlp::encode(index);
        let raw_receipt = rlp::encode(r);
        trie.insert(path, raw_receipt).unwrap();
    }
    let key = rlp::encode(&index);
    // root commit the trie
    let _root = trie.root().unwrap();
    let proof = trie.get_proof(&key).unwrap();
    //let value = MerklePatriciaTrie::verify_proof(root.clone(), &key, proof)
    let hash_proof = String::from("0x") + &hex::encode(rlp::encode(&proof));

    Ok(
        ReceiptProof {
            index: receipt.transaction_index,
            proof: hash_proof,
            header_hash: receipt.block_hash,
        }
    )
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
        let receipt_proof = get_receipt_proof(tx, shared).await?;
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
