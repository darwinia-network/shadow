//! `shadow` commands
use crate::result::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

mod count;
mod export;
mod run;
mod trim;
mod import;

#[derive(StructOpt)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Current block height in mmr store
    Count {
        /// Uri to db, Mysql url or Rocksdb path
        #[structopt(short, long)]
        uri: Option<String>,
    },
    /// Start shadow service
    Run {
        /// Http server port
        #[structopt(short, long, default_value = "3000")]
        port: u16,
        /// Uri to db, Mysql url or Rocksdb path
        #[structopt(short, long)]
        uri: Option<String>,
        /// Verbose mode
        #[structopt(short, long)]
        verbose: bool,
        /// Run mode, all, mmr, web
        #[structopt(short, long, default_value = "all")]
        mode: String,
    },
    /// Imports mmr from shadow backup or geth
    Import {
        /// Datadir of geth
        #[structopt(short, long)]
        path: String,
        /// To Ethereum block height
        #[structopt(short, long, default_value = "8000000")]
        to: u64,
        /// Uri to db, Mysql url or Rocksdb path
        #[structopt(short, long)]
        uri: Option<String>,
    },
    /// Exports shadow's rocksdb
    Export {
        /// Target datadir
        #[structopt(short, long)]
        dist: Option<PathBuf>,
        /// Uri to db, Mysql url or Rocksdb path
        #[structopt(short, long)]
        uri: Option<String>,
    },
    /// Trim mmr from target leaf
    Trim {
        /// The target leaf
        #[structopt(short, long)]
        leaf: u64,
        /// Uri to db, Mysql url or Rocksdb path
        #[structopt(short, long)]
        uri: Option<String>,
    },
}

/// Exec `shadow` binary
pub async fn exec() -> Result<()> {
    match Opt::from_args() {
        Opt::Count { uri } => count::exec(uri),
        Opt::Run { port, verbose, uri, mode } => run::exec(port, verbose, uri, mode).await,
        Opt::Import { path, to, uri } => import::exec(path, to, uri),
        Opt::Trim { leaf, uri } => trim::exec(leaf, uri),
        Opt::Export { dist, uri } => export::exec(dist, uri),
    }?;

    Ok(())
}
