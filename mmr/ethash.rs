use cmmr::Merge;

/// Simple ETHash
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ETHash(pub [u8; 32]);
impl From<&str> for ETHash {
    fn from(s: &str) -> ETHash {
        let mut ret = [0_u8; 32];
        let bytes = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .unwrap_or_default();
        ret.copy_from_slice(&bytes);
        ETHash(ret)
    }
}

/// MMR Merge trait for ETHash
pub struct MergeETHash;
impl Merge for MergeETHash {
    type Item = ETHash;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        let mut res: [u8; 32] = [0; 32];
        blake2b_rs::blake2b(&lhs.0, &rhs.0, &mut res);
        ETHash(res)
    }
}
