use std::process::Command;

fn main() {
    let os = Command::new("uname").output().unwrap();
    let ext = match String::from_utf8_lossy(os.stdout.as_slice())
        .into_owned()
        .trim_end()
        .as_ref()
    {
        "Darwin" => "dylib",
        _ => "so",
    };

    let profile = match std::env::var("PROFILE").unwrap().as_str() {
        "release" => "release",
        _ => "debug",
    };

    let lib = format!("target/{}/libeth.{}", profile, ext);
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &lib,
            "-buildmode=c-shared",
            "internal/ffi/mod.go",
        ])
        .status()
        .unwrap();

    println!(r"cargo:rustc-link-search=target/debug");
}
