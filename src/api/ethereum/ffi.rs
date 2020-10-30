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

impl From<&str> for GoString {
    fn from(s: &str) -> GoString {
        let c_tx = CString::new(s).expect("CString::new failed");
        GoString {
            a: c_tx.as_ptr(),
            b: c_tx.as_bytes().len() as i64,
        }
    }
}

#[repr(C)]
struct GoTuple {
    index: *const c_char,
    proof: *const c_char,
    header_hash: *const c_char,
}

#[link(name = "darwinia_shadow")]
extern "C" {
    fn Import(path: GoString, from: libc::c_int, to: libc::c_int) -> *const c_char;
    fn Proof(api: GoString, number: libc::c_uint) -> *const c_char;
    fn Receipt(api: GoString, tx: GoString) -> GoTuple;
    fn Epoch(input: libc::c_uint) -> bool;
}

/// Proof eth header by number
pub fn proof(api: &str, block: u64) -> String {
    let api = GoString::from(api);
    unsafe {
        CStr::from_ptr(Proof(GoString::from(api), block as u32))
            .to_string_lossy()
            .to_string()
    }
}

/// Proof eth header by number
pub fn epoch(block: u64) -> bool {
    unsafe { Epoch(block as u32) }
}

/// Get receipt by tx hash
pub fn receipt(api: &str, tx: &str) -> (String, String, String) {
    unsafe {
        let receipt = Receipt(GoString::from(api), GoString::from(tx));

        (
            CStr::from_ptr(receipt.index).to_string_lossy().to_string(),
            CStr::from_ptr(receipt.proof).to_string_lossy().to_string(),
            CStr::from_ptr(receipt.header_hash)
                .to_string_lossy()
                .to_string(),
        )
    }
}

/// Get receipt by tx hash
pub fn import(path: &str, from: i32, to: i32) -> String {
    unsafe {
        let c_path = CString::new(path).expect("CString::new failed");
        CStr::from_ptr(Import(
            GoString {
                a: c_path.as_ptr(),
                b: c_path.as_bytes().len() as i64,
            },
            from,
            to,
        ))
        .to_string_lossy()
        .to_string()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_proof() {
        super::proof(
            "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016",
            1,
        );
    }

    #[test]
    fn test_receipt() {
        super::receipt(
            "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016",
            "0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a",
        );
    }
}
