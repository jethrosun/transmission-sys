# transmission-sys
Rust bindings for [Transmission](https://transmissionbt.com/) using [Rust-Bindgen](https://github.com/rust-lang/rust-bindgen).

## Building

First make sure you have all the dependencies listed below then run

`git submodule update --init --recursive`

`cargo build`

### Dependencies
- gcc (or Clang)
- libclang-devel
- libopenssl-devel
- libcurl-devel
- libevent-devel
- autotools (automake, autoconf, libtool)
