use crate::error::Result;
pub trait MmrClientTrait {
    fn push(&mut self, elem: &str) -> Result<u64>;
    fn batch_push(&mut self, elems: &[&str]) -> Result<Vec<u64>>;
    fn get_mmr_size(&self) -> Result<u64>;
    fn get_last_leaf_index(&self) -> Result<Option<u64>>;
    fn get_elem(&self, pos: u64) -> Result<String>;
    fn gen_proof(&self, member: u64, last_leaf: u64) -> Result<Vec<String>>;
}