//! Ethereum ffi bindgen
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

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

#[link(name = "darwinia_shadow")]
extern "C" {
    fn Proof(input: libc::c_uint) -> *const c_char;
    fn Receipt(input: GoString) -> GoTuple;
}

/// Proof eth header by number
pub fn proof(block: u64) -> String {
    unsafe {
        CStr::from_ptr(Proof(block as u32))
            .to_string_lossy()
            .to_string()
    }
}

/// Get receipt by tx hash
pub fn receipt(tx: String) -> (String, String) {
    unsafe {
        let c_tx = CString::new(tx).expect("CString::new failed");
        let receipt = Receipt(GoString {
            a: c_tx.as_ptr(),
            b: c_tx.as_bytes().len() as i64,
        });

        (
            CStr::from_ptr(receipt.proof).to_string_lossy().to_string(),
            CStr::from_ptr(receipt.hash).to_string_lossy().to_string(),
        )
    }
}
