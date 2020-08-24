use crate::{
    chain::array::{H1024, U256},
    result::Error,
};
use reqwest::blocking::Client;
use scale::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fmt::Debug};

/// Ethereum JSON rpc response
#[derive(Serialize, Deserialize, Debug)]
pub struct EthHeaderRPCResp {
    jsonrpc: String,
    id: i32,
    /// Header Result of RPC
    pub result: RawEthHeader,
}

impl EthHeaderRPCResp {
    /// Get `EthHeader` by number
    pub fn get(client: &Client, block: u64) -> Result<EthHeaderRPCResp, Error> {
        let api = env::var("ETHEREUM_RPC").unwrap_or(crate::conf::DEFAULT_ETHEREUM_RPC.into());
        let map: Value = serde_json::from_str(&format! {
            "{{{}}}", vec![
                r#""jsonrpc":"2.0","#,
                r#""method":"eth_getBlockByNumber","#,
                &format!(r#""params":["{:#X}", false],"#, block),
                r#""id": 1"#,
            ].concat(),
        })?;

        Ok(client.post(&api).json(&map).send()?.json()?)
    }
}

/// Raw EthHeader from Ethereum rpc
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawEthHeader {
    difficulty: String,
    extra_data: String,
    gas_limit: String,
    gas_used: String,
    /// Ethereum header hash
    pub hash: String,
    logs_bloom: String,
    miner: String,
    mix_hash: String,
    nonce: String,
    number: String,
    parent_hash: String,
    receipts_root: String,
    sha3_uncles: String,
    size: String,
    state_root: String,
    timestamp: String,
    total_difficulty: String,
    transactions: Vec<String>,
    transactions_root: String,
    uncles: Vec<String>,
}

impl Into<EthHeader> for RawEthHeader {
    fn into(self) -> EthHeader {
        EthHeader {
            parent_hash: bytes!(self.parent_hash.as_str(), 32),
            timestamp: u64::from_str_radix(&self.timestamp.as_str(), 16).unwrap(),
            number: u64::from_str_radix(&self.number.as_str(), 16).unwrap(),
            author: bytes!(self.miner.as_str(), 20),
            transactions_root: bytes!(self.transactions_root.as_str(), 32),
            uncles_hash: bytes!(self.sha3_uncles.as_str(), 32),
            extra_data: bytes!(self.extra_data.as_str()),
            state_root: bytes!(self.state_root.as_str(), 32),
            receipts_root: bytes!(self.receipts_root.as_str(), 32),
            log_bloom: H1024(bytes!(self.logs_bloom.as_str(), 256)),
            gas_used: U256::from_dec_str(&self.gas_used.as_str()).unwrap_or_default(),
            gas_limit: U256::from_dec_str(&self.gas_limit.as_str()).unwrap_or_default(),
            difficulty: U256::from_dec_str(&self.difficulty.as_str()).unwrap_or_default(),
            seal: match self.mix_hash.is_empty() && self.nonce.is_empty() {
                true => vec![],
                false => vec![bytes!(self.mix_hash.as_str()), bytes!(self.nonce.as_str())],
            },
            hash: match self.hash.is_empty() {
                true => None,
                false => Some(bytes!(self.hash.as_str(), 32)),
            },
        }
    }
}

/// Darwinia Eth header
#[derive(Decode, Encode, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    log_bloom: H1024,
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
            log_bloom: H1024(bytes!(log_bloom, 256)),
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
