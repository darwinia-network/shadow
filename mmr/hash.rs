use cmmr::Merge;

/// H256 trait for `[u8;32]`
pub trait H256 {
    fn from(s: &str) -> Self;
    fn to_hex(&self) -> String;
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

    fn to_hex(&self) -> String {
        let mut s = String::new();
        for i in self {
            s.push_str(&format!("{:02x}", i));
        }
        s
    }
}

/// MMR Merge trait for ETHash
pub struct MergeETHash;
impl Merge for MergeETHash {
    type Item = [u8; 32];
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        let mut hash: [u8; 32] = [0; 32];
        blake2b_rs::blake2b(lhs, rhs, &mut hash);
        hash
    }
}
