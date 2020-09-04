use crate::{api, mmr::Runner, result::Error, ShadowShared};

/// Run shadow service
pub async fn exec(port: u16, verbose: bool) -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    let mut runner = Runner::from(shared.clone());

    if std::env::var("RUST_LOG").is_err() {
        if verbose {
            std::env::set_var("RUST_LOG", "info,darwinia_shadow");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    let (mr, ar) = futures::join!(runner.start(), api::serve(port, shared));
    mr?;
    ar?;
    Ok(())
}
