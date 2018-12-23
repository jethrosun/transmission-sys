# transmission-sys
Rust bindings for [Transmission]() using [Rust-Bindgen]()

## Building

First make sure you have all the dependencies listed below then run

`git submodule update --init --recursive`
`cargo build`

### Dependencies
- cmake
- gcc
- libclang
- openssl
- curl
- autotools (automake, autoconf, libtool)
