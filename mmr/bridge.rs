//! format bridge
#![macro_use]
use scale::{Decode, Encode};

macro_rules! bytes {
    // Convert hex to Vec<u8>
    ($hex:expr) => {{
        (0..$hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&$hex[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .unwrap_or_default()
    }};

    // Convert hex to [u8; $bits]
    ($hex:expr, $bits:expr) => {{
        let mut hash = [0_u8; $bits];
        hash.copy_from_slice(&bytes!($hex));
        hash
    }};
}

macro_rules! u256 {
    // Convert hex to [u64; 4]
    ($hex:expr) => {{
        let mut u256 = [0_u64; 4];
        let bytes = (0..$hex.len())
            .step_by(2)
            .map(|i| u64::from_str_radix(&$hex[i..i + 2], 64))
            .collect::<Result<Vec<u64>, _>>()
            .unwrap_or_default();
        u256.copy_from_slice(&bytes);
        u256
    }};
}

/// Eth header
#[derive(Decode, Encode)]
pub struct EthHeader {
    parent_hash: [u8; 32],
    timestamp: u64,
    number: u64,
    transactions_root: [u8; 32],
    uncles_hash: [u8; 32],
    extra_data: Vec<u8>,
    state_root: [u8; 32],
    receipts_root: [u8; 32],
    log_bloom: [u8; 256],
    gas_used: [u64; 4],
    gas_limit: [u64; 4],
    difficulty: [u64; 4],
    seal: Vec<Vec<u8>>,
    hash: [u8; 32],
}

impl EthHeader {
    /// New EthHeader from string array
    pub fn from_go_ffi(
        parent_hash: String,
        timestamp: u64,
        number: u64,
        transactions_root: String,
        uncles_hash: String,
        extra_data: String,
        state_root: String,
        receipts_root: String,
        log_bloom: String,
        gas_used: String,
        gas_limit: String,
        difficulty: String,
        mixh: String,
        nonce: String,
        hash: String,
    ) -> EthHeader {
        EthHeader {
            parent_hash: bytes!(parent_hash, 32),
            timestamp,
            number,
            transactions_root: bytes!(transactions_root, 32),
            uncles_hash: bytes!(uncles_hash, 32),
            extra_data: bytes!(extra_data),
            state_root: bytes!(state_root, 32),
            receipts_root: bytes!(receipts_root, 32),
            log_bloom: bytes!(log_bloom, 256),
            gas_used: u256!(gas_used),
            gas_limit: u256!(gas_limit),
            difficulty: u256!(difficulty),
            seal: vec![bytes!(mixh), bytes!(nonce)],
            hash: bytes!(hash, 32),
        }
    }
}

// pub struct DoubleNodeWithMerkleProof {
//     pub dag_nodes: [[u8; 64]; 2],
//     pub proof: Vec<[u8; 16]>,
// }
