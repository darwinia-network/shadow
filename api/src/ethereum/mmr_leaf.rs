//! Ethereum MMR API
use mmr::{Database, build_client};
use actix_web::{
    web::{Data, Path, Json},
    Responder
};
use crate::{Result, Error, AppData};
use serde::{Serialize};

/// MMR leaf result
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MMRLeafResult {
    MMRLeaf { mmr_leaf: String },
    Error { error: String }
}

/// Get target mmr
#[allow(clippy::eval_order_dependence)]
pub async fn handle(block: Path<String>, app_data: Data<AppData>) -> impl Responder {
    match mmr_leaf(block, &app_data.mmr_db) {
        Ok(leaf) => Json(MMRLeafResult::MMRLeaf { mmr_leaf: format!("0x{}", leaf) }),
        Err(err) => Json(MMRLeafResult::Error { error: err.to_string() })
    }
}

fn mmr_leaf(block: Path<String>, mmr_db: &Database) -> Result<String> {
    let leaf_index: u64 = block.to_string().parse()?;
    let client = build_client(mmr_db)?;
    let result = client.get_leaf(leaf_index)?;
    if let Some(leaf) = result {
        Ok(leaf)
    } else {
        Err(Error::LeafNotFound(leaf_index))
    }
}
