//! Mock the uncle block
use cmmr::MMR;
use darwinia_shadow::{
    hash::{MergeHash, H256},
    pool,
    store::Store,
};
use std::env;

/// Blocs 0 ~ 19
const HASHES: [&str; 20] = [
    "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3",
    "0x88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6",
    "0xb495a1d7e6663152ae92708da4843337b958146015a2802f4193a410044698c9",
    "0x3d6122660cc824376f11ee842f83addc3525e2dd6756b9bcf0affa6aa88cf741",
    "0x23adf5a3be0f5235b36941bcb29b62504278ec5b9cdfa277b992ba4a7a3cd3a2",
    "0xf37c632d361e0a93f08ba29b1a2c708d9caa3ee19d1ee8d2a02612bffe49f0a9",
    "0x1f1aed8e3694a067496c248e61879cda99b0709a1dfbacd0b693750df06b326e",
    "0xe0c7c0b46e116b874354dce6f64b8581bd239186b03f30a978e3dc38656f723a",
    "0x2ce94342df186bab4165c268c43ab982d360c9474f429fec5565adfc5d1f258b",
    "0x997e47bf4cac509c627753c06385ac866641ec6f883734ff7944411000dc576e",
    "0x4ff4a38b278ab49f7739d3a4ed4e12714386a9fdf72192f2e8f7da7822f10b4d",
    "0x3f5e756c3efcb93099361b7ddd0dabfeaa592439437c1c836e443ccb81e93242",
    "0xc63f666315fa1eae17e354fab532aeeecf549be93e358737d0648f50d57083a0",
    "0x55b6a7e73c57d1ca35b35cad22869eaa33e10fa2a822fb7308f419269794d611",
    "0x46015afbe00cf61ff284c26cc09a776a7303e422c7b359fe4317b4e6aaa410a4",
    "0x2d33dc73755afbbbeb6ec4885f2923398901bf1ad94beb325a4c4ecad5bf0f1c",
    "0x9657beaf8542273d7448f6d277bb61aef0f700a91b238ac8b34c020f7fb8664c",
    "0xf25fe829ebbf3e2459ecb89cbc1aaa5f83c04501df08d63fa8dd1589f6b1cae0",
    "0x480ff3f8a495b764e4361a6c2e296f34e8721cf1ec54fe5c46827937353bf118",
    // "0xec888de9fa46cb7a47b7bd812a2f601d948d89e5317cf9f68976a0dec92b1ee2",
    // the block below is uncle block 21(19)
    "0xb31db2ee05835be4fd025ae16eecaa55b670c1b67a009969a7912bea39f9951b",
];

fn main() {
    let db = env::temp_dir().join("test_mmr_proof.db");
    let conn = pool::conn(Some(db));
    let store = Store::with(conn);
    let mut mmr = MMR::<_, MergeHash, _>::new(0, &store);
    let mut roots: Vec<String> = vec![];
    let pos: Vec<u64> = (0..20)
        .map(|h| {
            let pos = mmr.push(<[u8; 32] as H256>::from(HASHES[h])).unwrap();
            roots.push(format!(
                "0x{}",
                mmr.get_root().expect("get root failed").hex()
            ));
            pos
        })
        .collect();

    println!("{:?}\n", roots);
    (0..20).for_each(|i| {
        println!(
            "[]string{{{:?}}},",
            mmr.gen_proof(vec![pos[i]])
                .unwrap()
                .proof_items()
                .iter()
                .map(|item| H256::hex(item))
                .collect::<Vec<String>>()
        );
    });
    // let proofs = mmr
    //     .gen_proof(vec![pos[19]])
    //     .unwrap()
    //     .proof_items()
    //     .iter()
    //     .map(|item| H256::hex(item))
    //     .collect::<Vec<String>>();
    //
    // println!("{:#?}, {:#?}, {:#?}", HASHES, roots, proofs);
}
