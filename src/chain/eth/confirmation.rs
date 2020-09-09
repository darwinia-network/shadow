use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

use std::{env, fmt::Debug};

use crate::result::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct EthBlockNumberResp {
    jsonrpc: String,
    id: i32,
    pub result: String,
}

/// Get ethereum confirmations
pub async fn get(client: &Client, block: u64) -> Result<u64, Error> {
    let map: Value = serde_json::from_str(&format! {
        "{{{}}}", vec![
            r#""jsonrpc":"2.0","#,
            r#""method":"eth_blockNumber","#,
            r#""params":[],"#,
            r#""id": 1"#,
        ].concat(),
    })?;

    Ok(i64::from_str_radix(
        &client
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
            .json::<EthBlockNumberResp>()
            .await?
            .result
            .trim_start_matches("0x"),
        16,
    )
    .unwrap_or(0) as u64
        - block)
}
