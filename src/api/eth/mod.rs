//! Ethereum API
mod ffi;
mod proof;
mod proposal;
mod receipt;

pub use self::{
    proof::handle as proof,
    proposal::{handle as proposal, ProposalReq},
    receipt::handle as receipt,
};
