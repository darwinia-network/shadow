//! `shadow` commands
use crate::result::Result;
use structopt::{clap::AppSettings, StructOpt};

mod run;
mod epoch;

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
        /// Run mode, all, mmr, api, epoch
        #[structopt(short, long, default_value = "all")]
        mode: String,
    },
    /// Generate epoch data for ethash
    Epoch {
        /// Block number for epoch
        #[structopt(short, long)]
        block: u64,
    },
}

/// Exec `shadow` binary
pub async fn exec() -> Result<()> {
    match Opt::from_args() {
        Opt::Run { port, verbose, mode } => run::exec(port, verbose, mode).await,
        Opt::Epoch { block } => epoch::exec(block),
    }?;

    Ok(())
}
