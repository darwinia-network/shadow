use super::helper::WebResult;
use crate::{
    api::ShadowShared,
    mmr::{helper as mmr_helper, Store},
};
use actix_web::{error, web};
use primitives::{
    bytes,
    chain::ethereum::{EthashProof, EthashProofJson, EthereumRelayProofsJson},
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
    fn ethash_proof(&self, api: &str) -> WebResult<Vec<EthashProofJson>> {
        let proof = super::ffi::proof(api, self.target);
        if let Ok(ethash_proof_vector) =
            <Vec<EthashProof>>::decode(&mut bytes!(proof.as_str()).as_ref())
        {
            Ok(ethash_proof_vector
                .iter()
                .map(|p| Into::<EthashProofJson>::into(p.clone()))
                .collect())
        } else {
            Err(error::ErrorInternalServerError(format!(
                "Get ethash proof of block {} failed",
                self.target
            )))
        }
    }

    /// Generate mmr proof
    pub fn mmr_proof(&self, store: &Store) -> Vec<String> {
        if self.last_leaf < 1 {
            return vec![];
        }

        mmr_helper::gen_proof(store, self.member, self.last_leaf)
    }

    /// Generate response
    pub async fn gen(&self, shared: web::Data<ShadowShared>) -> WebResult<EthereumRelayProofsJson> {
        Ok(EthereumRelayProofsJson {
            ethash_proof: self.ethash_proof(shared.eth.rpc())?,
            mmr_proof: self.mmr_proof(&shared.store),
        })
    }
}

/// Proposal Handler
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::ethereum, ShadowShared};
///
/// // POST `/ethereum/proof`
/// ethereum::proof(web::Json(ethereum::ProposalReq{
///     member: 10,
///     target: 19,
///     last_leaf: 18
/// }), web::Data::new(ShadowShared::new(None)));
/// ```
pub async fn handle(
    req: web::Json<ProposalReq>,
    share: web::Data<ShadowShared>,
) -> WebResult<web::Json<EthereumRelayProofsJson>> {
    Ok(web::Json(req.0.gen(share).await?))
}
