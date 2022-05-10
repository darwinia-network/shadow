mod ethash_proof;
mod ethash_proof_codec;
mod hash;
mod header;
// mod header_thing;
// mod mock_header_19;

pub use ethash_proof::{DAG_NODES, PROOF};
pub use ethash_proof_codec::ETHASH_PROOF_CODEC;
pub use hash::HASHES;
pub use header::HEADER;
// pub use header_thing::ETH_HEADER_THING;
// pub use mock_header_19::MOCK_HEADER_19;
use codec::Decode;

use mmr::H256;
use shadow_types::{
    bytes,
    chain::ethereum::{EthashProof, EthereumHeader},
};

/// Hash array for tests
pub fn ha() -> [[u8; 32]; 10] {
    let mut hashes = [[0; 32]; 10];
    (0..10).for_each(|i| hashes[i] = H256::from(HASHES[i]).unwrap());
    hashes
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
