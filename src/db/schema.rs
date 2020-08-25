//! Data schemas
#![allow(missing_docs)]

table! {
    mmr_store(pos) {
        elem -> Text,
        pos -> BigInt,
    }
}
