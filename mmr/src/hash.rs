//! MMR Hashes
use blake2_rfc::blake2b::blake2b;
use cmmr::Merge;
use array_bytes::{bytes, hex_str};
use anyhow::anyhow;
use crate::error::Result;

pub struct H256;

impl H256 {
    pub fn from(s: &str) -> Result<[u8; 32]> {
        let bytes: Vec<u8> = bytes(s)?;
        H256::from_bytes(&bytes)
    }

    pub fn from_bytes(b: &[u8]) -> Result<[u8; 32]> {
        if b.len() != 32 {
            Err(anyhow!("Length wrong"))?
        } else {
            let mut h = [0; 32];
            h.copy_from_slice(b);
            Ok(h)
        }
    }

    pub fn hex(bytes: &[u8; 32]) -> String {
        hex_str(bytes)
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
