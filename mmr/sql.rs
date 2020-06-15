//! SQL Queries

/// Create mmr table
pub const CREATE_MMR_IF_NOT_EXISTS: &str = r#"
  CREATE TABLE IF NOT EXISTS mmr_store (
    elem   TEXT     NOT NULL,
    pos    INTEGER  NOT NULL
  )
"#;

/// Drop mmr table
pub const DROP_MMR_TABLE: &str = r#"DROP TABLE mmr_store"#;
