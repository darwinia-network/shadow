use mysql::Pool;
use mysql::prelude::Queryable;
use crate::result::Result;

pub struct DatabaseService {
    mysql: Pool
}

impl DatabaseService {
    pub fn new(mysql: Pool) -> Self {
        DatabaseService {
            mysql
        }
    }

    pub async fn save_lock(&self, block: u32, account_id: String, ecdsa_address: String, asset_type: u8, amount: u128) -> Result<()> {
        let mysql = self.mysql.clone();
        tokio::task::spawn_blocking(move || {
            let sql = format!(
                "INSERT INTO darwinia_locks (block, account_id, ecdsa_address, asset_type , amount) VALUES ({}, '{}', '{}', {}, {})",
                block,
                account_id,
                ecdsa_address,
                asset_type,
                amount,
            );
            mysql.get_conn().and_then(|mut conn| conn.query_drop(sql))
        }).await??;
        Ok(())
    }
}
