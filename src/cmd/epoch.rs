use crate::result::Result;

/// Count mmr
pub fn exec(block: u64) -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_shadow");
    env_logger::init();

    let epoch = block / 30_000;
    info!("start epoch block number {} epoch {}", block, epoch);
    ffi::start(0);
    ffi::epoch_wait(epoch);
    ffi::stop();

    Ok(())
}
