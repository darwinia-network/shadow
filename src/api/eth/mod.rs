//! Ethereum API
pub mod ffi;
mod proposal;
mod receipt;

pub use proposal::handle as proposal;
pub use receipt::handle as receipt;
