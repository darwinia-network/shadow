use crate::{mmr::database, mmr::Runner, result::Result};
use std::sync::Arc;
use primitives::rpc::EthereumRPC;
use std::env;

/// Run shadow service
pub async fn exec(port: u16, verbose: bool, uri: Option<String>) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        if verbose {
            std::env::set_var("RUST_LOG", "info,darwinia_shadow");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    // Build eth client
    let eth = Arc::new(ethereum_rpc());

    // Build Runner
    let runner = Runner::new(eth, database(uri)?);

    // Start
    runner.start().await?;

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
