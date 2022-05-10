//! Ethereum receipt
use std::fmt::Debug;

use codec::{Decode, Encode};
use rlp::{Encodable, RlpStream};
use serde::Deserialize;

use crate::array::{H160, H256};

/// Ethereum rsp response body
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthReceiptBody {
    /// the block hash
    pub block_hash: String,
    block_number: String,
    cumulative_gas_used: String,
    from: String,
    gas_used: String,
    logs: Vec<LogJson>,
    logs_bloom: String,
    #[serde(alias = "root")]
    status: String,
    to: String,
    transaction_hash: String,
    /// the transaction index
    pub transaction_index: String,
}

/// Ethereum receipt transaction out come
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub enum TransactionOutcome {
    /// Status and state root are unknown under EIP-98 rules.
    Unknown,
    /// State root is known. Pre EIP-98 and EIP-658 rules.
    StateRoot(H256),
    /// Status code is known. EIP-658 rules.
    StatusCode(u8),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogJson {
    address: String,
    topics: Vec<String>,
    data: String,
}

impl Encodable for LogEntry {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append_list(&self.data);
    }
}

/// Ethereum receipt log entry
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct LogEntry {
    /// The address of the contract executing at the point of the `LOG` operation.
    pub address: H160,
    /// The topics associated with the `LOG` operation.
    pub topics: Vec<H256>,
    /// The data associated with the `LOG` operation.
    pub data: Vec<u8>,
}
