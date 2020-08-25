//! Relay primitives
use super::{DoubleNodeWithMerkleProof, EthHeader};
use scale::{Decode, Encode};

/// Darwinia eth relay header thing
#[derive(Decode, Encode, Default)]
pub struct HeaderThing {
    eth_header: EthHeader,
    ethash_proof: Vec<DoubleNodeWithMerkleProof>,
    mmr_root: [u8; 32],
    mmr_proof: Vec<[u8; 32]>,
}
