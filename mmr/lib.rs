#[macro_use]
extern crate diesel;

mod ethash;
mod store;

pub use self::ethash::{ETHash, MergeETHash};
