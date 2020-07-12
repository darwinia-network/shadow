use scale::{Decode, Encode};
use super::H512;

#[derive(Encode, Decode)]
pub struct DoubleNodeWithMerkleProof {
    pub dag_nodes: H512,
    pub proof: Vec<[u8;16]>,
}

impl Default for DoubleNodeWithMerkleProof {
    fn default() -> DoubleNodeWithMerkleProof {
        DoubleNodeWithMerkleProof {
            dag_nodes: [[0; 64]; 2]>,
            proof: Vec::new(),
        }
    }
}
