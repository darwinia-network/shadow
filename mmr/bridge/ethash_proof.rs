use super::{H128, H512};
use scale::{Decode, Encode};

/// Ethash proof
#[derive(Encode, Decode, Debug, PartialEq, Eq)]
pub struct DoubleNodeWithMerkleProof {
    /// Dag nodes
    pub dag_nodes: [H512; 2],
    /// Merkle Proofs
    pub proof: Vec<H128>,
}

impl DoubleNodeWithMerkleProof {
    /// Generate DoubleNodeWithMerkleProof from hex array
    pub fn from_tuple(dag_nodes: [&str; 2], proof: [&str; 23]) -> DoubleNodeWithMerkleProof {
        DoubleNodeWithMerkleProof {
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

impl Default for DoubleNodeWithMerkleProof {
    fn default() -> DoubleNodeWithMerkleProof {
        DoubleNodeWithMerkleProof {
            dag_nodes: <[H512; 2]>::default(),
            proof: Vec::new(),
        }
    }
}
