use crate::error::Result;
use std::path::PathBuf;
use std::time::Instant;

pub trait MmrClientTrait {
    fn push(&mut self, elem: &[u8; 32]) -> Result<u64>;
    fn batch_push(&mut self, elems: Vec<[u8; 32]>) -> Result<Vec<u64>>;
    fn get_mmr_size(&self) -> Result<u64>;
    fn get_last_leaf_index(&self) -> Result<Option<u64>>;
    fn get_elem(&self, pos: u64) -> Result<Option<String>>;
    fn get_leaf(&self, leaf_index: u64) -> Result<Option<String>>;
    fn get_mmr_root(&self, leaf_index: u64) -> Result<Option<String>>;
    fn gen_proof(&self, member: u64, last_leaf: u64) -> Result<Vec<String>>;
    /// delete from leaf_index, include the leaf_index
    fn trim_from(&self, leaf_index: u64) -> Result<()>;

    fn get_leaf_count(&self) -> Result<u64> {
        let count = match self.get_last_leaf_index()? {
            None => 0,
            Some(last_leaf_index) => {
                last_leaf_index + 1
            }
        };

        Ok(count)
    }

    fn backup(&self, dir: PathBuf) -> Result<()>;
    fn import_from_backup(&mut self, backup_file: PathBuf) -> Result<()>;

    fn import_from_geth(&mut self, geth_dir: PathBuf, til_block: u64) -> Result<()> {
        const BATCH: i32 = 10240;
        let start = Instant::now();
        let from = self.get_leaf_count()? as usize;
        if from >= til_block as usize {
            return Err(anyhow::anyhow!("The to position of mmr is {}, can not import mmr from {}, from must be less than to",
                til_block, from
            ).into());
        }
        let genesis = {
            match self.get_leaf(0)? {
                Some(hash) => hash,
                None => String::from(""),
            }
        };
        info!("Importing ethereum headers from {:?}", geth_dir);
        ffi::import(geth_dir.to_str().unwrap(), &genesis, from as i32, til_block as i32, BATCH, |hashes| {
            if let Err(err) = self.batch_push(hashes) {
                info!("batch push hashes failed, err {}", err);
                return false;
            }
            true
        });
        let elapsed = start.elapsed();
        info!("batch_push elapsed: {:?}", elapsed);
        info!("Done.");
        Ok(())
    }
}

