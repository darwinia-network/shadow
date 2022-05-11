use actix_web::{
    web::{Data, Json},
    Responder,
};
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use shadow_types::{
    chain::ethereum::ethash::{EthashProof, EthashProofJson},
    rpc::EthereumRPC,
};

use crate::error::ErrorJson;
use crate::{AppData, Result};

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EthashProofsJson {
    /// Ethereum Hash Proof
    pub ethash_proof: Vec<EthashProofJson>,
}

/// Proof result
#[derive(Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ProofResult {
    EthashProofs(EthashProofsJson),
    Error(ErrorJson),
}

/// Proposal post req
#[derive(Deserialize, Encode)]
pub struct ProposalReq {
    /// The target proposal block
    pub target: u64,
}

impl ProposalReq {
    /// Get `EtHashProof`
    fn ethash_proof(&self, api: &str) -> Result<Vec<EthashProofJson>> {
        let proof = ffi::proof(api, self.target, false)?;
        let proof_vec_u8 = array_bytes::hex2bytes(proof.as_str())?;
        let result = <Vec<EthashProof>>::decode(&mut proof_vec_u8.as_ref())?
            .iter()
            .map(|p| Into::<EthashProofJson>::into(p.clone()))
            .collect();
        Ok(result)
    }

    /// Generate response
    pub async fn gen(&self, eth: &EthereumRPC) -> Result<EthashProofsJson> {
        let result = EthashProofsJson {
            ethash_proof: self.ethash_proof(eth.rpc())?,
        };
        Ok(result)
    }
}

/// Proposal Handler
pub async fn handle(req: Json<ProposalReq>, app_data: Data<AppData>) -> impl Responder {
    match req.0.gen(&app_data.eth).await {
        Ok(result) => Json(ProofResult::EthashProofs(result)),
        Err(err) => Json(ProofResult::Error(err.to_json())),
    }
}
