//! C Bridge
use super::runner::Runner;

/// Run the mmr service
#[no_mangle]
pub extern "C" fn run() -> i32 {
    env_logger::init();

    info!("starting mmr service...");
    if Runner::default().start().is_ok() {
        0
    } else {
        1
    }
}
