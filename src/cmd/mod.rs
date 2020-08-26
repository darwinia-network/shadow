//! `shadow` command
use crate::{api, pool, Runner};
use std::thread;
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
    /// Cut mmr from target leaf
    Cut {
        /// The target leaf
        #[structopt(short, long)]
        _leaf: u64,
    },
}

/// Exec `shadow` binary
pub async fn exec() -> std::io::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Run { port, verbose } => {
            if let Err(_) = std::env::var("RUST_LOG") {
                if verbose {
                    std::env::set_var("RUST_LOG", "info,shadow");
                } else {
                    std::env::set_var("RUST_LOG", "info");
                }
            }

            env_logger::init();
            thread::spawn(move || {
                // Start mmr service
                let conn = pool::conn(None);
                let mut runner = Runner::with(conn);
                runner.start().unwrap();
            });

            // Start http server
            api::serve(port).await
        }
        Opt::Cut { _leaf: _ } => Ok(()),
    }
}
