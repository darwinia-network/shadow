//! scale codec interfaces

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
