use std::{ffi::CStr, os::raw::c_char};

#[link(name = "eth")]
extern "C" {
    fn Proof(input: libc::c_uint) -> *const c_char;
}

fn main() {
    println!("{:?}", unsafe { CStr::from_ptr(Proof(1)) });
}
