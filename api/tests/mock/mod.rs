mod ethash_proof;
mod ethash_proof_codec;
mod header;

pub use ethash_proof::{DAG_NODES, PROOF};
pub use ethash_proof_codec::ETHASH_PROOF_CODEC;
pub use header::HEADER;
// pub use header_thing::ETH_HEADER_THING;
// pub use mock_header_19::MOCK_HEADER_19;
use codec::Decode;

use mmr::H256;
use shadow_types::{
    bytes,
    chain::ethereum::{EthashProof, EthereumHeader},
};

pub fn ha() -> [[u8; 32]; 10] {
    return [
        bytes!(
            "0x34f61bfda344b3fad3c3e38832a91448b3c613b199eb23e5110a635d71c13c65",
            32
        ),
        bytes!(
            "0x70d641860d40937920de1eae29530cdc956be830f145128ebb2b496f151c1afb",
            32
        ),
        bytes!(
            "0x12e69454d992b9b1e00ea79a7fa1227c889c84d04b7cd47e37938d6f69ece45d",
            32
        ),
        bytes!(
            "0x3733bd06905e128d38b9b336207f301133ba1d0a4be8eaaff6810941f0ad3b1a",
            32
        ),
        bytes!(
            "0x3d7572be1599b488862a1b35051c3ef081ba334d1686f9957dbc2afd52bd2028",
            32
        ),
        bytes!(
            "0x2a04add3ecc3979741afad967dfedf807e07b136e05f9c670a274334d74892cf",
            32
        ),
        bytes!(
            "0xc58e247ea35c51586de2ea40ac6daf90eac7ac7b2f5c88bbc7829280db7890f1",
            32
        ),
        bytes!(
            "0x2cf0262f0a8b00cad22afa04d70fb0c1dbb2eb4a783beb7c5e27bd89015ff573",
            32
        ),
        bytes!(
            "0x05370d06def89f11486c994c459721b4bd023ff8c2347f3187e9f42ef39bddab",
            32
        ),
        bytes!(
            "0xc0c8c3f7dc9cdfa87d2433bcd72a744d634524a5ff76e019e44ea450476bac99",
            32
        ),
    ];
}

/// Block Number ffi ports
pub fn header() -> EthereumHeader {
    let bytes = array_bytes::hex2bytes_unchecked(HEADER);
    EthereumHeader::decode(&mut bytes.as_ref()).unwrap()
}

/// Generate DoubleNodeWithMerkleProof
pub fn proof() -> EthashProof {
    EthashProof::from_tuple(DAG_NODES, PROOF)
}
