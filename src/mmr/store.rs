//! MMR store
use crate::{hash::H256, model::*, pool::ConnPool, schema::mmr_store::dsl::*, sql::*};
use cmmr::{Error, MMRStore, Result as MMRResult};
use diesel::{dsl::count, prelude::*};

/// MMR Store
#[derive(Clone)]
pub struct Store {
    /// Connection Pool
    pub pool: ConnPool,
}

impl Store {
    /// New store with path
    ///
    /// This is the very begining part of mmr service, panic when connect db failed.
    pub fn with(pool: ConnPool) -> Store {
        // Create store table
        diesel::sql_query(CREATE_MMR_STORE_IF_NOT_EXISTS)
            .execute(&pool.get().unwrap())
            .unwrap_or_default();

        Store { pool }
    }
}

impl<H> MMRStore<H> for &Store
where
    H: H256,
{
    fn get_elem(&self, rpos: u64) -> MMRResult<Option<H>> {
        let cfp = self.pool.get();
        if cfp.is_err() {
            return Err(Error::StoreError("Connect to database failed".into()));
        }

        let conn = cfp.unwrap();
        let store = mmr_store.filter(pos.eq(rpos as i64)).first::<Header>(&conn);
        if let Ok(store) = store {
            Ok(Some(H::from(store.elem.as_str())))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, rpos: u64, elems: Vec<H>) -> MMRResult<()> {
        let cfp = self.pool.get();
        if cfp.is_err() {
            return Err(Error::StoreError("Connect to database failed".into()));
        }

        let conn = cfp.unwrap();
        let mut the_count: u64 = 0;
        let count_res = mmr_store.select(count(elem)).first::<i64>(&conn);
        if let Ok(count) = count_res {
            the_count = count as u64;
        }

        if rpos != the_count {
            return Err(Error::InconsistentStore);
        }

        for (i, relem) in elems.into_iter().enumerate() {
            let header = Header::new(relem.hex(), rpos as i64 + i as i64);
            let res = diesel::insert_into(mmr_store)
                .values(&vec![header])
                .execute(&*conn);

            if res.is_err() {
                return Err(Error::StoreError(format!(
                    "Insert mmr of pos {} into sqlite3 failed, {:?}",
                    rpos as i64 + i as i64,
                    res,
                )));
            }
        }

        Ok(())
    }
}
