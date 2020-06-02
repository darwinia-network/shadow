use mmr::{util::MemStore, Merge, MMR};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Error, Formatter},
    hash::{Hash, Hasher},
};
const HEADERS: [[u8; 64]; 10] = [
    *b"d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3",
    *b"88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6",
    *b"b495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9",
    *b"3d6122660cc824376f11ee842f83addc3525e2dd6756b9bcf0affa6aa88cf741",
    *b"23adf5a3be0f5235b36941bcb29b62504278ec5b9cdfa277b992ba4a7a3cd3a2",
    *b"f37c632d361e0a93f08ba29b1a2c708d9caa3ee19d1ee8d2a02612bffe49f0a9",
    *b"1f1aed8e3694a067496c248e61879cda99b0709a1dfbacd0b693750df06b326e",
    *b"e0c7c0b46e116b874354dce6f64b8581bd239186b03f30a978e3dc38656f723a",
    *b"2ce94342df186bab4165c268c43ab982d360c9474f429fec5565adfc5d1f258b",
    *b"997e47bf4cac509c627753c06385ac866641ec6f883734ff7944411000dc576e",
];

#[derive(Clone)]
struct NumberHash(pub [u8; 64]);
impl PartialEq for NumberHash {
    fn eq(&self, rhs: &Self) -> bool {
        for (i, v) in self.0.iter().enumerate() {
            if v != &rhs.0[i] {
                return false;
            }
        }

        true
    }
}

impl Debug for NumberHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("")
    }
}

impl Eq for NumberHash {}

struct MergeNumberHash;
impl Merge for MergeNumberHash {
    type Item = NumberHash;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        let mut hasher = DefaultHasher::new();
        let src = |x: &NumberHash, y: &NumberHash| {
            let [mut alpha, mut beta, mut gamma, mut delta] = [[0_u8; 32]; 4];
            alpha.clone_from_slice(&x.0[0..32]);
            beta.clone_from_slice(&x.0[32..]);
            gamma.clone_from_slice(&y.0[0..32]);
            delta.clone_from_slice(&y.0[32..]);

            [alpha, beta, gamma, delta]
        };

        let mut res = [0_u8; 64];
        Hash::hash_slice(&src(lhs, rhs), &mut hasher);
        res.copy_from_slice(format!("{:x}", hasher.finish()).as_bytes());
        NumberHash(res)
    }
}

#[test]
fn test_mmr() {
    let store = MemStore::default();
    let mut mmr = MMR::<_, MergeNumberHash, _>::new(0, &store);
    HEADERS.iter().for_each(|h| {
        mmr.push(NumberHash(h.clone())).unwrap();
    });
}
