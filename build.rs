use std::env;
use std::path::PathBuf;

use bindgen;
use cmake;
use pkg_config;

fn main() {
    // This builds and links the bundled libtransmission
    let dst = cmake::Config::new("transmission")
        // Turn everything we don't want off
        .define("ENABLE_DAEMON", "OFF")
        .define("ENABLE_GTK", "OFF")
        .define("ENABLE_QT", "OFF")
        .define("ENABLE_MAC", "OFF")
        .define("ENABLE_UTILS", "OFF")
        .define("ENABLE_CLI", "OFF")
        .define("ENABLE_TESTS", "OFF")
        .define("ENABLE_LIGHTWEIGHT", "OFF")
        .define("ENABLE_NLS", "OFF")
        .define("INSTALL_DOC", "OFF")
        .define("USE_SYSTEM_DHT", "OFF")
        .define("USE_SYSTEM_MINIUPNPC", "OFF")
        .define("USE_SYSTEM_NATPMP", "OFF")
        .define("USE_SYSTEM_UTP", "OFF")
        .define("USE_SYSTEM_B64", "OFF")
        .define("WITH_INOTIFY", "OFF")
        .define("WITH_KQUEUE", "OFF")
        .define("WITH_LIBAPPINDICATOR", "OFF")
        .define("WITH_SYSTEMD", "OFF")
        // Turn a few things on
        .define("ENABLE_UTP", "ON")
        .define("INSTALL_LIB", "ON")
        .define("WITH_CRYPTO", "openssl")
        // This is until Transmission fixes it
        .define("USE_SYSTEM_EVENT2", "ON")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib64").display());
    println!("cargo:rustc-link-lib=static=transmission");
    
    let dst_natpmp = cmake::Config::new("transmission/third-party/libnatpmp").build();
    
    println!("cargo:rustc-link-search=native={}", dst_natpmp.join("lib").display());
    println!("cargo:rustc-link-lib=static=natpmp");

    let dst_utp = cmake::Config::new("transmission/third-party/libutp").build();
    println!("cargo:rustc-link-search=native={}", dst_utp.join("lib").display());
    println!("cargo:rustc-link-lib=static=utp");

    let dst_miniupnpc = cmake::Config::new("transmission/third-party/miniupnpc").build();
    println!("cargo:rustc-link-search=native={}", dst_miniupnpc.join("lib").display());
    println!("cargo:rustc-link-lib=static=miniupnpc");

    let dst_dht = cmake::Config::new("transmission/third-party/dht").build();
    println!("cargo:rustc-link-search=native={}", dst_dht.join("lib").display());
    println!("cargo:rustc-link-lib=static=dht");

    let dst_b64 = cmake::Config::new("transmission/third-party/libb64").build();
    println!("cargo:rustc-link-search=native={}", dst_b64.join("lib").display());
    println!("cargo:rustc-link-lib=static=b64");

    // Link dependencies
    let st = cfg!(feature = "static");

    pkg_config::Config::new().statik(st).probe("libevent").unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // These are autogenerated so may fail for no reason and are disabled
        .layout_tests(false)
        // Workaround to fix documentation tests failing
        .generate_comments(false)
        // Make things pretty
        .rustfmt_bindings(true)
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
