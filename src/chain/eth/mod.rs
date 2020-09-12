//! Ethereum types
mod confirmation;
mod ethash_proof;
mod header;
mod mmr_proof;

pub use self::{
    confirmation::get as confirmation,
    ethash_proof::{EthashProof, EthashProofJson},
    header::{EthHeader, EthHeaderJson, EthHeaderRPCResp},
    mmr_proof::{HeaderStuffs, MMRProof, MMRProofJson},
};
