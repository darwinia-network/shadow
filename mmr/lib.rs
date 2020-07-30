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

mod model;
mod schema;
mod sql;

pub mod bridge;
mod ffi;
pub mod hash;
mod result;
mod runner;
pub mod store;

pub use runner::Runner;
