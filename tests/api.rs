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
        target: 1,
        last_leaf: 0,
    };

    // New relay call - Round 0
    let req = ProposalReq {
        leaves: vec![confirmed.target],
        target: 10,
        last_leaf: 9,
    };

    // Verify MMR
    let p = MerkleProof::<[u8; 32], MergeHash>::new(
        cmmr::leaf_index_to_mmr_size(req.last_leaf),
        req.mmr_proof(&shared.store)
            .await
            .into_iter()
            .map(|h| H256::from(&h))
            .collect(),
    );

    // Expand leaves
    let mut leaves = vec![];
    for l in &req.leaves {
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
    assert!(p
        .verify(H256::from(&req.mmr_root(&shared.store)), leaves)
        .unwrap_or(false));

    // New Round 1
    let req_r1 = ProposalReq {
        leaves: vec![confirmed.target],
        target: 9,
        last_leaf: 9,
    };
}
