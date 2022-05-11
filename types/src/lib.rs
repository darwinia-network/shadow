#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::manual_range_contains)]
#[macro_use]
extern crate serde;

// macros
mod byte;

pub mod array;
pub mod chain;
pub mod result;
pub mod rpc;
