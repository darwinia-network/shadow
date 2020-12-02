use crate::H256;
use cmmr::{MMRStore, Result as MMRResult};
use mysql::*;
use mysql::prelude::*;

/// Mysql MMR Store
#[allow(dead_code)]
pub struct MysqlStore<'a, 't, 'b> {
    /// Connection Pool
    pub db: Pool,
    pub(crate) tx: &'t mut Transaction<'a>,
    pub batch: &'b mut Vec<String>,
}

impl<'a, 't, 'b> MysqlStore<'a, 't, 'b> {
    pub fn new(db: Pool, tx: &'t mut Transaction<'a>, batch: &'b mut Vec<String>) -> Self {
        MysqlStore { db, tx, batch }
    }
}

impl MMRStore<[u8; 32]> for MysqlStore<'_, '_, '_> {
    fn get_elem(&self, pos: u64) -> MMRResult<Option<[u8; 32]>> {
        let mut conn = self
            .db.get_conn()
            .map_err(|err| cmmr::Error::StoreError(err.to_string()))?;

        let result = conn
            .query_first::<String, _>(format!("SELECT hash FROM mmr WHERE position={}", pos))
            .map_err(|err| cmmr::Error::StoreError(err.to_string()))?;

        if let Some(hash) = result {
            Ok(Some(H256::from(&hash).unwrap()))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, pos: u64, elems: Vec<[u8; 32]>) -> MMRResult<()> {
        let pos = pos as usize;
        for (i, elem) in elems.into_iter().enumerate() {
            let value = format!("({}, '{}', {})", pos + i, H256::hex(&elem), i == 0);
            self.batch.push(value);
        }
        Ok(())
    }
}
