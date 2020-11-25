use cmmr::MMR;
use mysql::*;
use mysql::prelude::*;

use crate::{Result, MergeHash, H256, MMRError, MmrClientTrait};
use crate::MysqlStore;
use std::path::PathBuf;

pub struct MmrClientForMysql {
    db: Pool
}

impl MmrClientForMysql {
    /// create a new client instance
    pub fn new(db: Pool) -> Self {
        MmrClientForMysql { db }
    }
}

impl MmrClientTrait for MmrClientForMysql {

    /// push single element to mmr
    fn push(&mut self, elem: &str) -> Result<u64> {
        let mut conn = self.db.get_conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;

        // push elem
        let store = MysqlStore::new(self.db.clone(), &mut tx);
        let mut mmr = MMR::<[u8; 32], MergeHash, _>::new(self.get_mmr_size()?, store);
        let elem = H256::from(elem)?;
        let position = mmr.push(elem)?;
        let root = H256::hex(&mmr.get_root()?);
        mmr.commit()?;

        // update its mmr root and leaf_index
        // leaf index
        let leaf_index = match self.get_last_leaf_index()? {
            Some(last_leaf_index) => last_leaf_index + 1,
            None => 0
        };
        //
        let stmt = tx.prep("UPDATE mmr SET root=:root, leaf_index=:leaf_index WHERE position=:position").unwrap();
        tx.exec_iter(&stmt, params! { root, leaf_index, position })?;
        tx.commit()?;

        Ok(position)
    }

    /// push elements to mmr
    fn batch_push(&mut self, elems: &[&str]) -> Result<Vec<u64>> {
        let mut result = vec![];

        let mut conn = self.db.get_conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;

        // 1. push elems to mmr
        let store = MysqlStore::new(self.db.clone(), &mut tx);
        let mut mmr = MMR::<[u8; 32], MergeHash, _>::new(self.get_mmr_size()?, store);
        let mut root_pos_list = vec![];
        for &elem in elems {
            let elem = H256::from(elem)?;
            let position = mmr.push(elem)?;
            let root = H256::hex(&mmr.get_root()?);
            root_pos_list.push((position, root));
            result.push(position);
        }
        mmr.commit()?;

        // 2. update mmr roots and leaf_index
        // leaf index
        let mut leaf_index = match self.get_last_leaf_index()? {
            Some(last_leaf_index) => last_leaf_index + 1,
            None => 0
        };
        //
        let stmt = tx.prep("UPDATE mmr SET root=:root, leaf_index=:leaf_index WHERE position=:position").unwrap();
        for (position, root) in root_pos_list {
            // let height = pos_height_in_tree(position);
            tx.exec_iter(&stmt, params! { root, leaf_index, position })?;
            leaf_index += 1;
        }
        tx.commit()?;

        Ok(result)
    }

    /// get mmr size from db directly
    fn get_mmr_size(&self) -> Result<u64> {
        let mut conn = self.db.get_conn()?;
        let mut mmr_size = 0;
        if let Some(result) = conn.query_first::<Option<u64>, _>("SELECT MAX(position)+1 FROM mmr")? {
            if let Some(count) = result {
                mmr_size = count;
            }
        };

        Ok(mmr_size)
    }

    fn get_last_leaf_index(&self) -> Result<Option<u64>> {
        let mut conn = self.db.get_conn()?;
        let mut leaf_index = None;
        if let Some(result) = conn.query_first::<Option<u64>, _>("SELECT MAX(leaf_index) FROM mmr")? {
            if let Some(max) = result {
                leaf_index = Some(max);
            }
        };

        Ok(leaf_index)
    }

    fn get_elem(&self, pos: u64) -> Result<String> {
        let mut conn = self.db.get_conn()?;

        let result = conn.query_first::<String, _>(format!("SELECT hash FROM mmr WHERE position={}", pos))?;

        if let Some(hash) = result {
            Ok(hash)
        } else {
            Err(MMRError::ElementNotFound(pos))?
        }
    }

    fn gen_proof(&self, member: u64, last_leaf: u64) -> Result<Vec<String>> {
        let mut conn = self.db.get_conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;
        let store = MysqlStore::new(self.db.clone(), &mut tx);
        let mmr_size = cmmr::leaf_index_to_mmr_size(last_leaf);
        let mmr = MMR::<[u8; 32], MergeHash, _>::new(mmr_size, store);
        let proof = mmr.gen_proof(vec![cmmr::leaf_index_to_pos(member)])?;
        tx.commit()?;

        Ok(
            proof
                .proof_items()
                .iter()
                .map(|item| H256::hex(item))
                .collect::<Vec<String>>()
        )
    }

    fn backup(&self, _dir: &PathBuf) -> Result<()> {
        unimplemented!()
    }

    fn trim_from(&self, _leaf_index: u64) -> Result<()> {
        unimplemented!()
    }
}

#[test]
fn test_client() {
    use crate::MmrClientForMysql;
    let db = Pool::new("mysql://root:@localhost:3306/mmr_store".to_string()).unwrap();
    let mut client = MmrClientForMysql::new(db);

    client.push("c0c8c3f7dc9cdfa87d2433bcd72a744d634524a5ff76e019e44ea450476bac99").unwrap();
    // println!("{:?}", client.get_last_leaf_index());
}

#[test]
fn test_client_batch_push() {
    use crate::MmrClientForMysql;

    let hashs: [&str; 10] = [
        "34f61bfda344b3fad3c3e38832a91448b3c613b199eb23e5110a635d71c13c65",
        "70d641860d40937920de1eae29530cdc956be830f145128ebb2b496f151c1afb",
        "12e69454d992b9b1e00ea79a7fa1227c889c84d04b7cd47e37938d6f69ece45d",
        "3733bd06905e128d38b9b336207f301133ba1d0a4be8eaaff6810941f0ad3b1a",
        "3d7572be1599b488862a1b35051c3ef081ba334d1686f9957dbc2afd52bd2028",
        "2a04add3ecc3979741afad967dfedf807e07b136e05f9c670a274334d74892cf",
        "c58e247ea35c51586de2ea40ac6daf90eac7ac7b2f5c88bbc7829280db7890f1",
        "2cf0262f0a8b00cad22afa04d70fb0c1dbb2eb4a783beb7c5e27bd89015ff573",
        "05370d06def89f11486c994c459721b4bd023ff8c2347f3187e9f42ef39bddab",
        "c0c8c3f7dc9cdfa87d2433bcd72a744d634524a5ff76e019e44ea450476bac99",
    ];

    let db = Pool::new("mysql://root:@localhost:3306/mmr_store".to_string()).unwrap();
    let mut client = MmrClientForMysql::new(db);
    client.batch_push(&hashs).unwrap();
}