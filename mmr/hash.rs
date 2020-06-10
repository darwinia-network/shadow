use blake2_rfc::blake2b::blake2b;
use cmmr::Merge;

/// H256 trait for `[u8;32]`
pub trait H256 {
    fn from(s: &str) -> Self;
    fn hex(&self) -> String;
}

impl H256 for [u8; 32] {
    fn from(s: &str) -> Self {
        let mut hash = [0_u8; 32];
        let bytes = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .unwrap_or_default();
        hash.copy_from_slice(&bytes);
        hash
    }

    fn hex(&self) -> String {
        let mut s = String::new();
        for i in self {
            s.push_str(&format!("{:02x}", i));
        }
        s
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
