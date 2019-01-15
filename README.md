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


## Manually Building
`transmission-sys` bundles in Transmision and compiles it for you.
If you would instead like to build and install `libtransmission` yourself
follow the steps in the Transmission readme but replace `cmake ..` with

```sh
cmake \
    -DENABLE_DAEMON=OFF \
    -DENABLE_GTK=OFF \
    -DENABLE_QT=OFF \
    -DENABLE_MAC=OFF \
    -DENABLE_UTILS=OFF \
    -DENABLE_CLI=OFF \
    -DENABLE_TESTS=OFF \
    -DENABLE_LIGHTWEIGHT=OFF \
    -DENABLE_UTP=ON \
    -DENABLE_NLS=OFF \
    -DINSTALL_DOC=OFF \
    -DINSTALL_LIB=ON \
    -DUSE_SYSTEM_EVENT2=ON \
    -DUSE_SYSTEM_DHT=OFF \
    -DUSE_SYSTEM_MINIUPNPC=OFF \
    -DUSE_SYSTEM_NATPMP=OFF \
    -DUSE_SYSTEM_UTP=OFF \
    -DUSE_SYSTEM_B64=OFF \
    -DWITH_CRYPTO=openssl \
    -DWITH_INOTIFY=OFF \
    -DWITH_KQUEUE=OFF \
    -DWITH_LIBAPPINDICATOR=OFF \
    -DWITH_SYSTEMD=OFF \
    ..
```
This will configure cmake to build libtransmission with the proper settings.

Then comment out the specified portions of `build.rs` and run `cargo build --lib` as normal.