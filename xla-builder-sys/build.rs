use std::env;
use std::path::PathBuf;

#[derive(Clone, Copy, Eq, PartialEq)]
enum OS {
    Linux,
    #[allow(clippy::enum_variant_names)]
    MacOS,
    Windows,
}

impl OS {
    fn get() -> Self {
        let os = env::var("CARGO_CFG_TARGET_OS").expect("Unable to get TARGET_OS");
        match os.as_str() {
            "linux" => Self::Linux,
            "macos" => Self::MacOS,
            "windows" => Self::Windows,
            os => panic!("Unsupported system {os}"),
        }
    }
}

fn main() {
    let os = OS::get();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let rxla_dir = PathBuf::from(env::var("RXLA_DIR").unwrap_or(String::from("bazel-bin/rxla")));
    let include = rxla_dir.join("include");
    let lib = rxla_dir.join("lib");

    println!("cargo:rerun-if-changed={}", include.display());

    // 1) TODO: build librxla.so using bazel
    println!("cargo:rustc-link-search=native={}", lib.display());
    println!("cargo:rustc-link-lib=rxla");
    if os == OS::MacOS {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib.display());
    } else {
        println!("cargo:rustc-link-arg=-Wl,-rpath={}", lib.display());
    }

    // 2) gen bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("unable to write bindings!");
}
