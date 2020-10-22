use crate::{
    api::ShadowShared,
    mmr::{helper, MergeHash, Store, H256},
};
use actix_web::{web, Responder};
use cmmr::MMR;
use primitives::{
    bytes,
    chain::ethereum::{EthashProof, EthashProofJson, EthereumHeaderJson, EthereumRelayProofsJson},
};
use scale::{Decode, Encode};

/// Proposal post req
#[derive(Deserialize, Encode)]
pub struct ProposalReq {
    /// MMR leaves
    pub member: u64,
    /// The target proposal block
    pub target: u64,
    /// The last leaf of mmr
    pub last_leaf: u64,
}

impl ProposalReq {
    /// Get `EtHashProof`
    fn ethash_proof(&self) -> Vec<EthashProofJson> {
        let proof = super::ffi::proof(self.target);
        <Vec<EthashProof>>::decode(&mut bytes!(proof.as_str()).as_ref())
            .unwrap_or_default()
            .iter()
            .map(|p| Into::<EthashProofJson>::into(p.clone()))
            .collect()
    }

    /// Get mmr root
    pub fn mmr_root(&self, store: &Store) -> String {
        if self.target < 1 {
            "0x0000000000000000000000000000000000000000000000000000000000000000".into()
        } else {
            format!(
                "0x{}",
                H256::hex(
                    &MMR::<_, MergeHash, _>::new(
                        cmmr::leaf_index_to_mmr_size(self.target - 1),
                        store
                    )
                    .get_root()
                    .unwrap_or_default()
                )
            )
        }
    }

    /// Generate mmr proof
    pub fn mmr_proof(&self, store: &Store) -> Vec<String> {
        if self.last_leaf < 1 {
            return vec![];
        }

        helper::gen_proof(store, self.member, self.last_leaf)
    }

    /// Generate response
    pub async fn gen(&self, shared: web::Data<ShadowShared>) -> EthereumRelayProofsJson {
        EthereumRelayProofsJson {
            ethash_proof: self.ethash_proof(),
            mmr_proof: self.mmr_proof(&shared.store),
        }
    }
}

/// Proposal Headers
#[derive(Serialize, Encode)]
pub struct ProposalHeader {
    header: EthereumHeaderJson,
    ethash_proof: Vec<EthashProofJson>,
    mmr_root: String,
    mmr_proof: Vec<String>,
}

/// Proposal Handler
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::eth, ShadowShared};
///
/// // POST `/eth/proof`
/// eth::proposal(web::Json(eth::ProposalReq{
///     member: 10,
///     target: 19,
///     last_leaf: 18
/// }), web::Data::new(ShadowShared::new(None)));
/// ```
pub async fn handle(req: web::Json<ProposalReq>, share: web::Data<ShadowShared>) -> impl Responder {
    web::Json(req.0.gen(share).await)
}
