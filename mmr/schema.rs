//! Data schemas

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
        pos -> BigInt,
        root -> Nullable<Text>,
    }
}
