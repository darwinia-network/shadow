use crate::{
    chain::eth::{
	    EthHeader, EthHeaderJson, get_confirmations},
    mmr::{MergeHash, H256},
    ShadowShared,
};
use actix_web::{web, Responder};
use cmmr::MMR;

#[derive(Serialize)]
struct HeaderThing {
    header: EthHeaderJson,
    mmr_root: String,
}

#[derive(Serialize)]
struct Resp {
	header_thing: HeaderThing,
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
    
	let header_thing = HeaderThing {
		header: EthHeader::get(&shared.client, num)
		.await
		.unwrap_or_default()
		.into(),
		mmr_root: format!("0x{}", root),
	};

	let confirmations = get_confirmations(&shared.client, num).await.unwrap_or(0);

	web::Json(Resp {
		header_thing: header_thing,
		confirmations: confirmations
	})
}
