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
        each_range(from, til_block, 5000, |first, last| {
            let hashes = ffi::import(geth_dir.to_str().unwrap(), first as i32, (last + 1) as i32);
            let hashes_vec: Vec<&str> = hashes.split(',').collect::<Vec<&str>>();

            // Check empty
            if hashes_vec[0].is_empty() {
                return Err(anyhow::anyhow!("Importing hashes from {:?} failed, it is empty", geth_dir).into());
            }

            self.batch_push(&hashes_vec)?;
            info!("Block {} ~ {}'s hash has been pushed into mmr store", first, last);

            Ok(())
        })?;

        info!("Done.");
        Ok(())
    }

}

/// not include `to`
fn each_range<F>(from: u64, to: u64, range_size: u64, mut f: F) -> Result<()>
where F: FnMut(u64, u64) -> Result<()>
{
    let total = to - from;
    let ranges = total / range_size + 1;
    for page_index in 0..ranges {
        let first = from + page_index * range_size;
        let last = if page_index == ranges - 1 { // last page
            first + total % range_size - 1
        } else {
            first + range_size - 1
        };

        f(first, last)?;
    }

    Ok(())
}