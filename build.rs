use std::{env, process::Command};

fn main() {
    // pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // build libdarwinia_shadow
    let os = Command::new("uname").output().unwrap();
    let ext = match String::from_utf8_lossy(os.stdout.as_slice())
        .into_owned()
        .trim_end()
        .as_ref()
    {
        "Darwin" => "dylib",
        _ => "so",
    };

    // get out dir
    let out_dir = env::var_os("OUT_DIR")
        .unwrap()
        .to_string_lossy()
        .to_string();

    // build libraries
    vec![("a", "archive"), (ext, "shared")]
        .iter()
        .for_each(|lib| {
            Command::new("go")
                .args(&[
                    "build",
                    "-o",
                    &format!("{}/libdarwinia_shadow.{}", out_dir, lib.0),
                    &format!("-buildmode=c-{}", lib.1),
                    "pkg/shadow/ffi/mod.go",
                ])
                .status()
                .unwrap();
        });

    // post-check
    println!("cargo:rustc-link-lib=static=darwinia_shadow");
    println!("cargo:rustc-link-search={}", out_dir);
}
