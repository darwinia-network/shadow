//! Ethereum API
mod ffi;
mod proposal;
mod receipt;

pub use self::{
    proposal::{handle as proposal, ProposalReq},
    receipt::handle as receipt,
};
