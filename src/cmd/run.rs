use crate::{mmr::database, mmr::Runner, result::Result};
use tokio::join;
use std::sync::Arc;
use primitives::rpc::EthereumRPC;
use std::env;

/// Run shadow service
pub async fn exec(port: u16, verbose: bool, uri: Option<String>, mode: String) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        if verbose {
            std::env::set_var("RUST_LOG", "info,darwinia_shadow");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    // Shared data
    let mmr_db = database(uri)?;
    let eth = Arc::new(ethereum_rpc());

    match mode.as_str() {
        "all" => {
            let runner = Runner::new(&eth, &mmr_db);
            let (a, b) = join!(
                runner.start(),
                api::serve(port, &mmr_db, &eth),
            );
            a?;
            b?;
        },
        "mmr" => {
            let runner = Runner::new(&eth, &mmr_db);
            runner.start().await?;
        },
        "web" => {
            api::serve(port, &mmr_db, &eth).await?;
        },
        _ => {
            return Err(anyhow::anyhow!("Unsupported mode: {}, only can be one of all, mmr and web", mode).into());
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


    info!("Avaiable ethereum rpc urls: \n{:#?}", rpcs);
    EthereumRPC::new(reqwest::Client::new(), rpcs)
}