use crate::{mmr::Runner, mmr::ClientType, result::Result};
use std::sync::Arc;
use crate::shared::ethereum_rpc;

/// Run shadow service
pub async fn exec(port: u16, verbose: bool) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        if verbose {
            std::env::set_var("RUST_LOG", "info,darwinia_shadow");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    // mysql runner
    let eth = Arc::new(ethereum_rpc());
    let runner = Runner::new(eth, ClientType::Mysql);
    runner.start().await?;

    // let shared = ShadowShared::new(None);
    // let mut runner = Runner::from(shared.clone());
    // let (mr, ar) = futures::join!(api::serve(port, shared), runner.start());
    // mr?;
    // ar?;

    Ok(())
}
