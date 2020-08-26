//! `shadow` command
use crate::{api, mmr::helper, pool, result::Error, Runner};
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
pub fn exec() -> Result<(), Error> {
    let opt = Opt::from_args();
    let conn = pool::conn(None);
    let mut runner = Runner::with(conn);

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

            // Start mmr service
            thread::spawn(move || runner.start().unwrap());

            // Start http server
            actix_rt::Runtime::new()
                .unwrap()
                .block_on(api::serve(port))?;
        }
        Opt::Count => {
            println!(
                "Current best block: {}",
                helper::mmr_size_to_last_leaf(runner.mmr_count().unwrap())
            );
        }
        Opt::Trim { leaf } => {
            runner.trim(leaf).unwrap();
            println!("Trimed leaves greater and equal than {}", leaf);
            println!(
                "Current best block: {}",
                helper::mmr_size_to_last_leaf(runner.mmr_count().unwrap())
            );
        }
    };

    Ok::<(), Error>(())
}
