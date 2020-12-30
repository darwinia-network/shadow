use mmr::{
    H256,
    mmr_size_to_last_leaf,
    build_client,
    Database,
    MmrClientTrait,
    MergeHash
};
use std::sync::Arc;
use rocksdb::{Options, DB};
use std::{env, path::PathBuf};

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


#[test]
fn test_hex() {
    &HEADERS_N_ROOTS.iter().for_each(|h| {
        assert_eq!(H256::hex(&H256::from(h.0).unwrap()), String::from(h.0));
    });
}

#[test]
fn test_mmr_size_to_last_leaf() {
    let leaf = mmr_size_to_last_leaf(0);
    assert_eq!(leaf, 0);
    let leaf = mmr_size_to_last_leaf(1);
    assert_eq!(leaf, 0);
    let leaf = mmr_size_to_last_leaf(3);
    assert_eq!(leaf, 1);
    let leaf = mmr_size_to_last_leaf(4);
    assert_eq!(leaf, 2);
    let leaf = mmr_size_to_last_leaf(7);
    assert_eq!(leaf, 3);
    let leaf = mmr_size_to_last_leaf(8);
    assert_eq!(leaf, 4);
    let leaf = mmr_size_to_last_leaf(10);
    assert_eq!(leaf, 5);
    let leaf = mmr_size_to_last_leaf(11);
    assert_eq!(leaf, 6);
    let leaf = mmr_size_to_last_leaf(15);
    assert_eq!(leaf, 7);
}

#[test]
fn test_mmr_size_n_leaves() {
    (0..1000).for_each(|i| {
        assert_eq!(
            i as i64,
            mmr_size_to_last_leaf(cmmr::leaf_index_to_mmr_size(i) as i64)
        );
    });
}

fn rocks_test_client(file: &str) -> (Box<dyn MmrClientTrait>, PathBuf) {
    use std::fs;
    let dbpath = env::temp_dir().join(file);
    fs::remove_dir_all(&dbpath).unwrap_or_else(|err|{
        println!("{}", err);
    });
    let rocksdb = DB::open_default(&dbpath).unwrap();
    let db = Arc::new(rocksdb);
    let clientdb = Database::Rocksdb(db.clone());
    return (build_client(&clientdb).unwrap(), dbpath);
}

#[test]
fn test_rocks_client_base() {
    let db = {
        let (mut client, db) = rocks_test_client("test_rocks_client.db");
        let pos: Vec<u64> = (0..10).map(|h| {
            client.push(&H256::from(HEADERS_N_ROOTS[h].0).unwrap()).unwrap()
        }).collect();

        let mmr_size = client.get_mmr_size().unwrap();
        assert_eq!(mmr_size, 18);
        let last_leaf_index = client.get_last_leaf_index().unwrap().unwrap();
        assert_eq!(last_leaf_index, 9);
        (0..10).for_each(|l| {
            assert_eq!(
                client.get_leaf(l).unwrap().unwrap(),
                HEADERS_N_ROOTS[l as usize].0
                );
            assert_eq!(
                client.get_elem(pos[l as usize]).unwrap().unwrap(),
                HEADERS_N_ROOTS[l as usize].0
                );
            let mmr_root = client.get_mmr_root(l).unwrap().unwrap();
            assert_eq!(mmr_root, HEADERS_N_ROOTS[l as usize].1);
        });
        db
    };
    assert!(DB::destroy(&Options::default(), &db).is_ok());
}

#[test]
fn test_rocks_client_proof() {
    let db = {
        let (mut client, db) = rocks_test_client("test_rocks_proof.db");
        let pos: Vec<u64> = (0..10).map(|h| {
            client.push(&H256::from(HEADERS_N_ROOTS[h].0).unwrap()).unwrap()
        }).collect();

        let last_leaf_index = client.get_last_leaf_index().unwrap().unwrap();
        let mmr_root = client.get_mmr_root(last_leaf_index).unwrap().unwrap();
        (0..10).for_each(|l| {
            let proof = client.gen_proof(l, last_leaf_index).unwrap();
            let mmr_size = client.get_mmr_size().unwrap();
            let merkle_proof: cmmr::MerkleProof<[u8;32], MergeHash> = cmmr::MerkleProof::new(mmr_size, proof.iter().map(|h| {
                H256::from(h).unwrap()
            }).collect::<Vec<[u8; 32]>>());
            let verify = merkle_proof.verify(H256::from(&mmr_root).unwrap(), vec![(pos[l as usize], H256::from(&HEADERS_N_ROOTS[l as usize].0).unwrap())]).unwrap();
            assert_eq!(verify, true);
            let verify = merkle_proof.verify(H256::from(&mmr_root).unwrap(), vec![(pos[l as usize], H256::from(&HEADERS_N_ROOTS[((l + 1) % 10) as usize].0).unwrap())]).unwrap();
            assert_eq!(verify, false);
        });
        db
    };
    assert!(DB::destroy(&Options::default(), &db).is_ok());
}

