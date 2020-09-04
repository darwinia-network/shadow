//! `shadow` command
use crate::result::Error;
use structopt::{clap::AppSettings, StructOpt};

mod count;
mod run;
mod trim;

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
    match Opt::from_args() {
        Opt::Run { port, verbose } => run::exec(port, verbose).await,
        Opt::Count => count::exec(),
        Opt::Trim { leaf } => trim::exec(leaf),
    }?;

    Ok::<(), Error>(())
}
