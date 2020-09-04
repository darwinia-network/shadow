use crate::{
    mmr::{helper, Runner},
    result::Error,
    ShadowShared,
};

/// Trim mmrs
pub fn exec(leaf: u64) -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    let mut runner = Runner::from(shared.clone());

    runner.trim(leaf).unwrap();
    println!("Trimed leaves greater and equal than {}", leaf);
    println!(
        "Current best block: {}",
        helper::mmr_size_to_last_leaf(runner.mmr_count() as i64)
    );

    Ok(())
}
