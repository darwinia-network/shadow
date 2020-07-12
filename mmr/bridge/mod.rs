//! format bridge
use scale::{Decode, Encode};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// macros
mod util;

// types
// mod ethash_proof;
mod header;

// shared
use uint::construct_uint;
construct_uint! {
    #[derive(Encode, Decode)]
    pub struct U256(4);
}

construct_hash_bytes! {
    pub struct H512(64);
}

construct_hash_bytes! {
    pub struct Bloom(256);
}

// #[derive(Encode, Decode)]
// pub struct H512(pub [u8; 64]);
//
// impl PartialEq for H512 {
//     fn eq(&self, other: &Self) -> bool {
//         true
//     }
// }

pub use header::EthHeader;
