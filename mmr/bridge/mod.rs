//! format bridge
use scale::{Decode, Encode};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// macros
mod util;

// types
mod ethash_proof;
mod header;

// shared
use uint::construct_uint;
construct_uint! {
    #[derive(Encode, Decode)]
    pub struct U256(4);
}

construct_hash_bytes! {
    pub struct H128(16);
}

construct_hash_bytes! {
    pub struct H512(64);
}

construct_hash_bytes! {
    pub struct Bloom(256);
}

pub use self::{ethash_proof::DoubleNodeWithMerkleProof, header::EthHeader};
