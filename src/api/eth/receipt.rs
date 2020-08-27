use actix_web::{web, Responder};

#[derive(Serialize)]
struct ReceiptResp {
    proof: String,
    hash: String,
}

impl From<(String, String)> for ReceiptResp {
    fn from(t: (String, String)) -> ReceiptResp {
        ReceiptResp {
            proof: t.0,
            hash: t.1,
        }
    }
}

/// Receipt Handler
pub async fn handle(tx: web::Path<String>) -> impl Responder {
    web::Json(Into::<ReceiptResp>::into(super::ffi::receipt(
        tx.to_string(),
    )))
}
