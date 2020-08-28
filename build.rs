use std::{env, process::Command};

fn main() {
    // pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // build libdarwinia_shadow
    let out_dir = env::var_os("OUT_DIR")
        .unwrap()
        .to_string_lossy()
        .to_string();

    // env::set_var("CGO_ENABLED", "0");
    // env::set_var("GO111MODULE", "on");
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("{}/libdarwinia_shadow.a", out_dir),
            "-buildmode=c-archive",
            "-v",
            "pkg/shadow/ffi/mod.go",
        ])
        .status()
        .unwrap();

    // post-check
    println!("cargo:rustc-link-lib=static=darwinia_shadow");
    println!("cargo:rustc-link-search={}", out_dir);
}
