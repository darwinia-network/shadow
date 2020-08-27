use std::{env, process::Command};

fn main() {
    // pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // build libeth.a
    let out_dir = env::var_os("OUT_DIR")
        .unwrap()
        .to_string_lossy()
        .to_string();
    let lib = format!("{}/libeth.a", out_dir);
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &lib,
            "-buildmode=c-archive",
            "pkg/shadow/ffi/mod.go",
        ])
        .status()
        .unwrap();

    // post-check
    println!("cargo:rustc-link-lib=eth");
    println!("cargo:rustc-link-search={}", out_dir);
}
