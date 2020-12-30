/// Default Ethereum RPC
pub const DEFAULT_ETHEREUM_RPC: &str =
    r#"https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016"#;

/// Default Ethereum Ropsten RPC
pub const DEFAULT_ETHEREUM_ROPSTEN_RPC: &str =
    r#"https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016"#;

/// Default rocksdb file path
pub const DEFAULT_ROCKSDB_FILE: &str = ".shadow/cache/mmr";

/// EPOCH_INIT_BLOCK
pub const EPOCH_INIT_BLOCK: u64 = 15000u64;

/// EPOCH_LOCK_FILE
pub const EPOCH_LOCK_FILE: &str = ".shadow/proof.lock";

/// EPOCH_BLOCK_FILE
pub const EPOCH_BLOCK_FILE: &str = ".shadow/block";

// /// Default mysql uri
// pub const DEFAULT_MYSQL_URI: &str = "mysql://root:@localhost:3306/mmr_store";