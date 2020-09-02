//! Mock the uncle block
use cmmr::{leaf_index_to_mmr_size, leaf_index_to_pos, MMR};
use darwinia_shadow::{
    db::pool,
    mmr::{MergeHash, Store, H256},
};

fn main() {
    let conn = pool::conn(None);
    let store = Store::with(conn);

    let mmr = MMR::<_, MergeHash, _>::new(leaf_index_to_mmr_size(9), &store);
    let root = mmr.get_root().expect("get root failed");
    // let root = H256::from("0xe28d7f650efb9cbaaca7f485d078c0f6b1104807a3a31f85fc1268b0673140ff");
    let proof = mmr
        // .gen_proof((0..10).map(|i| leaf_index_to_pos(i)).collect())
        .gen_proof(vec![leaf_index_to_pos(1)])
        .unwrap();

    // let p = MerkleProof::<[u8; 32], MergeHash>::new(
    //     cmmr::leaf_index_to_mmr_size(9),
    //     vec![
    //         "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3",
    //         "0xcea81d50343714a15b240c330d617f0259526bfd55e41a074cdb6eeb5f2fc97b",
    //         "0x843749778a9d9df1a7d2751c7bb4d5a38cdc01ad3c2e5b154715474386c8df48",
    //         "0x8e57ae1d44d62821b07a46c0e20d1ea5d1d3ef0200623a946dc82c4373f971df",
    //     ]
    //     .into_iter()
    //     .map(|h| H256::from(&h))
    //     .collect(),
    // );

    println!(
        "{:?}",
        proof.verify(
            root,
            vec![(
                leaf_index_to_pos(1),
                H256::from("0x88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6")
            )]
        )
    );

    // let proofs = proof
    //     .proof_items()
    //     .iter()
    //     .map(|item| H256::hex(item))
    //     .collect::<Vec<String>>();
    //
    // println!("leaf root: 0x{}", H256::hex(&root));
    // println!("{:?}", proofs);
}
