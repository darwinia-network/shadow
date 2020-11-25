use crate::error::Result;
use std::path::PathBuf;

pub trait MmrClientTrait {
    fn push(&mut self, elem: &str) -> Result<u64>;
    fn batch_push(&mut self, elems: &[&str]) -> Result<Vec<u64>>;
    fn get_mmr_size(&self) -> Result<u64>;
    fn get_last_leaf_index(&self) -> Result<Option<u64>>;
    fn get_elem(&self, pos: u64) -> Result<String>;
    fn gen_proof(&self, member: u64, last_leaf: u64) -> Result<Vec<String>>;
    fn backup(&self, dir: &PathBuf) -> Result<()>;
    /// delete from leaf_index, include the leaf_index
    fn trim_from(&self, leaf_index: u64) -> Result<()>;

    fn count(&self) -> Result<Option<u64>> {
        self.get_last_leaf_index()
            .map(|option| option.map(|index| index + 1))
    }
}