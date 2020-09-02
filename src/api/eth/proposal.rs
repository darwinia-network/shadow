use crate::{
    api::ShadowShared,
    bytes,
    chain::eth::{EthHeader, EthHeaderJson, EthashProof, EthashProofJson},
    mmr::{MergeHash, Store, H256},
};
use actix_web::{web, Responder};
use cmmr::MMR;
use reqwest::Client;
use scale::{Decode, Encode};

/// Proposal post req
#[derive(Deserialize, Encode)]
pub struct ProposalReq {
    /// MMR leaves
    pub leaves: Vec<u64>,
    /// The target proposal block
    pub target: u64,
    /// The last leaf of mmr
    pub last_leaf: u64,
}

impl ProposalReq {
    /// Get `EthHeader`
    async fn header(&self, client: &Client) -> EthHeaderJson {
        EthHeader::get(&client, self.target)
            .await
            .unwrap_or_default()
            .into()
    }

    /// Get `EtHashProof`
    fn ethash_proof(&self) -> Vec<EthashProofJson> {
        let proof = super::ffi::proof(self.target);
        <Vec<EthashProof>>::decode(&mut bytes!(proof.as_str()).as_ref())
            .unwrap_or_default()
            .iter()
            .map(Into::<EthashProofJson>::into)
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
    pub async fn mmr_proof(&self, store: &Store) -> Vec<String> {
        if self.last_leaf < 1 || self.leaves.is_empty() {
            return vec![];
        }

        match MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(self.last_leaf), store)
            .gen_proof(
                self.leaves
                    .iter()
                    .map(|l| cmmr::leaf_index_to_pos(*l))
                    .collect(),
            ) {
            Err(e) => {
                error!(
                    "Generate proof failed {:?}, target: {:?}, leaves: {:?}",
                    e, self.target, self.leaves
                );
                vec![]
            }
            Ok(proof) => {
                let res = proof
                    .proof_items()
                    .iter()
                    .map(|item| format!("0x{}", H256::hex(item)))
                    .collect::<Vec<String>>();
                res
            }
        }
    }

    /// To headers
    pub async fn gen(&self, shared: web::Data<ShadowShared>) -> ProposalHeader {
        ProposalHeader {
            header: self.header(&shared.client).await,
            ethash_proof: self.ethash_proof(),
            mmr_root: self.mmr_root(&shared.store),
            mmr_proof: self.mmr_proof(&shared.store).await,
        }
    }
}

/// Proposal Headers
#[derive(Serialize, Encode)]
pub struct ProposalHeader {
    header: EthHeaderJson,
    ethash_proof: Vec<EthashProofJson>,
    mmr_root: String,
    mmr_proof: Vec<String>,
}

/// Proposal Handler
///
/// ```
/// use darwinia_shadow::api::eth;
/// use actix_web::web;
///
/// // POST `/eth/proposal`
/// eth::proposal(web::Json(eth::ProposalReq{
///     leaves: vec![10],
///     target: 19,
///     last_leaf: 18
/// }));
/// ```
pub async fn handle(req: web::Json<ProposalReq>, share: web::Data<ShadowShared>) -> impl Responder {
    web::Json(req.0.gen(share).await)
}
