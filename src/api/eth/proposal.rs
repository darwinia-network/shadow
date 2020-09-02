use crate::{
    bytes,
    chain::eth::{EthHeader, EthHeaderJson, EthashProof, EthashProofJson},
    db::pool,
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

    // Get mmr root
    fn mmr_root(&self, store: &Store) -> String {
        if self.target == 0 {
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
    fn mmr_proof(&self, store: &Store) -> Vec<String> {
        let last_leaf = *self.leaves.iter().max().unwrap_or(&self.target);
        if last_leaf < 1 {
            return vec![];
        }

        match MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(last_leaf), store).gen_proof(
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
            Ok(proof) => proof
                .proof_items()
                .iter()
                .map(|item| format!("0x{}", H256::hex(item)))
                .collect::<Vec<String>>(),
        }
    }

    /// To headers
    pub async fn gen(&self) -> ProposalHeader {
        // TODO: optimzie the `clients` below
        //
        // Move them out of this handler
        let conn = pool::conn(None);
        let store = Store::with(conn);
        let client = Client::new();

        // Proposal Header
        ProposalHeader {
            header: self.header(&client).await,
            ethash_proof: self.ethash_proof(),
            mmr_root: self.mmr_root(&store),
            mmr_proof: self.mmr_proof(&store),
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
///     leaves: vec![18],
///     target: 19,
/// }));
/// ```
pub async fn handle(req: web::Json<ProposalReq>) -> impl Responder {
    web::Json(req.0.gen().await)
}
