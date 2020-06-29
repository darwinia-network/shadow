//! MMR store
use super::{hash::H256, model::*, schema::mmr_store::dsl::*, sql::*};
use cmmr::{Error, MMRStore, Result as MMRResult};
use diesel::{dsl::count, prelude::*};
use std::path::PathBuf;

/// Constants
pub const DEFAULT_RELATIVE_MMR_DB: &str = ".darwinia/cache/shadow.db";

/// MMR Store
pub struct Store {
    /// Database path
    pub path: PathBuf,
    /// Sqlite3 Connection
    pub conn: SqliteConnection,
}

impl Store {
    /// new conn
    pub fn conn(&self) -> SqliteConnection {
        SqliteConnection::establish(&self.path.to_string_lossy())
            .unwrap_or_else(|_| panic!("Error connecting to {:?}", &self.path))
    }

    /// New store with path
    ///
    /// This is the very begining part of mmr service, panic when connect db failed.
    pub fn new(p: &PathBuf) -> Store {
        let op_dir = p.parent();
        if op_dir.is_none() {
            panic!("Wrong db path: {:?}", p);
        }

        let dir = op_dir.unwrap();
        if !dir.exists() {
            let res = std::fs::create_dir_all(dir);
            if res.is_err() {
                panic!("Create dir failed: {:?}", res);
            }
        }

        let conn = SqliteConnection::establish(&p.to_string_lossy())
            .unwrap_or_else(|_| panic!("Error connecting to {:?}", p));

        // Create store table
        diesel::sql_query(CREATE_MMR_STORE_IF_NOT_EXISTS)
            .execute(&conn)
            .unwrap_or_default();

        Store {
            path: p.to_path_buf(),
            conn,
        }
    }

    /// Drop mmr table and create it again
    pub fn re_create(&self) -> Result<usize, diesel::result::Error> {
        let r = diesel::sql_query(DROP_MMR_TABLE).execute(&self.conn);
        if r.is_ok() {
            diesel::sql_query(CREATE_MMR_STORE_IF_NOT_EXISTS).execute(&self.conn)
        } else {
            r
        }
    }
}

impl Default for Store {
    fn default() -> Store {
        let mut root = dirs::home_dir().unwrap_or_default();
        root.push(DEFAULT_RELATIVE_MMR_DB);
        Store::new(&root)
    }
}

impl<H> MMRStore<H> for Store
where
    H: H256,
{
    fn get_elem(&self, rpos: u64) -> MMRResult<Option<H>> {
        let store = mmr_store
            .filter(pos.eq(rpos as i64))
            .first::<Header>(&self.conn);

        if let Ok(store) = store {
            Ok(Some(H::from(store.elem.as_str())))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, rpos: u64, elems: Vec<H>) -> MMRResult<()> {
        let mut the_count: u64 = 0;
        let count_res = mmr_store.select(count(elem)).first::<i64>(&self.conn);
        if let Ok(count) = count_res {
            the_count = count as u64;
        }

        if rpos != the_count {
            return Err(Error::InconsistentStore);
        }

        for (i, relem) in elems.into_iter().enumerate() {
            let header = Header::new(relem.hex(), rpos as i64 + i as i64);
            let res = diesel::replace_into(mmr_store)
                .values(&vec![header])
                .execute(&self.conn);

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
