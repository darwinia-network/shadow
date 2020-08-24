//! Data schemas
#![allow(missing_docs)]

table! {
    mmr_store(pos) {
        elem -> Text,
        pos -> BigInt,
    }
}

table! {
    eth_header_with_proof_caches(number) {
        hash -> Text,
        number -> BigInt,
        root -> Nullable<Text>,
    }
}
