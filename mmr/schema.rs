//! Data schemas

table! {
    mmr_store(pos) {
        elem -> Text,
        pos -> BigInt,
    }
}
