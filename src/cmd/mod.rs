//! `shadow` commands
use crate::result::Error;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

mod count;
mod export;
mod import;
mod run;
mod trim;

#[derive(StructOpt)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Current block height in mmr store
    Count,
    /// Start shadow service
    Run {
        /// Http server port
        #[structopt(short, long, default_value = "3000")]
        port: u16,
        /// Verbose mode
        #[structopt(short, long)]
        verbose: bool,
    },
    /// Imports mmr from geth
    Import {
        /// Datadir of geth
        #[structopt(short, long)]
        path: String,
        /// From Ethereum block height
        #[structopt(short, long)]
        from: i32,
        /// To Ethereum block height
        #[structopt(short, long)]
        to: i32,
    },
    /// Exports shadow's rocksdb
    Export {
        /// Target datadir
        #[structopt(short, long)]
        dist: PathBuf,
    },
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
        Opt::Count => count::exec(),
        Opt::Run { port, verbose } => run::exec(port, verbose).await,
        Opt::Import { path, from, to } => import::exec(path, from, to),
        Opt::Trim { leaf } => trim::exec(leaf),
        Opt::Export { dist } => export::exec(dist),
    }?;

    Ok::<(), Error>(())
}
