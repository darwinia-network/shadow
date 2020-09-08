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

pub async fn get_confirmations(client: &Client, block: u64) -> Result<u64, Error> {
    let map: Value = serde_json::from_str(&format! {
        "{{{}}}", vec![
            r#""jsonrpc":"2.0","#,
            r#""method":"eth_blockNumber","#,
            r#""params":[],"#,
            r#""id": 1"#,
        ].concat(),
    })?;

    let rpc_url = env::var("ETHEREUM_RPC").unwrap_or_else(|_| {
        if env::var("ETHEREUM_ROPSTEN").is_ok() {
            crate::conf::DEFAULT_ETHEREUM_ROPSTEN_RPC.into()
        } else {
            crate::conf::DEFAULT_ETHEREUM_RPC.into()
        }
    });

    let result = client
        .post(&rpc_url)
        .json(&map)
        .send()
        .await?;

    println!("{:?}", result);
    let resp = result.json::<EthBlockNumberResp>().await?;

    // println!("{:?}", resp);
    let raw
        = resp.result.trim_start_matches("0x");
    let current_height = i64::from_str_radix(&raw, 16).unwrap_or(0) as u64;

    Ok(current_height - block)
}
