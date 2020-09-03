use cmmr::MerkleProof;
use darwinia_shadow::{
    api::eth::ProposalReq,
    chain::eth::EthHeader,
    mmr::{MergeHash, Runner, H256},
    ShadowShared,
};

#[actix_rt::test]
async fn test_proposal() {
    let shared = ShadowShared::new(None);
    let mut runner = Runner::from(shared.clone());

    // Gen mmrs
    assert!(runner.stops_at(30).await.is_ok());

    // Confirmed block on chain
    let confirmed = ProposalReq {
        leaves: vec![],
        target: 0,
        last_leaf: 0,
    };

    // New relay call - Round 0
    let req_r0 = ProposalReq {
        leaves: vec![confirmed.target],
        target: 3,
        last_leaf: 2,
    };

    // Verify MMR
    let p_r0 = MerkleProof::<[u8; 32], MergeHash>::new(
        cmmr::leaf_index_to_mmr_size(req_r0.last_leaf),
        req_r0
            .mmr_proof(&shared.store)
            .await
            .into_iter()
            .map(|h| H256::from(&h))
            .collect(),
    );

    // Expand leaves
    let mut leaves = vec![];
    for l in &req_r0.leaves {
        leaves.push((
            cmmr::leaf_index_to_pos(*l),
            EthHeader::get(&shared.client, *l)
                .await
                .unwrap()
                .hash
                .unwrap(),
        ));
    }

    // Should pass verification
    assert!(p_r0
        .verify(H256::from(&req_r0.mmr_root(&shared.store)), leaves)
        .unwrap_or(false));

    // New Round 1
    let req_r1 = ProposalReq {
        leaves: vec![2],
        target: 2,
        last_leaf: 2,
    };

    // Verify MMR
    let p_r1 = MerkleProof::<[u8; 32], MergeHash>::new(
        cmmr::leaf_index_to_mmr_size(req_r1.last_leaf),
        req_r1
            .mmr_proof(&shared.store)
            .await
            .into_iter()
            .map(|h| H256::from(&h))
            .collect(),
    );

    // Expand leaves
    let mut leaves = vec![];
    for l in &req_r1.leaves {
        leaves.push((
            cmmr::leaf_index_to_pos(*l),
            EthHeader::get(&shared.client, *l)
                .await
                .unwrap()
                .hash
                .unwrap(),
        ));
    }

    println!("last_leaf: {:?}", req_r1.last_leaf);
    println!("mmr_proof: {:?}", req_r1.mmr_proof(&shared.store).await);
    println!("mmr_root: {:?}", req_r0.mmr_root(&shared.store));
    // Should pass verification
    //
    // The the round 0's mmr_root to verify round 1's hash
    assert!(p_r1
        .verify(H256::from(&req_r0.mmr_root(&shared.store)), leaves)
        .unwrap_or(false));
}
