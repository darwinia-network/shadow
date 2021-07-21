use mmr::{Database, build_client};
use actix_web::{
    web::{Data, Json},
    Responder
};
use primitives::{
    chain::ethereum::{EthashProof, EthashProofJson, EthereumRelayProofsJson},
    rpc::EthereumRPC,
};
use codec::{Decode, Encode};
use crate::{Result, AppData};
use serde::{Serialize, Deserialize};
use crate::error::ErrorJson;
use array_bytes::bytes;

/// Proof result
#[derive(Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ProofResult {
    EthereumRelayProofs(EthereumRelayProofsJson),
    Error(ErrorJson)
}

/// Proposal post req
#[derive(Deserialize, Encode)]
pub struct ProposalReq {
    /// MMR leaves
    pub member: u64,
    /// The target proposal block
    pub target: u64,
    /// The last leaf of mmr
    pub last_leaf: u64,
    // Block or Unblock for generate ethash
    // pub block: bool,
}

impl ProposalReq {
    /// Get `EtHashProof`
    fn ethash_proof(&self, api: &str) -> Result<Vec<EthashProofJson>> {
        let proof = ffi::proof(api, self.target, false)?;
        let proof_vec_u8 = bytes(proof.as_str())?;
        let result = <Vec<EthashProof>>::decode(&mut proof_vec_u8.as_ref())?
            .iter()
            .map(|p| Into::<EthashProofJson>::into(p.clone()))
            .collect();
        Ok(result)
    }

    /// Generate mmr proof
    pub fn mmr_proof(&self, mmr_db: &Database) -> Result<Vec<String>> {
        let client = build_client(mmr_db)?;
        let member = self.member;
        let last_leaf = self.last_leaf;
        client.gen_proof(member, last_leaf).map_err(|err| err.into())
    }

    /// Generate response
    pub async fn gen(&self, mmr_db: &Database, eth: &EthereumRPC) -> Result<EthereumRelayProofsJson> {
        let result = EthereumRelayProofsJson {
            ethash_proof: self.ethash_proof(eth.rpc())?,
            mmr_proof: self.mmr_proof(mmr_db)?,
        };
        Ok(result)
    }
}

/// Proposal Handler
pub async fn handle(req: Json<ProposalReq>, app_data: Data<AppData>) -> impl Responder {
    match req.0.gen(&app_data.mmr_db, &app_data.eth).await {
        Ok(result) => Json(ProofResult::EthereumRelayProofs(result)),
        Err(err) => Json(ProofResult::Error(err.to_json()))
    }
}
