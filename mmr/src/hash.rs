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
            Err(anyhow!("Length wrong").into())
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

#[test]
fn test_merge() {
    let lhs: [u8; 32] = H256::from("34f61bfda344b3fad3c3e38832a91448b3c613b199eb23e5110a635d71c13c65").unwrap();
    let rhs: [u8; 32] = H256::from("70d641860d40937920de1eae29530cdc956be830f145128ebb2b496f151c1afb",).unwrap();
    assert_eq!(
        "3aafcc7fe12cb8fad62c261458f1c19dba0a3756647fa4e8bff6e248883938be",
        H256::hex(&MergeHash::merge(&lhs, &rhs))
    );
}

