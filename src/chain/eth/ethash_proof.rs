#![allow(dead_code)]
use crate::chain::array::{H128, H512};
use scale::{Decode, Encode};

/// Ethash proof
#[derive(Encode, Decode, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct EthashProof {
    /// Dag nodes
    pub dag_nodes: [H512; 2],
    /// Merkle Proofs
    pub proof: Vec<H128>,
}

impl EthashProof {
    /// Generate EthashProof from hex array
    pub fn from_tuple(dag_nodes: [&str; 2], proof: [&str; 23]) -> EthashProof {
        EthashProof {
            dag_nodes: [
                H512(bytes!(dag_nodes[0], 64)),
                H512(bytes!(dag_nodes[1], 64)),
            ],
            proof: proof
                .iter()
                .map(|s| H128(bytes!(*s, 16)))
                .collect::<Vec<H128>>(),
        }
    }
}
