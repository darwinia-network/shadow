use std::process::Command;

fn main() {
    // Pre-check
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    // Declare build args
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
    let args = vec![
        "build",
        "-o",
        &lib,
        "-buildmode=c-shared",
        "-v",
        "pkg/shadow/ffi/mod.go",
    ];

    // Build the dynamic library
    if !Command::new("go").args(&args).status().unwrap().success()
        && !Command::new("sudo")
            .args(vec![vec!["go"], args].concat())
            .status()
            .unwrap()
            .success()
    {
        panic!(
            "{}{}",
            "It seems we don't have permission to create our library at `/usr/local/lib`",
            ", please get the permission and install darwinia-shadow again~"
        );
    }

    if ext.contains("so")
        && !Command::new("ldconfig")
            .arg(&lib)
            .status()
            .unwrap()
            .success()
        && !Command::new("sudo")
            .args(&["ldconfig", &lib])
            .status()
            .unwrap()
            .success()
    {
        panic!("Could update LD_LIBRARY_PATH");
    }

    // Post-check
    println!("cargo:rustc-link-lib=darwinia_shadow");
}
