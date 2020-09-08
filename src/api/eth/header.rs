use crate::{
    chain::eth::{EthHeader, EthHeaderJson},
    mmr::{MergeHash, H256},
    ShadowShared,
};
use actix_web::{web, Responder};
use cmmr::MMR;
use std::env;

#[derive(Serialize)]
struct ProofResp {
    header: EthHeaderJson,
    mmr_root: String,
}

#[derive(Serialize)]
struct Resp {
	proof: ProofResp,
	confirmations: u64,
}

/// Proof target header
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::eth, ShadowShared};
///
/// // GET `/eth/header/19`
/// eth::header(web::Path::from("19".to_string()), web::Data::new(ShadowShared::new(None)));
/// ```
pub async fn handle(block: web::Path<String>, shared: web::Data<ShadowShared>) -> impl Responder {
    let num: u64 = block.to_string().parse().unwrap_or(0);
    let root = if num == 0 {
        "0000000000000000000000000000000000000000000000000000000000000000".to_string()
    } else {
        H256::hex(
            &MMR::<_, MergeHash, _>::new(cmmr::leaf_index_to_mmr_size(num - 1), &shared.store)
                .get_root()
                .unwrap_or_default(),
        )
    };
    
	let proof = ProofResp {
		header: EthHeader::get(&shared.client, num)
		.await
		.unwrap_or_default()
		.into(),
		mmr_root: format!("0x{}", root),
	};

	let confirmations = get_confirmations(num).await.unwrap_or(0);

	web::Json(Resp {
		proof: proof,
		confirmations: confirmations
	})
}

pub async fn get_confirmations(block_number: u64) -> web3::Result<u64> {

	let rpc_url = env::var("ETHEREUM_RPC").unwrap_or_else(|_| {
		if env::var("ETHEREUM_ROPSTEN").is_ok() {
			crate::conf::DEFAULT_ETHEREUM_ROPSTEN_RPC.into()
		} else {
			crate::conf::DEFAULT_ETHEREUM_RPC.into()
		}
	});

	let transport = web3::transports::Http::new(&rpc_url)?;
	let web3 = web3::Web3::new(transport);

	let current_height = web3.eth().block_number().await?;

	Ok(current_height.as_u64() - block_number)
}
