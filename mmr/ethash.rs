use cmmr::Merge;

/// The root log id of mmr
pub const MMR_ROOT_LOG_ID: [u8; 4] = *b"MMRR";

/// Simple ETHash
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ETHash {
    pub prefix: [u8; 4],
    pub hash: [u8; 32],
}

impl From<&str> for ETHash {
    fn from(s: &str) -> ETHash {
        let mut hash = [0_u8; 32];
        let bytes = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .unwrap_or_default();
        hash.copy_from_slice(&bytes);
        ETHash {
            prefix: MMR_ROOT_LOG_ID,
            hash: hash,
        }
    }
}

/// MMR Merge trait for ETHash
pub struct MergeETHash;
impl Merge for MergeETHash {
    type Item = ETHash;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        let mut hash: [u8; 32] = [0; 32];
        blake2b_rs::blake2b(&lhs.hash, &rhs.hash, &mut hash);
        ETHash {
            prefix: MMR_ROOT_LOG_ID,
            hash: hash,
        }
    }
}
