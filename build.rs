use std::process::Command;

fn main() {
    // pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // build libdarwinia_shadow
    let ext =
        match String::from_utf8_lossy(Command::new("uname").output().unwrap().stdout.as_slice())
            .into_owned()
            .trim_end()
            .as_ref()
        {
            "Darwin" => "dylib",
            _ => "so",
        };
    let lib = format!("/usr/local/lib/libdarwinia_shadow.{}", ext);
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
    println!("cargo:rustc-link-search=/usr/local/lib");
}
