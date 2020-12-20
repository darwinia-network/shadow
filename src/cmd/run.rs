use crate::{mmr::database, mmr::Runner, result::Result};
use mmr::Database;
use tokio::select;
use std::sync::Arc;
use primitives::rpc::EthereumRPC;
use std::env;
use crate::epoch::EpochRunner;
use crate::darwinia::{DarwiniaClient, BlockSubscriber, EventHandler, DatabaseService};

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

    // Darwinia
    let darwinia_client = DarwiniaClient::new("ws://t1.hkg.itering.com:9944").await?;

    match mode.as_str() {
        "all" => {
            let runner = Runner::new(&eth, &mmr_db);
            let mut epoch_runner = EpochRunner::new(&eth);
            select! {
                _ = runner.start() => {
                    info!("Mmr Runner completed");
                },
                r = api::serve(port, &mmr_db, &eth) => {
                    error!("Api service completed: {:?}", r);
                },
                _ = epoch_runner.start() => {
                    info!("Epoch Runner completed");
                },
            };
        },
        "mmr" => {
            let runner = Runner::new(&eth, &mmr_db);
            runner.start().await;
        },
        "api" => {
            api::serve(port, &mmr_db, &eth).await?;
        },
        "epoch" => {
            let mut epoch_runner = EpochRunner::new(&eth);
            epoch_runner.start().await;
        },
        "darwinia" => {
            if let Database::Mysql(pool) = mmr_db {
                let db = DatabaseService::new(pool);
                let event_handler = EventHandler::new(db);
                let sub = BlockSubscriber::new(darwinia_client, event_handler).await;
                sub.start().await?;
            } 
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported mode: {}, only can be one of all, mmr, api, epoch", mode).into());
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
