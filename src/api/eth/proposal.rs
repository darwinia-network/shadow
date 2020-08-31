use crate::{
    bytes,
    chain::eth::{EthHeader, EthHeaderJson, EthashProof, EthashProofJson},
    db::pool,
    mmr::{MergeHash, Store, H256},
};
use actix_web::{web, Responder};
use cmmr::MMR;
use reqwest::Client;
use scale::Decode;

/// Proposal post req
#[derive(Deserialize)]
pub struct ProposalReq {
    /// MMR members
    pub members: Vec<u64>,
    /// The last leaf of mmr proof
    pub target: u64,
}

impl ProposalReq {
    /// Get `EthHeader`
    async fn eth_header(client: &Client, block: u64) -> EthHeaderJson {
        EthHeader::get(&client, block)
            .await
            .unwrap_or_default()
            .into()
    }

    /// Get `EtHashProof`
    fn ethash_proof(block: u64) -> Vec<EthashProofJson> {
        let proof = super::ffi::proof(block);
        <Vec<EthashProof>>::decode(&mut bytes!(proof.as_str()).as_ref())
            .unwrap_or_default()
            .iter()
            .map(Into::<EthashProofJson>::into)
            .collect()
    }

    // Get mmr root
    fn mmr_root(store: &Store, leaf: u64) -> String {
        let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(leaf), store);
        format!("0x{}", H256::hex(&mmr.get_root().unwrap_or_default()))
    }

    /// Generate mmr proof
    fn mmr_proof(&self, store: &Store, member: u64) -> Vec<String> {
        let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(self.target - 1), store);
        match mmr.gen_proof(vec![cmmr::leaf_index_to_pos(member)]) {
            Err(e) => {
                error!(
                    "Generate proof failed {:?}, last_leaf: {:?}, member: {:?}",
                    e,
                    self.target - 1,
                    member
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
    pub async fn headers(&self) -> Vec<ProposalHeader> {
        // TODO: optimzie the `clients` below
        //
        // Move them out of this handler
        let conn = pool::conn(None);
        let store = Store::with(conn);
        let client = Client::new();

        // Proposal Headers
        let mut phs = vec![];
        for m in self.members.iter() {
            phs.push(ProposalHeader {
                eth_header: ProposalReq::eth_header(&client, *m).await,
                ethash_proof: ProposalReq::ethash_proof(*m),
                mmr_root: ProposalReq::mmr_root(&store, *m),
                mmr_proof: ProposalReq::mmr_proof(&self, &store, *m),
            });
        }
        phs
    }
}

/// Proposal Headers
#[derive(Serialize)]
pub struct ProposalHeader {
    eth_header: EthHeaderJson,
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
///     members: vec![19],
///     last_leaf: 19,
/// }));
/// ```
pub async fn handle(req: web::Json<ProposalReq>) -> impl Responder {
    web::Json(req.0.headers().await)
}
