//! ethereum

mod ethash_proof;
mod header;
mod relay;

pub use self::{
    ethash_proof::DoubleNodeWithMerkleProof,
    header::{EthHeader, EthHeaderRPCResp},
    relay::HeaderThing,
};
