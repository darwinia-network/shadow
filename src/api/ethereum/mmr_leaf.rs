//! Ethereum MMR API
use super::helper::WebResult;
use crate::ShadowShared;
use actix_web::{error, web};
use cmmr::MMRStore;
use primitives::{bytes, hex};

/// Single MMR struct
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MMRLeaf {
    /// MMR Leaf
    pub mmr_leaf: [u8; 32],
}

/// MMR Root Json
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MMRLeafJson {
    /// MMR leaf string
    pub mmr_leaf: String,
}

impl Into<MMRLeaf> for MMRLeafJson {
    fn into(self) -> MMRLeaf {
        MMRLeaf {
            mmr_leaf: bytes!(self.mmr_leaf.as_str(), 32),
        }
    }
}

impl Into<MMRLeafJson> for MMRLeaf {
    fn into(self) -> MMRLeafJson {
        MMRLeafJson {
            mmr_leaf: primitives::hex!(&self.mmr_leaf),
        }
    }
}

/// Get target mmr
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::ethereum, ShadowShared};
///
/// // GET `/ethereum/mmr_leaf/19`
/// ethereum::mmr_leaf(web::Path::from("19".to_string()), web::Data::new(ShadowShared::new(None)));
/// ```
#[allow(clippy::eval_order_dependence)]
pub async fn handle(
    block: web::Path<String>,
    shared: web::Data<ShadowShared>,
) -> WebResult<web::Json<MMRLeafJson>> {
    let num: u64 = block.to_string().parse().unwrap_or(0);

    (&shared.store)
        .get_elem(cmmr::leaf_index_to_pos(num))
        .and_then(|elem: Option<[u8; 32]> | {
            elem.ok_or(
                cmmr::Error::StoreError(format!("No leaf of index {} found in store", num))
            )
        })
        .map(|leaf| format!("0x{}", hex!(&leaf)))
        .map(|mmr_leaf| web::Json(MMRLeafJson { mmr_leaf }))
        .map_err(|err| {
            error::ErrorInternalServerError(format!(
                "Get mmr leaf of {} failed, caused by: {}",
                block, err.to_string()
            ))
        })
}
