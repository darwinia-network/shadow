mod ethash_proof;
mod ethash_proof_codec;
mod hash;
mod header;
mod header_thing;
mod mock_header_19;

pub use ethash_proof::{DAG_NODES, PROOF};
pub use ethash_proof_codec::ETHASH_PROOF_CODEC;
pub use hash::HASHES;
pub use header::HEADER;
pub use header_thing::ETH_HEADER_THING;
pub use mock_header_19::MOCK_HEADER_19;

use mmr::{
    chain::eth::{DoubleNodeWithMerkleProof, EthHeader},
    hash::H256,
};

/// Hash array for tests
pub fn ha() -> [[u8; 32]; 10] {
    let mut hashes = [[0; 32]; 10];
    (0..10).for_each(|i| hashes[i] = <[u8; 32] as H256>::from(HASHES[i]));
    hashes
}

/// Block Number ffi ports
pub fn header() -> EthHeader {
    EthHeader::from_go_ffi(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            0,
            0,
            "0x0000000000000000000000000000000000000000",
            "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "0x11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa",
            "0xd7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544",
            "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "0",
            "5000",
            "17179869184",
            "0xa00000000000000000000000000000000000000000000000000000000000000000",
            "0x880000000000000042",
            "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3",
        )
}

/// Generate DoubleNodeWithMerkleProof
pub fn proof() -> DoubleNodeWithMerkleProof {
    DoubleNodeWithMerkleProof::from_tuple(DAG_NODES, PROOF)
}
