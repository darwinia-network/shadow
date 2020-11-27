use cmmr::MerkleProof;
use darwinia_shadow::{
    api::ethereum::{helper, ProposalReq},
    mmr::{helper as mmr_helper, MergeHash, Runner, H256},
    result::Error,
    ShadowShared,
};
use primitives::rpc::RPC;
use rocksdb::{IteratorMode, DB};

async fn stops_at(db: &DB, runner: &mut Runner, count: i64) -> Result<(), Error> {
    let mut mmr_size = db.iterator(IteratorMode::Start).count() as u64;
    let mut ptr = {
        let last_leaf = mmr_helper::mmr_size_to_last_leaf(mmr_size as i64);
        if last_leaf == 0 {
            0
        } else {
            last_leaf + 1
        }
    };

    loop {
        if ptr >= count {
            break;
        }
        if let Ok(mmr_size_new) = runner.push(ptr, mmr_size).await {
            mmr_size = mmr_size_new;
            ptr += 1;
        }
    }

    Ok(())
}

#[actix_rt::test]
async fn test_proposal() {
    let shared = ShadowShared::new(None);
    let mut runner = Runner::from(shared.clone());
    let rpc = &shared.eth;

    // Gen mmrs
    assert!(stops_at(&shared.db, &mut runner, 30).await.is_ok());

    // Confirmed block on chain
    let confirmed = ProposalReq {
        member: 0,
        target: 0,
        last_leaf: 0,
    };

    // New relay call - Round 0
    let req_r0 = ProposalReq {
        member: confirmed.target,
        target: 3,
        last_leaf: 2,
    };

    // Verify MMR
    let p_r0 = MerkleProof::<[u8; 32], MergeHash>::new(
        cmmr::leaf_index_to_mmr_size(req_r0.last_leaf),
        req_r0
            .mmr_proof(&shared.store)
            .into_iter()
            .map(|h| H256::from(&h))
            .collect(),
    );

    // Should pass verification
    assert!(p_r0
        .verify(
            H256::from(&helper::parent_mmr_root(req_r0.target, &shared).unwrap()),
            vec![(
                cmmr::leaf_index_to_pos(req_r0.member),
                rpc.get_header_by_number(req_r0.member)
                    .await
                    .unwrap()
                    .hash
                    .unwrap(),
            )]
        )
        .unwrap_or(false));

    // New Round 1
    let req_r1 = ProposalReq {
        member: 2,
        target: 2,
        last_leaf: 2,
    };

    // Verify MMR
    let p_r1 = MerkleProof::<[u8; 32], MergeHash>::new(
        cmmr::leaf_index_to_mmr_size(req_r1.last_leaf),
        req_r1
            .mmr_proof(&shared.store)
            .into_iter()
            .map(|h| H256::from(&h))
            .collect(),
    );

    // Should pass verification
    //
    // The the round 0's mmr_root to verify round 1's hash
    assert!(p_r1
        .verify(
            H256::from(&helper::parent_mmr_root(req_r0.target, &shared).unwrap()),
            vec![(
                cmmr::leaf_index_to_pos(req_r1.member),
                rpc.get_header_by_number(req_r1.member)
                    .await
                    .unwrap()
                    .hash
                    .unwrap(),
            )]
        )
        .unwrap_or(false));
}
