use std::{env, process::Command};

fn main() {
    // Pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // Declare build args
    let staticlib = format!("{}/libdarwinia_shadow.a", env::var("OUT_DIR").unwrap());
    let dylib = format!(
        "/usr/local/lib/libdarwinia_shadow.{}",
        match String::from_utf8_lossy(Command::new("uname").output().unwrap().stdout.as_slice())
            .into_owned()
            .trim_end()
            .as_ref()
        {
            "Darwin" => "dylib",
            _ => "so",
        }
    );

    let args = |dynamic: bool| {
        vec![
            "build",
            "-o",
            if dynamic { &dylib } else { &staticlib },
            if dynamic {
                "-buildmode=c-shared"
            } else {
                "-buildmode=c-archive"
            },
            "-v",
            "pkg/shadow/ffi/mod.go",
        ]
    };

    // Build the link library
    if !Command::new("go")
        .args(&args(true))
        .status()
        .unwrap()
        .success()
    {
        Command::new("go").args(&args(false)).status().unwrap();
    }

    // Post-check
    println!("cargo:rustc-link-lib=darwinia_shadow");
}
