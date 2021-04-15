use api::{ethereum::ProposalReq, Error};
use darwinia_shadow::mmr::database;
use mmr::{build_client, mmr_size_to_last_leaf, MergeHash, MerkleProof, MmrClientTrait, H256};
use primitives::rpc::{EthereumRPC, RPC};

use std::{env, sync::Arc};

async fn stops_at(
    rpc: &Arc<EthereumRPC>,
    client: &mut Box<dyn MmrClientTrait>,
    count: i64,
) -> Result<(), Error> {
    let mmr_size = client.get_mmr_size().unwrap();
    let mut ptr = {
        let last_leaf = mmr_size_to_last_leaf(mmr_size as i64);
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
        let hash_from_ethereum = rpc.get_header_by_number(ptr as u64).await?.hash;
        if let Some(hash) = hash_from_ethereum {
            client.push(&hash)?;
            ptr += 1;
        }
    }

    Ok(())
}

#[actix_rt::test]
async fn test_proposal() {
    use std::fs;
    let rpcs = vec![String::from(
        "https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016",
    )];
    let rpc = Arc::new(EthereumRPC::new(reqwest::Client::new(), rpcs));
    let dbpath = env::temp_dir().join("test_proposal.db");
    fs::remove_dir_all(&dbpath).unwrap_or_else(|err| {
        println!("{}", err);
    });
    let path = dbpath.to_str().unwrap().to_string();
    let mmr_db = database(Some(path)).unwrap();
    //let mut runner = Runner::new(&rpc, &mmr_db);
    let mut client = build_client(&mmr_db).unwrap();

    // Gen mmrs
    assert!(stops_at(&rpc, &mut client, 30).await.is_ok());

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
        mmr::leaf_index_to_mmr_size(req_r0.last_leaf),
        req_r0
            .mmr_proof(&mmr_db)
            .unwrap()
            .into_iter()
            .map(|h| H256::from(&h).unwrap())
            .collect(),
    );

    // Should pass verification
    assert!(p_r0
        .verify(
            H256::from(&client.get_mmr_root(req_r0.target - 1).unwrap().unwrap()).unwrap(),
            vec![(
                mmr::leaf_index_to_pos(req_r0.member),
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
        mmr::leaf_index_to_mmr_size(req_r1.last_leaf),
        req_r1
            .mmr_proof(&mmr_db)
            .unwrap()
            .into_iter()
            .map(|h| H256::from(&h).unwrap())
            .collect(),
    );

    // Should pass verification
    //
    // The the round 0's mmr_root to verify round 1's hash
    assert!(p_r1
        .verify(
            H256::from(&client.get_mmr_root(req_r0.target - 1).unwrap().unwrap()).unwrap(),
            vec![(
                mmr::leaf_index_to_pos(req_r1.member),
                rpc.get_header_by_number(req_r1.member)
                    .await
                    .unwrap()
                    .hash
                    .unwrap(),
            )]
        )
        .unwrap_or(false));

    fs::remove_dir_all(&dbpath).unwrap_or_else(|err| {
        println!("{}", err);
    });
}
