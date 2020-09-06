use std::{env, process::Command};

fn main() {
    // Pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=pkg/shadow/eth/receipt.go");
    println!("cargo:rerun-if-changed=pkg/shadow/eth/proof.go");
    println!("cargo:rerun-if-changed=pkg/shadow/ffi/mod.go");

    // Declare build args
    let mut dynamic = match env::var("LIBRARY_TYPE") {
        Ok(ty) => ty.to_lowercase() != "static",
        Err(_) => true,
    };
    let out_dir = env::var("OUT_DIR").unwrap();
    go(&mut dynamic, &out_dir);

    // Post-check
    if dynamic {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=dylib=darwinia_shadow");
    } else {
        println!("cargo:rustc-link-search={}", out_dir);
        println!("cargo:rustc-link-lib=static=darwinia_shadow");
    }
}

/// Build golang library
fn go(dynamic: &mut bool, out_dir: &str) {
    if *dynamic {
        if !Command::new("go")
            .args(&gorgs(dynamic, out_dir))
            .status()
            .unwrap()
            .success()
        {
            *dynamic = false;
            go(dynamic, out_dir);
        }
    } else {
        Command::new("go")
            .args(&gorgs(dynamic, out_dir))
            .status()
            .unwrap();
        println!("built static library at {}", out_dir);
    }
}

fn gorgs(dynamic: &mut bool, out_dir: &str) -> Vec<String> {
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

    vec![
        "build",
        "-o",
        if *dynamic { &dylib } else { &staticlib },
        if *dynamic {
            "-buildmode=c-shared"
        } else {
            "-buildmode=c-archive"
        },
        "-v",
        "pkg/shadow/ffi/mod.go",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
