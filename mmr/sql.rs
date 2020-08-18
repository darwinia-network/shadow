//! SQL Queries

/// Create mmr table
pub const CREATE_MMR_STORE_IF_NOT_EXISTS: &str = r#"
  CREATE TABLE IF NOT EXISTS mmr_store (
    elem   TEXT     NOT NULL,
    pos    INTEGER  NOT NULL
  )
"#;
