//! Ethereum ffi bindgen
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void, c_int},
    fmt,
    slice,
};
use std::convert::TryInto;

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

#[repr(C)]
struct GoTuple {
    index: *const c_char,
    proof: *const c_char,
    header_hash: *const c_char,
}

extern "C" fn geth_handler(x: *const c_char, size: c_int, arg: *mut c_void) -> bool {
    unsafe {
        let receiver: &mut &mut dyn FnMut(Vec<[u8; 32]>) -> bool =  &mut *(arg as *mut &mut dyn FnMut(Vec<[u8; 32]>) -> bool);

        let mut results : Vec<[u8; 32]> = Vec::new();
        let buf: &[u8] = slice::from_raw_parts(x as *mut u8, size as usize);
        for i in 0..(size >> 5) {
            let start = (i << 5) as usize;
            let result: [u8; 32] = buf[start..start+32].try_into().expect("split header hash");
            results.push(result);
        }
        receiver(results)
    }
}

#[link(name = "darwinia_shadow")]
extern "C" {
    fn Import(path: GoString, from: libc::c_int, to: libc::c_int, batch: libc::c_int, f: Option<extern "C" fn(x: *const c_char, len: c_int, arg: *mut c_void) -> bool>, arg: *mut c_void) -> bool;
    fn Proof(api: GoString, number: libc::c_uint) -> *const c_char;
    fn Receipt(api: GoString, tx: GoString) -> GoTuple;
    fn Epoch(input: libc::c_uint) -> bool;
    fn Free(pointer: *const c_char);
    fn Start(input: libc::c_uint);
    fn Stop();
}

struct WrapperCString {
    data: *const c_char,
}

impl WrapperCString {
    pub fn new(data: *const c_char) -> WrapperCString {
        WrapperCString {
            data,
        }
    }
}

impl fmt::Display for WrapperCString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            CStr::from_ptr(self.data).to_string_lossy().fmt(f)
        }
    }
}

impl Drop for WrapperCString {
    fn drop(&mut self) {
        unsafe {
            Free(self.data);
        }
    }
}

/// Proof eth header by number
pub fn proof(api: &str, block: u64) -> String {
    let c_api = CString::new(api).expect("CString::new failed");
    unsafe {
        WrapperCString::new(
            Proof(
                GoString {
                    a: c_api.as_ptr(),
                    b: c_api.as_bytes().len() as i64,
                },
                block as u32,
                )
            ).to_string()
    }
}

/// Proof eth header by number
pub fn epoch(block: u64) -> bool {
    unsafe { Epoch(block as u32) }
}

/// Start ethproof with block
pub fn start(epoch: u64) {
    unsafe { Start(epoch as u32) }
}

/// Stop ethproof
pub fn stop() {
    unsafe { Stop(); }
}

/// Get receipt by tx hash
pub fn receipt(api: &str, tx: &str) -> (String, String, String) {
    let c_api = CString::new(api).expect("CString::new failed");
    let c_tx = CString::new(tx).expect("CString::new failed");
    unsafe {
        let receipt = Receipt(
            GoString {
                a: c_api.as_ptr(),
                b: c_api.as_bytes().len() as i64,
            },
            GoString {
                a: c_tx.as_ptr(),
                b: c_tx.as_bytes().len() as i64,
            },
        );

        (
            WrapperCString::new(receipt.index).to_string(),
            WrapperCString::new(receipt.proof).to_string(),
            WrapperCString::new(receipt.header_hash).to_string(),
        )
    }
}

/// import from geth
pub fn import<F>(path: &str, from: i32, to: i32, batch: i32, mut callback: F) -> bool
    where F: FnMut(Vec<[u8; 32]>) -> bool
{
    // reason for double indirection is that a "Trait Object" is a fat pointer, the size of
    // which is incompatible with the C pointer *mut void
    let mut cb: &mut dyn FnMut(Vec<[u8; 32]>) -> bool = &mut callback;
    let cb = &mut cb;
    let c_path = CString::new(path).expect("CString::new failed");
    unsafe { 
        Import(
            GoString {
                a: c_path.as_ptr(),
                b: c_path.as_bytes().len() as i64,
            },
            from,
            to,
            batch,
            Some(geth_handler),
            cb as *mut _ as *mut c_void
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_proof() {
        super::start(0);
        super::proof(
            "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016",
            1,
        );
        super::stop();
    }

    #[test]
    fn test_receipt() {
        super::receipt(
            "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016",
            "0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a",
        );
    }
}
