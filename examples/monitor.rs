use darwinia_shadow::{hex, mmr::H256, ShadowShared};
use rocksdb::IteratorMode;

fn main() {
    let db = ShadowShared::new(None).db;
    for (k, v) in db.iterator(IteratorMode::Start) {
        let hash: [u8; 32] = H256::from_bytes(&v.to_vec());
        let mut pos = [0; 8];
        pos.copy_from_slice(&k);
        println!("| {} | 0x{} |", usize::from_le_bytes(pos), hex!(&hash));
    }
}
