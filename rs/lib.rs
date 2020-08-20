//! Darwinia MMR Implementation
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

mod bridge;
mod db;
mod ffi;
mod mmr;
mod result;

pub use db::{model, pool, schema, sql};
pub use mmr::{hash, helper, runner::Runner, store};
