use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("./vendor/assist/src")
        .canonicalize()
        .expect("Unable to find libdir");

    let headers_path = libdir_path.join("assist.h");
    let headers_path_str = headers_path
        .to_str()
        .expect("Path to headers is not valid UTF-8");

    println!("cargo:rerun-if-changed={}", headers_path_str);
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=assist");

    let rebound_include = match std::env::var_os("DEP_REBOUND_INCLUDE") {
        Some(include) => include.to_string_lossy().into_owned(),
        None => panic!("Unable to find rebound include path"),
    };

    let mut build_cfg = cc::Build::new();

    build_cfg
        .files(vec![
            "vendor/assist/src/assist.c",
            "vendor/assist/src/forces.c",
            "vendor/assist/src/planets.c",
            "vendor/assist/src/spk.c",
        ])
        .include(rebound_include.as_str())
        .include("vendor/assist/src")
        .warnings(false)
        .compile("assist");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .clang_arg("-I".to_string() + rebound_include.as_str())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_recursively(false)
	.allowlist_var("(assist|ASSIST)_.*")
	.allowlist_type("(assist|ASSIST)_.*")
	.allowlist_type("(spk|jpl)_s")
	.allowlist_function("(assist|ASSIST)_.*")
	.raw_line("use rebound_sys::*;")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("writing bindings to {}", out_path.to_str().unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
