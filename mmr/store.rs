//! MMR store
use self::mmr_store::{columns::pos, dsl::*};
use super::{sql::*, H256};
use cmmr::{Error, MMRStore, Result as MMRResult};
use diesel::{dsl::count, prelude::*};
use std::path::PathBuf;

/// Constants
const RELATIVE_DB: &str = ".darwinia/cache/shadow.db";

/// Shadow db table
#[derive(AsChangeset, Clone, Insertable, Queryable, Debug)]
#[table_name = "mmr_store"]
pub struct Header {
    elem: String,
    number: i64,
    pos: i64,
}

impl Header {
    fn new(relem: String, rnumber: i64, rpos: i64) -> Header {
        Header {
            elem: relem,
            number: rnumber,
            pos: rpos,
        }
    }
}

table! {
    mmr_store(number) {
        elem -> Text,
        number -> BigInt,
        pos -> BigInt,
    }
}

/// MMR Store
pub struct Store {
    pub path: PathBuf,
    pub conn: SqliteConnection,
}

impl Store {
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
        diesel::sql_query(CREATE_MMR_IF_NOT_EXISTS)
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
            diesel::sql_query(CREATE_MMR_IF_NOT_EXISTS).execute(&self.conn)
        } else {
            r
        }
    }
}

impl Default for Store {
    fn default() -> Store {
        let mut root = dirs::home_dir().unwrap_or_default();
        root.push(RELATIVE_DB);
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

        if store.is_ok() {
            Ok(Some(H::from(store.unwrap().elem.as_str())))
        } else {
            Ok(None)
        }
    }

    fn append(&mut self, rpos: u64, elems: Vec<H>) -> MMRResult<()> {
        let mut the_count: u64 = 0;
        let count_res = mmr_store.select(count(elem)).first::<i64>(&self.conn);
        if count_res.is_ok() {
            the_count = count_res.unwrap() as u64;
        }

        if rpos != the_count {
            Err(Error::InconsistentStore)?;
        }

        for (i, relem) in elems.into_iter().enumerate() {
            let header = Header::new(relem.to_hex(), rpos as i64 + i as i64, 0);
            let res = diesel::replace_into(mmr_store)
                .values(&vec![header])
                .execute(&self.conn);

            if res.is_err() {
                return Err(Error::StoreError(format!(
                    "Insert mmr of pos {} into sqlite3 failed",
                    rpos as i64 + i as i64
                )));
            }
        }

        Ok(())
    }
}
