use crate::{api, mmr::MysqlRunner, result::Result, ShadowShared};

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

    let shared = ShadowShared::new(None);
    let mut runner = Runner::from(shared.clone());
    // let runner = MysqlRunner::new(shared.eth.clone());
    let (mr, ar) = futures::join!(api::serve(port, shared), runner.start());
    mr?;
    ar?;
    Ok(())
}
