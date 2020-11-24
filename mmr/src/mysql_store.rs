use crate::H256;
use cmmr::{MMRStore, Result as MMRResult};
use mysql::*;
use mysql::prelude::*;

/// Mysql MMR Store
pub struct MysqlStore<'a, 't> {
    /// Connection Pool
    pub db: Pool,
    pub(crate) tx: &'t mut Transaction<'a>,
}

impl<'a, 't> MysqlStore<'a, 't> {
    pub fn new(db: Pool, tx: &'t mut Transaction<'a>) -> Self {
        MysqlStore { db, tx }
    }
}

impl MMRStore<[u8; 32]> for MysqlStore<'_, '_> {
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

        self.tx.exec_batch(
            r"INSERT INTO mmr (position, hash, leaf) VALUES (:position, :hash, :leaf)",
            elems.into_iter().enumerate().map(|(i, elem)| {
                let position = pos + i;
                let params = params! {
                    "position" => position,
                    "hash" => H256::hex(&elem),
                    "leaf" => i == 0,
                };
                params
            })
        )
        .map_err(|err| cmmr::Error::StoreError(err.to_string()))?;

        Ok(())
    }
}
