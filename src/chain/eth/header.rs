use crate::{
    chain::array::{H1024, U256},
    result::Error,
};
use reqwest::{blocking::Client, Client as AsyncClient};
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
    /// The get block api
    pub fn get_block_api(block: u64) -> Result<Value, Error> {
        Ok(serde_json::from_str(&format! {
            "{{{}}}", vec![
                r#""jsonrpc":"2.0","#,
                r#""method":"eth_getBlockByNumber","#,
                &format!(r#""params":["{:#X}", false],"#, block),
                r#""id": 1"#,
            ].concat(),
        })?)
    }

    /// Get `EthHeader` by number
    pub fn get(client: &Client, block: u64) -> Result<EthHeaderRPCResp, Error> {
        Ok(client
            .post(&env::var("ETHEREUM_RPC").unwrap_or(crate::conf::DEFAULT_ETHEREUM_RPC.into()))
            .json(&EthHeaderRPCResp::get_block_api(block)?)
            .send()?
            .json()?)
    }

    /// Async get block
    pub async fn async_get(client: &AsyncClient, block: u64) -> Result<EthHeaderRPCResp, Error> {
        Ok(client
            .post(&env::var("ETHEREUM_RPC").unwrap_or(crate::conf::DEFAULT_ETHEREUM_RPC.into()))
            .json(&EthHeaderRPCResp::get_block_api(block)?)
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
    pub fn get(client: &Client, block: u64) -> Result<EthHeader, Error> {
        Ok(EthHeaderRPCResp::get(client, block)?.result.into())
    }

    /// Async Get header
    pub async fn async_get(client: &AsyncClient, block: u64) -> Result<EthHeader, Error> {
        Ok(EthHeaderRPCResp::async_get(client, block)
            .await?
            .result
            .into())
    }
}
