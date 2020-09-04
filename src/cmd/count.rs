use crate::{mmr::Runner, result::Error, ShadowShared};

/// Count mmr
pub fn exec() -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    let runner = Runner::from(shared.clone());
    println!(
        "Current best block: {}",
        crate::mmr::helper::mmr_size_to_last_leaf(runner.mmr_count() as i64)
    );

    Ok(())
}
