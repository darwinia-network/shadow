//! ethereum
use scale::{Decode, Encode};

mod confirmation;
mod ethash_proof;
mod header;

/// Darwinia eth relay header thing
#[derive(Decode, Encode, Default)]
pub struct HeaderThing {
    eth_header: EthHeader,
    ethash_proof: Vec<EthashProof>,
    mmr_root: [u8; 32],
    mmr_proof: Vec<[u8; 32]>,
}

pub use self::{
    confirmation::get as confirmation,
    ethash_proof::{EthashProof, EthashProofJson},
    header::{EthHeader, EthHeaderJson, EthHeaderRPCResp},
};
