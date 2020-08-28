use std::{env, process::Command};

fn main() {
    // pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // Get build paths
    let out_dir =
        env::var("DARWINIA_SHADOW_LIBRARY").unwrap_or_else(|_| env::var("OUT_DIR").unwrap());
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

    // Build the dynamic library
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

    // Load dynamic libdarwinia_shadow.so in common linux
    if ext.contains("so") && Command::new("ldconfig").args(&[&out_dir]).status().is_err() {
        Command::new("sudo")
            .args(&["ldconfig", &out_dir])
            .status()
            .unwrap();
    }

    // post-check
    println!("cargo:rustc-link-lib=darwinia_shadow");
    println!("cargo:rustc-link-search={}", out_dir);
}
