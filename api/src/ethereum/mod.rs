//! Ethereum API
mod ethash_proof;
mod receipt_proof;

pub use self::{
    ethash_proof::{handle as ethash_proof, ProposalReq},
    receipt_proof::{handle as receipt_proof, ReceiptResp},
};
