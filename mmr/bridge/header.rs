use super::{Bloom, U256};
use scale::{Decode, Encode};
use std::fmt::Debug;

/// Darwinia Eth header
#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct EthHeader {
    parent_hash: [u8; 32],
    timestamp: u64,
    number: u64,
    author: [u8; 20],
    transactions_root: [u8; 32],
    uncles_hash: [u8; 32],
    extra_data: Vec<u8>,
    state_root: [u8; 32],
    receipts_root: [u8; 32],
    log_bloom: Bloom,
    gas_used: U256,
    gas_limit: U256,
    difficulty: U256,
    seal: Vec<Vec<u8>>,
    hash: Option<[u8; 32]>,
}

impl EthHeader {
    /// New EthHeader from string array
    pub fn from_go_ffi(
        parent_hash: &str,
        timestamp: u64,
        number: u64,
        author: &str,
        transactions_root: &str,
        uncles_hash: &str,
        extra_data: &str,
        state_root: &str,
        receipts_root: &str,
        log_bloom: &str,
        gas_used: &str,
        gas_limit: &str,
        difficulty: &str,
        mixh: &str,
        nonce: &str,
        hash: &str,
    ) -> EthHeader {
        EthHeader {
            parent_hash: bytes!(parent_hash, 32),
            timestamp,
            number,
            author: bytes!(author, 20),
            transactions_root: bytes!(transactions_root, 32),
            uncles_hash: bytes!(uncles_hash, 32),
            extra_data: bytes!(extra_data),
            state_root: bytes!(state_root, 32),
            receipts_root: bytes!(receipts_root, 32),
            log_bloom: Bloom(bytes!(log_bloom, 256)),
            gas_used: U256::from_dec_str(gas_used).unwrap_or_default(),
            gas_limit: U256::from_dec_str(gas_limit).unwrap_or_default(),
            difficulty: U256::from_dec_str(difficulty).unwrap_or_default(),
            seal: match mixh.is_empty() && nonce.is_empty() {
                true => vec![],
                false => vec![bytes!(mixh), bytes!(nonce)],
            },
            hash: match hash.is_empty() {
                true => None,
                false => Some(bytes!(hash, 32)),
            },
        }
    }
}

impl Default for EthHeader {
    fn default() -> EthHeader {
        EthHeader::from_go_ffi(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            0,
            0,
            "0x0000000000000000000000000000000000000000",
            "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "",
            "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "0",
            "0",
            "0",
            "",
            "",
            ""
        )
    }
}
