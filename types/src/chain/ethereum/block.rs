use std::{fmt::Debug, str::FromStr};

use codec::{Decode, Encode};

use crate::{
    array::{Bloom, U256},
    hex,
};

/// Raw EthereumBlock from Ethereum rpc
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthereumBlockRPC {
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    /// Ethereum header hash
    pub hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    /// Parent hash
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: String,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: String,
    pub transactions_root: String,
    /// Block transactions
    pub transactions: Vec<String>,
    pub uncles: Vec<String>,
    pub base_fee_per_gas: Option<String>,
}

impl From<EthereumBlockRPC> for EthereumHeader {
    fn from(that: EthereumBlockRPC) -> Self {
        let seal: Vec<Vec<u8>> = vec![
            rlp::encode(&bytes!(that.mix_hash.as_str())),
            rlp::encode(&bytes!(that.nonce.as_str())),
        ];
        EthereumHeader {
            parent_hash: bytes!(that.parent_hash.as_str(), 32),
            timestamp: u64::from_str_radix(&that.timestamp.as_str()[2..], 16).unwrap_or_default(),
            number: u64::from_str_radix(&that.number.as_str()[2..], 16).unwrap_or_default(),
            author: bytes!(that.miner.as_str(), 20),
            transactions_root: bytes!(that.transactions_root.as_str(), 32),
            uncles_hash: bytes!(that.sha3_uncles.as_str(), 32),
            extra_data: bytes!(that.extra_data.as_str()),
            state_root: bytes!(that.state_root.as_str(), 32),
            receipts_root: bytes!(that.receipts_root.as_str(), 32),
            log_bloom: Bloom(bytes!(that.logs_bloom.as_str(), 256)),
            gas_used: U256::from_str(&that.gas_used[2..]).unwrap_or_default(),
            gas_limit: U256::from_str(&that.gas_limit[2..]).unwrap_or_default(),
            difficulty: U256::from_str(&that.difficulty[2..]).unwrap_or_default(),
            seal,
            hash: match that.hash.is_empty() {
                true => None,
                false => Some(bytes!(that.hash.as_str(), 32)),
            },
            base_fee_per_gas: that
                .base_fee_per_gas
                .map(|v| U256::from_str(&v[2..]).unwrap_or_default()),
        }
    }
}

/// Darwinia Eth header
#[derive(Clone, Decode, Encode, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EthereumHeader {
    pub parent_hash: [u8; 32],
    pub timestamp: u64,
    /// Block number
    pub number: u64,
    pub author: [u8; 20],
    pub transactions_root: [u8; 32],
    pub uncles_hash: [u8; 32],
    pub extra_data: Vec<u8>,
    pub state_root: [u8; 32],
    pub receipts_root: [u8; 32],
    pub log_bloom: Bloom,
    pub gas_used: U256,
    pub gas_limit: U256,
    pub difficulty: U256,
    pub seal: Vec<Vec<u8>>,
    /// Ethereum header hash
    pub hash: Option<[u8; 32]>,
    pub base_fee_per_gas: Option<U256>,
}

/// Darwinia Eth header Json foramt
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default, Encode, Clone)]
pub struct EthereumHeaderJson {
    pub parent_hash: String,
    pub timestamp: u64,
    /// Block Number
    pub number: u64,
    pub author: String,
    pub transactions_root: String,
    pub uncles_hash: String,
    pub extra_data: String,
    pub state_root: String,
    pub receipts_root: String,
    pub log_bloom: String,
    pub gas_used: u128,
    pub gas_limit: u128,
    pub difficulty: u128,
    pub seal: Vec<String>,
    pub hash: String,
    pub base_fee_per_gas: Option<u128>,
}

impl From<EthereumHeader> for EthereumHeaderJson {
    fn from(that: EthereumHeader) -> Self {
        EthereumHeaderJson {
            parent_hash: format!("0x{}", hex!(that.parent_hash.to_vec())),
            timestamp: that.timestamp,
            number: that.number,
            author: format!("0x{}", hex!(that.author.to_vec())),
            transactions_root: format!("0x{}", hex!(that.transactions_root.to_vec())),
            uncles_hash: format!("0x{}", hex!(that.uncles_hash.to_vec())),
            extra_data: format!("0x{}", hex!(that.extra_data.to_vec())),
            state_root: format!("0x{}", hex!(that.state_root.to_vec())),
            receipts_root: format!("0x{}", hex!(that.receipts_root.to_vec())),
            log_bloom: format!("0x{}", hex!(that.log_bloom.0.to_vec())),
            gas_used: that.gas_used.as_u128(),
            gas_limit: that.gas_limit.as_u128(),
            difficulty: that.difficulty.as_u128(),
            seal: that
                .seal
                .iter()
                .map(|s| format!("0x{}", hex!(s.to_vec())))
                .collect(),
            hash: format!("0x{}", hex!(that.hash.unwrap_or_default().to_vec())),
            base_fee_per_gas: that.base_fee_per_gas.map(|v| v.as_u128()),
        }
    }
}

impl From<EthereumHeaderJson> for EthereumHeader {
    fn from(that: EthereumHeaderJson) -> Self {
        EthereumHeader {
            parent_hash: bytes!(that.parent_hash.as_str(), 32),
            timestamp: that.timestamp,
            number: that.number,
            author: bytes!(that.author.as_str(), 20),
            transactions_root: bytes!(that.transactions_root.as_str(), 32),
            uncles_hash: bytes!(that.uncles_hash.as_str(), 32),
            extra_data: bytes!(that.extra_data.as_str()),
            state_root: bytes!(that.state_root.as_str(), 32),
            receipts_root: bytes!(that.receipts_root.as_str(), 32),
            log_bloom: Bloom(bytes!(that.log_bloom.as_str(), 256)),
            gas_used: U256::from(that.gas_used),
            gas_limit: U256::from(that.gas_limit),
            difficulty: U256::from(that.difficulty),
            seal: that.seal.iter().map(|s| bytes!(s.as_str())).collect(),
            hash: Some(bytes!(that.hash.as_str(), 32)),
            base_fee_per_gas: that.base_fee_per_gas.map(|v| U256::from(v)),
        }
    }
}
