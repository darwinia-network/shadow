//! Api helpers
use crate::{
    mmr::{MergeHash, H256},
    ShadowShared,
};
use actix_web::error;
use cmmr::MMR;
use primitives::{chain::ethereum::EthereumHeaderJson, rpc::RPC};
use primitives::{chain::ethereum::EthReceiptBody};

/// Web result
pub type WebResult<R> = Result<R, error::Error>;

/// Get parent_mmr_root string with web response
/// block's parent is leaf index
pub fn parent_mmr_root(block: u64, shared: &ShadowShared) -> WebResult<String> {
    let num: u64 = block.to_string().parse().unwrap_or(0);
    if num == 0 {
        return Err(error::ErrorBadRequest("Requesting mmr_root of block 0"));
    }

    match MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(num - 1), &shared.store).get_root() {
        Ok(hash_bytes) => {
            Ok(format!("0x{}", H256::hex(&hash_bytes)))
        },
        Err(err) => {
            Err(error::ErrorInternalServerError(format!(
                "Get mmr root of block {}'s parent failed, caused by {}",
                num, err.to_string()
            )))
        }
    }
}

/// Get header json with web response
pub async fn header(block: u64, shared: &ShadowShared) -> WebResult<EthereumHeaderJson> {
    shared.eth
        .get_header_by_number(block).await
        .map(|header| header.into())
        .map_err(|err| {
            error::ErrorInternalServerError(format!(
                "Get block header {} failed, caused by {}",
                block, err.to_string()
            ))
        })
}

/// Get header json with web response
pub async fn header_by_hash(block: &str, shared: &ShadowShared) -> WebResult<EthereumHeaderJson> {
    shared.eth.
        get_header_by_hash(block).await
        .map(|header| header.into())
        .map_err(|err| {
            error::ErrorInternalServerError(format!(
                "Get block header {} failed, caused by {}",
                block, err.to_string()
            ))
        })
}

/// Get receipt json with web response
pub async fn receipt(txhash: &str, shared: &ShadowShared) -> WebResult<EthReceiptBody> {
    shared.eth.
        get_receipt(txhash).await
        .map_err(|err| {
            error::ErrorInternalServerError(format!(
                "Get receipt {} failed, caused by {}",
                txhash, err.to_string()
            ))
        })
}

