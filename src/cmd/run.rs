use std::env;
use std::sync::Arc;

use shadow_types::rpc::EthereumRPC;
use tokio::select;

use crate::{result::Result, runner::Runner};

/// Run shadow service
pub async fn exec(port: u16, verbose: bool, mode: String) -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        if verbose {
            env::set_var("RUST_LOG", "info,darwinia_shadow");
        } else {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    let eth = Arc::new(ethereum_rpc());

    match mode.as_str() {
        "all" => {
            let runner = Runner::new(&eth);
            select! {
                _ = runner.start() => {
                    info!("Mmr Runner completed");
                },
                r = api::serve(port, &eth) => {
                    error!("Api service completed: {:?}", r);
                },
            };
        }
        "runner" => {
            let runner = Runner::new(&eth);
            runner.start().await;
        }
        "api" => {
            api::serve(port, &eth).await?;
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported mode: {}, only can be one of all, mmr, api, epoch",
                mode
            )
            .into());
        }
    }

    Ok(())
}

fn ethereum_rpc() -> EthereumRPC {
    let rpcs = env::var("ETHEREUM_RPC")
        .unwrap_or_else(|_| {
            if env::var("ETHEREUM_ROPSTEN").is_ok() {
                crate::conf::DEFAULT_ETHEREUM_ROPSTEN_RPC.into()
            } else {
                crate::conf::DEFAULT_ETHEREUM_RPC.into()
            }
        })
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    info!("Available ethereum rpc urls: \n{:#?}", rpcs);
    EthereumRPC::new(reqwest::Client::new(), rpcs)
}
