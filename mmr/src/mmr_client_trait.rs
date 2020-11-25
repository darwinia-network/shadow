use crate::error::Result;
use std::path::PathBuf;

pub trait MmrClientTrait {
    fn push(&mut self, elem: &str) -> Result<u64>;
    fn batch_push(&mut self, elems: &[&str]) -> Result<Vec<u64>>;
    fn get_mmr_size(&self) -> Result<u64>;
    fn get_last_leaf_index(&self) -> Result<Option<u64>>;
    fn get_elem(&self, pos: u64) -> Result<String>;
    fn gen_proof(&self, member: u64, last_leaf: u64) -> Result<Vec<String>>;
    fn backup(&self, dir: PathBuf) -> Result<()>;
    /// delete from leaf_index, include the leaf_index
    fn trim_from(&self, leaf_index: u64) -> Result<()>;
    fn import_from_backup(&self, backup_file: PathBuf) -> Result<()>;
    fn import_from_geth(&self, geth_dir: PathBuf, til_block: u64) -> Result<()>;

    fn count(&self) -> Result<u64> {
        let count = match self.get_last_leaf_index()? {
            None => 0,
            Some(last_leaf_index) => {
                last_leaf_index + 1
            }
        };

        Ok(count)
    }
}