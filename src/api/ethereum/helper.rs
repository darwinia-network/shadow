//! Api helpers
use crate::{
    mmr::{MergeHash, H256},
    ShadowShared,
};
use actix_web::error;
use cmmr::MMR;

// Get mmr_root string with web response
pub fn mmr_root(block: u64, shared: &ShadowShared) -> Result<String, error::Error> {
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
