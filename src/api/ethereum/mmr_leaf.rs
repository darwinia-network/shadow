//! Ethereum MMR API
use crate::{
    mmr::{H256},
    ShadowShared,
};
use actix_web::{web, Responder};
use cmmr::{MMRStore};
use primitives::bytes;

/// Single MMR struct
#[derive(Clone, Debug,Default, PartialEq, Eq)]
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
pub async fn handle(block: web::Path<String>, shared: web::Data<ShadowShared>) -> impl Responder {
    let num: u64 = block.to_string().parse().unwrap_or(0);

    let leaf = H256::hex(
        &(&shared.store)
            .get_elem(cmmr::leaf_index_to_pos(num))
            .unwrap().unwrap_or([0;32]),
    );

    web::Json(MMRLeafJson {
        mmr_leaf: format!("0x{}", leaf),
    })
}
