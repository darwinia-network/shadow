use cmmr::MerkleProof;
use darwinia_shadow::{
    api::eth::ProposalReq,
    chain::eth::EthHeader,
    db::pool,
    mmr::{MergeHash, Store, H256},
};
use reqwest::Client;

#[actix_rt::test]
async fn test_proposal() {
    let conn = pool::conn(None);
    let store = Store::with(conn);
    let client = Client::new();

    // Confirmed block on chain
    let confirmed = ProposalReq {
        leaves: vec![],
        target: 1,
        last_leaf: 0,
    };

    // New relay call
    let req = ProposalReq {
        leaves: vec![confirmed.target],
        target: 10,
        last_leaf: 9,
    };

    // Verify MMR
    let p = MerkleProof::<[u8; 32], MergeHash>::new(
        cmmr::leaf_index_to_mmr_size(req.last_leaf),
        req.mmr_proof(&store)
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
            EthHeader::get(&client, *l).await.unwrap().hash.unwrap(),
        ));
    }

    // Should pass verification
    assert!(p
        .verify(H256::from(&req.mmr_root(&store)), leaves)
        .unwrap_or(false));
}
