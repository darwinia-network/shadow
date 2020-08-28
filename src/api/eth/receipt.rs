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
///
/// ```
/// use darwinia_shadow::api::eth;
/// use actix_web::web;
///
/// // GET `/eth/receipt/0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a`
/// eth::receipt(web::Path::from(
///     "0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a".to_string(),
/// ));
/// ```
pub async fn handle(tx: web::Path<String>) -> impl Responder {
    web::Json(Into::<ReceiptResp>::into(super::ffi::receipt(
        tx.to_string(),
    )))
}
