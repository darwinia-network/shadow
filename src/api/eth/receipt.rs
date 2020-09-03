use actix_web::{web, Responder};

#[derive(Serialize)]
struct ReceiptResp {
    index: String,
    proof: String,
    header_hash: String,
}

impl From<(String, String, String)> for ReceiptResp {
    fn from(t: (String, String, String)) -> ReceiptResp {
        ReceiptResp {
            index: t.0,
            proof: t.1,
            header_hash: t.2,
        }
    }
}

/// Receipt Handler
///
/// ```
/// use actix_web::web;
/// use darwinia_shadow::{api::eth, ShadowShared};
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
