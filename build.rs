use std::env;
use std::error::Error;
use std::path::PathBuf;

/// builds hunspell in the `vendor` git submodule with the
/// `cc` crate: ignore any hunspell's build-scripts and
/// just compile the source code to a static lib.
///
/// Note: list of *.cxx files might need to be updated,
/// if `vendor` git submodule is updated
#[cfg(feature = "bundled")]
fn build_or_find_hunspell() -> Result<bindgen::Builder, Box<dyn Error>> {
    let target = std::env::var("TARGET").unwrap();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    let libcpp = match target_os.as_str() {
        "macos" => Some("dylib=c++"),
        "linux" => Some("dylib=stdc++"),
        _ => None,
    };

    if let Some(link) = libcpp {
        println!("cargo:rustc-link-lib={}", link);
    }

    println!("cargo:rustc-link-lib=static=hunspell-1.7");

    let mut build = cc::Build::new();
    let mut bind = bindgen::Builder::default().clang_arg(format!("-I{}", "vendor/src"));
    build.define("BUILDING_LIBHUNSPELL", "1").cpp(true);

    for entry in std::fs::read_dir("vendor/src/hunspell").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "cxx" {
                build.file(path);
            }
        }
    }

    if target_os == "wasi" {
        println!("cargo:rerun-if-env-changed=WASI_SDK_PATH");
        let wasi_sdk = env::var("WASI_SDK_PATH").expect("WASI_SDK_PATH not set");
        let libdir = format!("{}/share/wasi-sysroot/lib/wasm32-wasi", wasi_sdk);
        let bin = PathBuf::from(wasi_sdk).join("bin");
        build
            .define("_WASI_EMULATED_PROCESS_CLOCKS", "1")
            .compiler(bin.join("clang++"))
            .archiver(bin.join("ar"));
        bind = bind.clang_arg("--target=x86_64-unknown-linux-gnu");
        println!("cargo:rustc-link-lib=wasi-emulated-process-clocks");

        println!("cargo:rustc-link-search=native={}", libdir);
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
        println!("cargo:rerun-if-env-changed=CXX_{}", target);
        println!("cargo:rerun-if-env-changed=CC_{}", target);
    }

    build.compile("hunspell-1.7");
    Ok(bind)
}

#[cfg(not(feature = "bundled"))]
fn build_or_find_hunspell() -> Result<bindgen::Builder, Box<dyn Error>> {
    pkg_config::Config::new()
        .atleast_version("1.0.0")
        .statik(cfg!(feature = "static"))
        .probe("hunspell")?;

    Ok(bindgen::Builder::default())
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = build_or_find_hunspell()?;

    let bindings = builder
        .header("vendor/src/hunspell/hunspell.h")
        .generate()
        .expect("could not generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
