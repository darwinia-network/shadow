//! Api helpers
use crate::{
    mmr::{MergeHash, H256},
    ShadowShared,
};
use actix_web::error;
use cmmr::MMR;
use primitives::{chain::ethereum::EthereumHeaderJson, rpc::RPC};

/// Web result
pub type WebResult<R> = Result<R, error::Error>;

/// Get parent_mmr_root string with web response
/// block's parent is leaf index
pub fn parent_mmr_root(block: u64, shared: &ShadowShared) -> WebResult<String> {
    let num: u64 = block.to_string().parse().unwrap_or(0);
    if num == 0 {
        return Err(error::ErrorBadRequest("Requesting mmr_root of block 0"));
    }

    if let Ok(hash_bytes) =
        MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(num - 1), &shared.store).get_root()
    {
        Ok(format!("0x{}", H256::hex(&hash_bytes)))
    } else {
        Err(error::ErrorInternalServerError(format!(
            "Get mmr root of block {} failed",
            num
        )))
    }
}

/// Get header json with web response
pub async fn header(block: u64, shared: &ShadowShared) -> WebResult<EthereumHeaderJson> {
    if let Ok(h) = shared.eth.get_header_by_number(block).await {
        Ok(h.into())
    } else {
        return Err(error::ErrorInternalServerError(format!(
            "Get block header {} failed",
            block
        )));
    }
}

/// Get header json with web response
pub async fn header_by_hash(block: &str, shared: &ShadowShared) -> WebResult<EthereumHeaderJson> {
    if let Ok(h) = shared.eth.get_header_by_hash(block).await {
        Ok(h.into())
    } else {
        return Err(error::ErrorInternalServerError(format!(
            "Get block header {} failed",
            block
        )));
    }
}
