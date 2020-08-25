use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

const TX: &str = "0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a";

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}

#[repr(C)]
struct GoTuple {
    proof: *const c_char,
    hash: *const c_char,
}

#[link(name = "eth")]
extern "C" {
    fn Receipt(input: GoString) -> GoTuple;
}

fn main() {
    let c_tx = CString::new(TX).expect("CString::new failed");
    let ptr = c_tx.as_ptr();
    let tx = GoString {
        a: ptr,
        b: c_tx.as_bytes().len() as i64,
    };

    unsafe {
        let gt = Receipt(tx);
        println!("{:?}", CStr::from_ptr(gt.proof));
        println!("{:?}", CStr::from_ptr(gt.hash));
    }
}
