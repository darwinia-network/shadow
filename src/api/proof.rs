use crate::{
    bytes,
    chain::eth::{DoubleNodeWithMerkleProof, EthHeader},
    hash::{MergeHash, H256},
    pool,
    store::Store,
};
use actix_web::{web, Responder};
use cmmr::MMR;
use reqwest::blocking::Client;
use scale::Decode;
use std::{ffi::CStr, os::raw::c_char};

/// Proposal post req
#[derive(Deserialize)]
pub struct ProposalReq {
    members: Vec<u64>,
    last_leaf: u64,
}

impl ProposalReq {
    /// Get `EthHeader`
    fn eth_header(client: &Client, block: u64) -> EthHeader {
        EthHeader::get(&client, block).unwrap_or_default()
    }

    /// Get `EtHashProof`
    fn ethash_proof(block: u64) -> Vec<DoubleNodeWithMerkleProof> {
        unsafe {
            let proof = CStr::from_ptr(Proof(block as u32))
                .to_string_lossy()
                .to_string();
            <Vec<DoubleNodeWithMerkleProof>>::decode(&mut bytes!(proof.as_str()).as_ref())
                .unwrap_or_default()
        }
    }

    // Get mmr root
    fn mmr_root(store: &Store, leaf: u64) -> String {
        let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(leaf), store);
        H256::hex(&mmr.get_root().unwrap_or_default())
    }

    /// Generate mmr proof
    fn mmr_proof(&self, store: &Store, member: u64) -> Vec<String> {
        let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(self.last_leaf), store);
        match mmr.gen_proof(vec![cmmr::leaf_index_to_pos(member)]) {
            Err(e) => {
                error!(
                    "Generate proof failed {:?}, last_leaf: {:?}, member: {:?}",
                    e, self.last_leaf, member
                );
                vec![]
            }
            Ok(proof) => proof
                .proof_items()
                .iter()
                .map(|item| H256::hex(item))
                .collect::<Vec<String>>(),
        }
    }
}

#[derive(Serialize)]
struct ProposalHeader {
    eth_header: EthHeader,
    ethash_proof: Vec<DoubleNodeWithMerkleProof>,
    mmr_root: String,
    mmr_proof: Vec<String>,
}

impl From<ProposalReq> for Vec<ProposalHeader> {
    fn from(p: ProposalReq) -> Vec<ProposalHeader> {
        // TODO: optimzie the `clients` below
        //
        // Move them out of this handler
        let conn = pool::conn(None);
        let store = Store::with(conn);
        let client = Client::new();

        // Proposal Headers
        let mut phs = vec![];
        for m in p.members.iter() {
            phs.push(ProposalHeader {
                eth_header: ProposalReq::eth_header(&client, *m),
                ethash_proof: ProposalReq::ethash_proof(*m),
                mmr_root: ProposalReq::mmr_root(&store, *m),
                mmr_proof: ProposalReq::mmr_proof(&p, &store, *m),
            });
        }
        phs
    }
}

pub async fn handle(req: web::Json<ProposalReq>) -> impl Responder {
    web::Json(Into::<Vec<ProposalHeader>>::into(req.0))
}

#[link(name = "eth")]
extern "C" {
    fn Proof(input: libc::c_uint) -> *const c_char;
}
