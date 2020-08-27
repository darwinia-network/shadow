use cmmr::{Merge, MMR};
use darwinia_shadow::{
    hash::{MergeHash, H256},
    helper, pool,
    store::Store,
};
use std::{env, fs, path::PathBuf};

const HEADERS_N_ROOTS: [(&str, &str); 10] = [
    (
        "34f61bfda344b3fad3c3e38832a91448b3c613b199eb23e5110a635d71c13c65",
        "34f61bfda344b3fad3c3e38832a91448b3c613b199eb23e5110a635d71c13c65",
    ),
    (
        "70d641860d40937920de1eae29530cdc956be830f145128ebb2b496f151c1afb",
        "3aafcc7fe12cb8fad62c261458f1c19dba0a3756647fa4e8bff6e248883938be",
    ),
    (
        "12e69454d992b9b1e00ea79a7fa1227c889c84d04b7cd47e37938d6f69ece45d",
        "7ddf10d67045173e3a59efafb304495d9a7c84b84f0bc0235470a5345e32535d",
    ),
    (
        "3733bd06905e128d38b9b336207f301133ba1d0a4be8eaaff6810941f0ad3b1a",
        "488e9565547fec8bd36911dc805a7ed9f3d8d1eacabe429c67c6456933c8e0a6",
    ),
    (
        "3d7572be1599b488862a1b35051c3ef081ba334d1686f9957dbc2afd52bd2028",
        "6e0c4ab56e0919a7d45867fcd1216e2891e06994699eb838386189e9abda55f1",
    ),
    (
        "2a04add3ecc3979741afad967dfedf807e07b136e05f9c670a274334d74892cf",
        "293b49420345b185a1180e165c76f76d8cc28fe46c1f6eb4a96959253b571ccd",
    ),
    (
        "c58e247ea35c51586de2ea40ac6daf90eac7ac7b2f5c88bbc7829280db7890f1",
        "2dee5b87a481a9105cb4b2db212a1d8031d65e9e6e68dc5859bef5e0fdd934b2",
    ),
    (
        "2cf0262f0a8b00cad22afa04d70fb0c1dbb2eb4a783beb7c5e27bd89015ff573",
        "54be644b5b3291dd9ae9598b49d1f986e4ebd8171d5e89561b2a921764c7b17c",
    ),
    (
        "05370d06def89f11486c994c459721b4bd023ff8c2347f3187e9f42ef39bddab",
        "620dbc3a28888da8b17ebf5b18dba53794621463e2bbabcf88b8cbc97508ab38",
    ),
    (
        "c0c8c3f7dc9cdfa87d2433bcd72a744d634524a5ff76e019e44ea450476bac99",
        "a94bf2a4e0437c236c68675403d980697cf7c9b0f818a622cb40199db5e12cf8",
    ),
];

fn gen_mmr<F>(db: &PathBuf, f: F)
where
    F: Fn(MMR<[u8; 32], MergeHash, &Store>, Vec<u64>),
{
    let db = env::temp_dir().join(db);
    let conn = pool::conn(Some(db));
    let store = Store::with(conn);
    let mut mmr = MMR::<_, MergeHash, _>::new(0, &store);
    let pos: Vec<u64> = (0..10)
        .map(|h| {
            mmr.push(<[u8; 32] as H256>::from(HEADERS_N_ROOTS[h].0))
                .unwrap()
        })
        .collect();

    f(mmr, pos);
}

#[test]
fn test_hex() {
    &HEADERS_N_ROOTS.iter().for_each(|h| {
        assert_eq!(<[u8; 32] as H256>::from(h.0).hex(), String::from(h.0));
    });
}

#[test]
fn test_mmr_proof() {
    let db = env::temp_dir().join("test_mmr_proof.db");
    gen_mmr(&db, |mmr, pos| {
        let root = mmr.get_root().expect("get root failed");
        let proof = mmr
            .gen_proof((0..10).map(|e| pos[e]).collect())
            .expect("gen proof");

        let result = proof
            .verify(
                root,
                (0..10)
                    .map(|e| (pos[e], <[u8; 32] as H256>::from(HEADERS_N_ROOTS[e].0)))
                    .collect(),
            )
            .unwrap();

        assert!(result);
        assert!(fs::remove_file(&db).is_ok());
    });
}

#[test]
fn test_mmr_size_n_leaves() {
    (0..1000).for_each(|i| {
        assert_eq!(
            i as i64,
            helper::mmr_size_to_last_leaf(cmmr::leaf_index_to_mmr_size(i) as i64)
        );
    });
}

#[test]
fn test_mmr_merge() {
    let lhs: [u8; 32] = <[u8; 32] as H256>::from(HEADERS_N_ROOTS[0].0);
    let rhs: [u8; 32] = <[u8; 32] as H256>::from(HEADERS_N_ROOTS[1].0);
    assert_eq!(
        "3aafcc7fe12cb8fad62c261458f1c19dba0a3756647fa4e8bff6e248883938be",
        H256::hex(&MergeHash::merge(&lhs, &rhs))
    );
}

// TODO: Unit test for this, need a specific module to test this.
//
// ------------
//
// #[test]
// fn test_default_runner() {
//     const LAST_LEAF_INDEX: u64 = 35;
//     println!(
//         "last leaf pos: {:?}",
//         cmmr::leaf_index_to_pos(LAST_LEAF_INDEX)
//     );
//
//     let store = Store::default();
//     let mmr = MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(LAST_LEAF_INDEX), store);
//
//     let mut index: u64 = 0;
//     while index < 35 {
//         let proof = mmr.gen_proof(vec![cmmr::leaf_index_to_pos(index)]);
//         if proof.is_err() {
//             index += 1;
//         } else {
//             println!(
//                 "index is: {:?}, pos is: {:?}",
//                 index,
//                 cmmr::leaf_index_to_pos(index)
//             );
//             println!(
//                 "{:?}",
//                 proof
//                     .unwrap()
//                     .proof_items()
//                     .iter()
//                     .map(|item| H256::hex(item))
//                     .collect::<Vec<String>>()
//                     .join(",")
//             );
//             index += 1;
//         }
//     }
// }
