use crate::mmr::merge;
use cmmr::Merge;

struct MergeHash;
impl Merge for MergeHash {
    type Item = [u8; 32];
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        merge(lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::MergeHash;
    use crate::mmr::{bag_rhs_peaks, gen_proof, gen_proof_positions, get_peaks};
    use blake2_rfc::blake2b::blake2b;
    use cmmr::{helper, util::MemStore, Error, MMRStore, Result, MMR};

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut dest = [0; 32];
        dest.copy_from_slice(blake2b(32, &[], data).as_bytes());
        dest
    }

    #[test]
    fn test_mmr_root() {
        let store = MemStore::default();
        let mut mmr = MMR::<_, MergeHash, _>::new(0, &store);
        let leaf_index = 1000;

        let _positions: Vec<u64> = (0u32..leaf_index)
            .map(|i| mmr.push(hash(&i.to_le_bytes())).unwrap())
            .collect();
        // mmrsize = leaf_index_to_pos(leaf_index);
        let mmrsize = mmr.mmr_size();
        let mmr_root1 = mmr.get_root();
        mmr.commit().unwrap();

        //1. get peaks
        let peak_positions = get_peaks(mmrsize);
        //2. query from db
        let peaks = peak_positions
            .into_iter()
            .map(|pos| {
                (&store)
                    .get_elem(pos)
                    .and_then(|elem| elem.ok_or(Error::InconsistentStore))
            })
            .collect::<Result<Vec<[u8; 32]>>>();
        // bag peaks
        let mmr_root2 = bag_rhs_peaks(peaks.unwrap());
        assert_eq!(mmr_root1, mmr_root2);
    }

    #[test]
    fn test_mmr_proof() {
        let store = MemStore::default();
        let mut mmr = MMR::<_, MergeHash, _>::new(0, &store);
        let leaf_index = 20000;
        let check_pos = helper::leaf_index_to_pos(7820);

        let _positions: Vec<u64> = (0u32..leaf_index+1)
            .map(|i| mmr.push(hash(&i.to_le_bytes())).unwrap())
            .collect();
        let mmrsize_fromindex = cmmr::leaf_index_to_pos((leaf_index+1).into());
        let mmrsize = mmr.mmr_size();
        assert_eq!(mmrsize_fromindex, mmrsize);
        let mmr_proof1 = mmr.gen_proof(vec![check_pos]).unwrap();
        let root = mmr.get_root().unwrap();
        mmr.commit().unwrap();

        // 1. gen positions
        let (merkle_proof_pos, peaks_pos, peak_pos) = gen_proof_positions(check_pos, mmrsize);
        // 2. query hash from db
        let merkle_proof = merkle_proof_pos
            .iter()
            .map(|pos| {
                (&store)
                    .get_elem(*pos)
                    .and_then(|elem| elem.ok_or(Error::InconsistentStore))
            })
            .collect::<Result<Vec<[u8; 32]>>>();
        let peaks = peaks_pos
            .iter()
            .map(|pos| (*pos, (&store).get_elem(*pos).unwrap().unwrap()))
            .collect::<Vec<(u64, [u8; 32])>>();
        // 3. gen proof
        let mmr_proof2 = gen_proof(merkle_proof.unwrap(), peaks, peak_pos);
        assert_eq!(mmr_proof1.proof_items(), mmr_proof2);
        assert_eq!(true, mmr_proof1.verify(root, vec![(check_pos, hash(&7820i32.to_le_bytes()))]).unwrap());
        let mmr_proof_instance = cmmr::MerkleProof::<[u8;32], MergeHash>::new(mmrsize, mmr_proof2);
        assert_eq!(true, mmr_proof_instance.verify(root, vec![(check_pos, hash(&7820i32.to_le_bytes()))]).unwrap());
    }
}