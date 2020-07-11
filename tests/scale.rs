use eth::header::EthHeader as DEthHeader;
use mmr::{bridge::EthHeader, hash::H256, hex};
use scale::{Decode, Encode};

const HASHES: [&str; 10] = [
    "34f61bfda344b3fad3c3e38832a91448b3c613b199eb23e5110a635d71c13c65",
    "70d641860d40937920de1eae29530cdc956be830f145128ebb2b496f151c1afb",
    "12e69454d992b9b1e00ea79a7fa1227c889c84d04b7cd47e37938d6f69ece45d",
    "3733bd06905e128d38b9b336207f301133ba1d0a4be8eaaff6810941f0ad3b1a",
    "3d7572be1599b488862a1b35051c3ef081ba334d1686f9957dbc2afd52bd2028",
    "2a04add3ecc3979741afad967dfedf807e07b136e05f9c670a274334d74892cf",
    "c58e247ea35c51586de2ea40ac6daf90eac7ac7b2f5c88bbc7829280db7890f1",
    "2cf0262f0a8b00cad22afa04d70fb0c1dbb2eb4a783beb7c5e27bd89015ff573",
    "05370d06def89f11486c994c459721b4bd023ff8c2347f3187e9f42ef39bddab",
    "c0c8c3f7dc9cdfa87d2433bcd72a744d634524a5ff76e019e44ea450476bac99",
];

/// Hash array for tests
fn ha() -> [[u8; 32]; 10] {
    let mut hashes = [[0; 32]; 10];
    (0..10).for_each(|i| hashes[i] = <[u8; 32] as H256>::from(HASHES[i]));
    hashes
}

/// the scale codec of hash is its hex string
#[test]
fn hash() {
    let hashes = ha();
    (0..10).for_each(|i| {
        assert_eq!(hashes[i].encode(), hashes[i]);
    });
}

/// the scale codec of hash array is its concatention
#[test]
fn hash_array() {
    let hashes = ha();
    let encoded = hashes.encode();
    assert_eq!(encoded, hashes.concat());
}

#[test]
fn mmr_proof() {
    let hashes = ha();
    assert_eq!(
        format!("08{}", &hex!(&hashes[0..2].concat())),
        hex!(hashes[0..2].to_vec().encode())
    );
}

#[test]
fn eth_header() {
    assert_eq!(
        EthHeader::default().encode(),
        DEthHeader::default().encode()
    );
}
