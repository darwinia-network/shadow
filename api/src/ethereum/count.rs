use actix_web::{web::Data, Responder};
use mmr::{build_client, Database};
use crate::{Result, AppData};
use actix_web::web::Json;
use serde::Serialize;

/// Count result
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CountResult {
    Count { count: String },
    Error { error: String }
}

/// Count the mmr of ethereum headers
pub async fn handle(app_data: Data<AppData>) -> impl Responder {
    match count(&app_data.mmr_db) {
        Ok(count) => Json(CountResult::Count { count }),
        Err(err) => Json(CountResult::Error { error: err.to_string() })
    }
}

fn count(mmr_db: &Database) -> Result<String> {
    let client = build_client(mmr_db)?;
    Ok(format!("{}", client.get_leaf_count()?))
}
