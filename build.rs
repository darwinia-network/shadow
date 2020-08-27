use std::{env, process::Command};

fn main() {
    // pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // main
    let os = Command::new("uname").output().unwrap();
    let ext = match String::from_utf8_lossy(os.stdout.as_slice())
        .into_owned()
        .trim_end()
        .as_ref()
    {
        "Darwin" => "dylib",
        _ => "so",
    };

    let out_dir = env::var_os("OUT_DIR")
        .unwrap()
        .to_string_lossy()
        .to_string();
    let lib = format!("{}/libeth.{}", out_dir, ext);
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &lib,
            "-buildmode=c-shared",
            "pkg/shadow/ffi/mod.go",
        ])
        .status()
        .unwrap();

    // post-check
    println!("cargo:rustc-link-lib=eth");
    println!("cargo:rustc-link-search={}", out_dir);
}
