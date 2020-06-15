//! MMR Runner
use super::{
    schema::mmr_store::{dsl::*, *},
    store::DEFAULT_RELATIVE_MMR_DB,
    Error, MergeHash, Store,
};
use cmmr::MMR;
use diesel::{dsl::count, prelude::*};
use std::path::PathBuf;

/// MMR Runner
pub struct Runner {
    /// MMR Storage
    pub path: PathBuf,
}

impl Default for Runner {
    fn default() -> Runner {
        let mut path = dirs::home_dir().unwrap_or_default();
        path.push(DEFAULT_RELATIVE_MMR_DB);

        Runner { path }
    }
}

impl Runner {
    /// Get the count of mmr store
    pub fn count(&self) -> Result<i64, Error> {
        let store = Store::new(&self.path);
        let res = mmr_store.select(count(elem)).first::<i64>(&store.conn);
        if let Err(e) = res {
            Err(Error::Diesel(e))
        } else {
            Ok(res?)
        }
    }

    /// Push new header hash into storage
    pub fn push(&mut self, hash: [u8; 32]) -> Result<(), Error> {
        let store = Store::new(&self.path);
        let count = self.count()?;
        let mut mmr = MMR::<_, MergeHash, _>::new(count as u64, store);
        mmr.push(hash)?;

        let root = mmr.get_root()?;
        println!("{:?}", &root);

        mmr.commit()?;
        Ok(())
    }
}
