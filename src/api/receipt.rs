use actix_web::{web, Responder};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

#[derive(Serialize)]
struct ReceiptResp {
    proof: String,
    hash: String,
}

/// Receipt Handler
pub async fn handle(tx: web::Path<String>) -> impl Responder {
    unsafe {
        let resp: ReceiptResp = Receipt(GoString::from(tx.to_string())).into();
        web::Json(resp)
    }
}

// FFI
#[link(name = "eth")]
extern "C" {
    fn Receipt(input: GoString) -> GoTuple;
}

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

impl From<String> for GoString {
    fn from(s: String) -> GoString {
        let c_tx = CString::new(s).expect("CString::new failed");
        GoString {
            a: c_tx.as_ptr(),
            b: c_tx.as_bytes().len() as i64,
        }
    }
}

#[repr(C)]
struct GoTuple {
    proof: *const c_char,
    hash: *const c_char,
}

impl Into<ReceiptResp> for GoTuple {
    fn into(self) -> ReceiptResp {
        unsafe {
            ReceiptResp {
                proof: format!("{:?}", CStr::from_ptr(self.proof)),
                hash: format!("{:?}", CStr::from_ptr(self.hash)),
            }
        }
    }
}
