use crate::{
    chain::array::{H1024, U256},
    hex,
    result::Error,
};
use reqwest::Client;
use scale::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fmt::Debug, str::FromStr};

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
    pub async fn get(client: &Client, block: u64) -> Result<EthHeaderRPCResp, Error> {
        let map: Value = serde_json::from_str(&format! {
            "{{{}}}", vec![
                r#""jsonrpc":"2.0","#,
                r#""method":"eth_getBlockByNumber","#,
                &format!(r#""params":["{:#X}", false],"#, block),
                r#""id": 1"#,
            ].concat(),
        })?;

        Ok(client
            .post(&env::var("ETHEREUM_RPC").unwrap_or_else(|_| {
                if env::var("ETHEREUM_ROPSTEN").is_ok() {
                    crate::conf::DEFAULT_ETHEREUM_ROPSTEN_RPC.into()
                } else {
                    crate::conf::DEFAULT_ETHEREUM_RPC.into()
                }
            }))
            .json(&map)
            .send()
            .await?
            .json()
            .await?)
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
        let seal: Vec<Vec<u8>> = vec![
            rlp::encode(&bytes!(self.mix_hash.as_str())),
            rlp::encode(&bytes!(self.nonce.as_str())),
        ];
        EthHeader {
            parent_hash: bytes!(self.parent_hash.as_str(), 32),
            timestamp: u64::from_str_radix(&self.timestamp.as_str()[2..], 16).unwrap_or_default(),
            number: u64::from_str_radix(&self.number.as_str()[2..], 16).unwrap_or_default(),
            author: bytes!(self.miner.as_str(), 20),
            transactions_root: bytes!(self.transactions_root.as_str(), 32),
            uncles_hash: bytes!(self.sha3_uncles.as_str(), 32),
            extra_data: bytes!(self.extra_data.as_str()),
            state_root: bytes!(self.state_root.as_str(), 32),
            receipts_root: bytes!(self.receipts_root.as_str(), 32),
            log_bloom: H1024(bytes!(self.logs_bloom.as_str(), 256)),
            gas_used: U256::from_str(&self.gas_used[2..]).unwrap_or_default(),
            gas_limit: U256::from_str(&self.gas_limit[2..]).unwrap_or_default(),
            difficulty: U256::from_str(&self.difficulty[2..]).unwrap_or_default(),
            seal: seal,
            hash: match self.hash.is_empty() {
                true => None,
                false => Some(bytes!(self.hash.as_str(), 32)),
            },
        }
    }
}

/// Darwinia Eth header
#[derive(Decode, Encode, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
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
    /// Get header
    pub async fn get(client: &Client, block: u64) -> Result<EthHeader, Error> {
        Ok(EthHeaderRPCResp::get(client, block).await?.result.into())
    }
}

/// Darwinia Eth header Json foramt
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EthHeaderJson {
    parent_hash: String,
    timestamp: u64,
    number: u64,
    author: String,
    transactions_root: String,
    uncles_hash: String,
    extra_data: String,
    state_root: String,
    receipts_root: String,
    log_bloom: String,
    gas_used: u128,
    gas_limit: u128,
    difficulty: u128,
    seal: Vec<String>,
    hash: String,
}

impl From<EthHeader> for EthHeaderJson {
    fn from(e: EthHeader) -> EthHeaderJson {
        EthHeaderJson {
            parent_hash: format!("0x{}", hex!(e.parent_hash.to_vec())),
            timestamp: e.timestamp,
            number: e.number,
            author: format!("0x{}", hex!(e.author.to_vec())),
            transactions_root: format!("0x{}", hex!(e.transactions_root.to_vec())),
            uncles_hash: format!("0x{}", hex!(e.uncles_hash.to_vec())),
            extra_data: format!("0x{}", hex!(e.extra_data.to_vec())),
            state_root: format!("0x{}", hex!(e.state_root.to_vec())),
            receipts_root: format!("0x{}", hex!(e.receipts_root.to_vec())),
            log_bloom: format!("0x{}", hex!(e.log_bloom.0.to_vec())),
            gas_used: e.gas_used.as_u128(),
            gas_limit: e.gas_limit.as_u128(),
            difficulty: e.difficulty.as_u128(),
            seal: e
                .seal
                .iter()
                .map(|s| format!("0x{}", hex!(s.to_vec())))
                .collect(),
            hash: format!("0x{}", hex!(e.hash.unwrap_or_default().to_vec())),
        }
    }
}
