//! MMR Hashes
use crate::{bytes, hex};
use blake2_rfc::blake2b::blake2b;
use cmmr::Merge;

/// H256 trait for `[u8;32]`
pub trait H256 {
    /// Generate `H256` from `&str`
    fn from(s: &str) -> Self;
    /// Convert `H256` bytes to hex string
    fn hex(&self) -> String;
}

impl H256 for [u8; 32] {
    fn from(s: &str) -> Self {
        bytes!(s, 32)
    }

    fn hex(&self) -> String {
        hex!(self)
    }
}

/// BlakeTwo256 hash function
pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut dest = [0; 32];
    dest.copy_from_slice(blake2b(32, &[], data).as_bytes());
    dest
}

/// MMR Merge trait for ETHash
pub struct MergeHash;
impl Merge for MergeHash {
    type Item = [u8; 32];
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        let mut data = vec![];
        data.append(&mut lhs.to_vec());
        data.append(&mut rhs.to_vec());
        hash(&data.as_slice())
    }
}
