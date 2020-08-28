use std::{env, process::Command};

fn main() {
    // pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // build libdarwinia_shadow
    let out_dir = env::var("OUT_DIR").unwrap();
    let ext =
        match String::from_utf8_lossy(Command::new("uname").output().unwrap().stdout.as_slice())
            .into_owned()
            .trim_end()
            .as_ref()
        {
            "Darwin" => "dylib",
            _ => "so",
        };
    let lib = format!("{}/libdarwinia_shadow.{}", out_dir, ext);
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &lib,
            "-buildmode=c-shared",
            "-v",
            "pkg/shadow/ffi/mod.go",
        ])
        .status()
        .unwrap();

    // link libdarwinia_shadow
    println!("cargo:rustc-link-lib=darwinia_shadow");
    println!("cargo:rustc-link-search={}", out_dir);
}
