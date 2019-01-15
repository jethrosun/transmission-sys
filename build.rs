use std::env;
use std::path::PathBuf;

use bindgen;

fn main() {
    println!("cargo:rustc-link-lib=transmission"); 

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
