use std::{env, process::Command};

fn main() {
    // Pre-check
    println!("cargo:rerun-if-changed=build.rs");

    // Declare build args
    let mut dynamic = true;
    let out_dir = env::var("OUT_DIR").unwrap();
    let staticlib = format!("{}/libdarwinia_shadow.a", out_dir);
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
        .args(&args(dynamic))
        .status()
        .unwrap()
        .success()
    {
        dynamic = false;
        Command::new("go").args(&args(dynamic)).status().unwrap();
        println!("built static library at {}", out_dir);
    }

    // Post-check
    if dynamic {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=dylib=darwinia_shadow");
    } else {
        println!("cargo:rustc-link-search={}", out_dir);
        println!("cargo:rustc-link-lib=static=darwinia_shadow");
    }
}
