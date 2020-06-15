//! Darwinia MMR Implementation
#![warn(missing_docs)]

#[macro_use]
extern crate diesel;

mod hash;
mod model;
mod result;
mod runner;
mod schema;
mod sql;
mod store;

pub use self::{
    hash::{hash, MergeHash, H256},
    result::Error,
    runner::Runner,
    store::Store,
};
