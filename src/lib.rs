//! # Shadow
//!
//! [![Golang CI][workflow-badge]][github]
//!
//! The shadow service for relayers and verify workers to retrieve header data and generate proof. Shadow will index the data it needs from blockchain nodes, such as Ethereum and Darwinia.
//!
//! ## Usage
//!
//! ```text
//! shadow 0.1.0
//!
//! USAGE:
//!     shadow <SUBCOMMAND>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! SUBCOMMANDS:
//!     count    Current block height in mmr store
//!     help     Prints this message or the help of the given subcommand(s)
//!     run      Start shadow service
//!     trim     Trim mmr from target leaf
//! ```
//!
//! ## Contribute and Build
//!
//! Downloads shadow service
//!
//! ```text
//! git clone https://github.com/darwinia-network/shadow.git
//! ```
//!
//! Starts shadow service:
//!
//! ```text
//! # Starts shadow serives at port 3000
//! $ cargo run -p 3000
//!
//! # If you have fast eth node:
//! $ ETHEREUM_RPC=<your-api> cargo run -p 3000
//! ```
//!
//! ## Trouble Shooting
//!
//! Everytime you run `proof` in error, please delete `~/.ethashproof` and `~/.ethash`
//! and retry.
//!
//! ## LICENSE
//!
//! GPL-3.0
//!
//!
//! [infura]: https://infura.io
//! [github]: https://github.com/darwinia-network/shadow
//! [spec]: https://github.com/darwinia-network/darwinia/wiki/Darwinia-offchain-worker-shadow-service-spec
//! [workflow-badge]: https://github.com/darwinia-network/shadow/workflows/Golang%20CI/badge.svg
//! [api]: https://darwinia-network.github.io/shadow

#![warn(missing_docs)]
#![allow(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod conf;
mod db;
mod mmr;
mod result;

pub mod api;
pub mod chain;
pub mod cmd;
pub use self::{
    db::{model, pool, schema, sql},
    mmr::{hash, helper, runner::Runner, store},
};
