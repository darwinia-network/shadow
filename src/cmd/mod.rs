//! `shadow` command
use crate::{
    api,
    mmr::{helper, Runner},
    result::Error,
    ShadowShared,
};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Start shadow service
    Run {
        /// Http server port
        #[structopt(short, long, default_value = "3000")]
        port: u16,
        /// Verbose mode
        #[structopt(short, long)]
        verbose: bool,
    },
    /// Current block height in mmr store
    Count,
    /// Trim mmr from target leaf
    Trim {
        /// The target leaf
        #[structopt(short, long)]
        leaf: u64,
    },
}

/// Exec `shadow` binary
pub async fn exec() -> Result<(), Error> {
    let shared = ShadowShared::new(None);
    let mut runner = Runner::from(shared.clone());

    match Opt::from_args() {
        Opt::Run { port, verbose } => {
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
        }
        Opt::Count => {
            println!(
                "Current best block: {}",
                helper::mmr_size_to_last_leaf(runner.mmr_count() as i64)
            );
        }
        Opt::Trim { leaf } => {
            runner.trim(leaf).unwrap();
            println!("Trimed leaves greater and equal than {}", leaf);
            println!(
                "Current best block: {}",
                helper::mmr_size_to_last_leaf(runner.mmr_count() as i64)
            );
        }
    };

    Ok::<(), Error>(())
}
