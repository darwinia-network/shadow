//! Ethereum MMR API
use mmr::{Database, build_client};
use actix_web::{
    web::{Data, Path, Json},
    Responder
};
use crate::{Result, Error, AppData};
use serde::Serialize;

/// MMR root result
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MMRRootResult {
    MmrRoot { mmr_root: String },
    Error { error: String }
}

/// Get target mmr
#[allow(clippy::eval_order_dependence)]
pub async fn handle(block: Path<String>, app_data: Data<AppData>) -> impl Responder {
    match parent_mmr_root(block, &app_data.mmr_db) {
        Ok(root) => Json(MMRRootResult::MmrRoot { mmr_root: format!("0x{}", root) }),
        Err(err) => Json(MMRRootResult::Error { error: err.to_string() })
    }
}

fn parent_mmr_root(block: Path<String>, mmr_db: &Database) -> Result<String> {
    let block: u64 = block.to_string().parse()?;
    let leaf_index = block - 1;
    let client = build_client(mmr_db)?;
    let result = client.get_mmr_root(leaf_index)?;
    if let Some(root) = result {
        Ok(root)
    } else {
        Err(Error::MmrRootNotFound(leaf_index))
    }
}
