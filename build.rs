use std::{env, process::Command};

fn main() {
    // Pre-check
    println!("cargo:rerun-if-changed=build.rs");

    // Declare build args
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("go")
        .args(&vec![
            "build",
            "-o",
            &format!("{}/libdarwinia_shadow.a", out_dir),
            "-buildmode=c-archive",
            "-v",
            "pkg/shadow/ffi/mod.go",
        ])
        .status()
        .unwrap();

    // Post-check
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=darwinia_shadow");
}
