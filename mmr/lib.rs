//! Darwinia MMR Implementation
#![warn(missing_docs)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod model;
mod schema;
mod sql;

mod ffi;
pub mod hash;
mod result;
mod runner;
pub mod store;

pub use runner::Runner;
