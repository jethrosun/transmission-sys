use std::env;
use std::path::PathBuf;

use cmake;
use bindgen;

fn main() {
    let dst = cmake::Config::new("vendor")
        .define("ENABLE_DAEMON", "OFF")
        .define("ENABLE_GTK", "OFF")
        .define("ENABLE_QT", "OFF")
        .define("ENABLE_MAC", "OFF")
        .define("ENABLE_UTILS", "OFF")
        .define("ENABLE_CLI", "OFF")
        .define("ENABLE_TESTS", "OFF")
        .define("INSTALL_DOC", "OFF")
        .no_build_target(true)
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=transmission");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
