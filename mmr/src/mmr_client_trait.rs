use crate::error::Result;
use std::path::PathBuf;

pub trait MmrClientTrait {
    fn push(&mut self, elem: &str) -> Result<u64>;
    fn batch_push(&mut self, elems: &[&str]) -> Result<Vec<u64>>;
    fn get_mmr_size(&self) -> Result<u64>;
    fn get_last_leaf_index(&self) -> Result<Option<u64>>;
    fn get_elem(&self, pos: u64) -> Result<Option<String>>;
    fn get_leaf(&self, leaf_index: u64) -> Result<Option<String>>;
    fn get_mmr_root(&self, leaf_index: u64) -> Result<Option<String>>;
    fn gen_proof(&self, member: u64, last_leaf: u64) -> Result<Vec<String>>;
    /// delete from leaf_index, include the leaf_index
    fn trim_from(&self, leaf_index: u64) -> Result<()>;

    fn count(&self) -> Result<u64> {
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
        let from = self.count()?;
        if from >= til_block {
            return Err(anyhow::anyhow!("The to position of mmr is {}, can not import mmr from {}, from must be less than to",
                til_block, from
            ).into());
        }

        // Get hashes
        info!("Importing ethereum headers from {:?}...", geth_dir);
        let hashes = ffi::import(geth_dir.to_str().unwrap(), from as i32, til_block as i32);
        let hashes_vec: Vec<&str> = hashes.split(',').collect::<Vec<&str>>();

        // Check empty
        info!("Imported {} hashes from ethereum node", hashes_vec.len());
        if hashes_vec[0].is_empty() {
            return Err(anyhow::anyhow!("Importing hashes from {:?} failed", geth_dir ).into());
        }

        // Push
        let mut ptr = from;
        let mut batch = vec![];
        for hash in hashes_vec {
            batch.push(hash);
            if ptr % 1000 == 0 {
                trace!("Start to push hash into mmr for block {:?}/{}", ptr as usize, til_block);
                self.batch_push(&batch)?;
                batch = vec![];
            }

            ptr += 1;
        }

        if batch.len() > 0 {
            self.batch_push(&batch)?;
        }

        info!("done.");
        Ok(())
    }

}