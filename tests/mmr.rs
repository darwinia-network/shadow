use cmmr::{util::MemStore, MMR};
use mmr::{ETHash, MergeETHash};

const HEADERS: [&str; 10] = [
    "d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3",
    "88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6",
    "b495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9",
    "3d6122660cc824376f11ee842f83addc3525e2dd6756b9bcf0affa6aa88cf741",
    "23adf5a3be0f5235b36941bcb29b62504278ec5b9cdfa277b992ba4a7a3cd3a2",
    "f37c632d361e0a93f08ba29b1a2c708d9caa3ee19d1ee8d2a02612bffe49f0a9",
    "1f1aed8e3694a067496c248e61879cda99b0709a1dfbacd0b693750df06b326e",
    "e0c7c0b46e116b874354dce6f64b8581bd239186b03f30a978e3dc38656f723a",
    "2ce94342df186bab4165c268c43ab982d360c9474f429fec5565adfc5d1f258b",
    "997e47bf4cac509c627753c06385ac866641ec6f883734ff7944411000dc576e",
];

#[test]
fn test_mmr() {
    let store = MemStore::default();
    let mut mmr = MMR::<_, MergeETHash, _>::new(0, &store);
    let pos: Vec<u64> = (0usize..10usize)
        .map(|h| mmr.push(ETHash::from(HEADERS[h])).unwrap())
        .collect();

    let root = mmr.get_root().expect("get root failed");
    let proof = mmr
        .gen_proof((0..10).map(|e| pos[e]).collect())
        .expect("gen proof");

    mmr.commit().expect("commit changes");
    let result = proof
        .verify(
            root,
            (0..10)
                .map(|e| (pos[e], ETHash::from(HEADERS[e])))
                .collect(),
        )
        .unwrap();
    assert!(result);
}
