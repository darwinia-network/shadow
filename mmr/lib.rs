#[macro_use]
extern crate diesel;

mod hash;
mod sql;
mod store;

pub use self::{
    hash::{hash, MergeHash, H256},
    store::Store,
};
