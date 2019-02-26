# transmission-sys
[![](https://meritbadge.herokuapp.com/transmission-sys)](https://crates.io/crates/transmission-sys)
[![Released API docs](https://docs.rs/transmission-sys/badge.svg)](https://docs.rs/transmission-sys)
[![pipeline status](https://gitlab.com/tornado-torrent/transmission-sys/badges/master/pipeline.svg)](https://gitlab.com/tornado-torrent/transmission-sys/commits/master)

Rust bindings for [Transmission](https://transmissionbt.com/) made using [Rust-Bindgen](https://github.com/rust-lang/rust-bindgen).

Created and maintained by the [Tornado Project](https://gitlab.com/tornado-torrent/)

**Note this package only supports Linux at this time.**

Support for MacOS and Windows is planned, but pull requests helping
are greatly appreciated.
Especially for MacOS because I do not have access to one.

## Building

First make sure you have all the dependencies listed below then run

`git submodule update --init --recursive`

`cargo build`

### Dependencies
- gcc (or Clang)
- cmake
- libclang-devel
- libopenssl-devel
- libcurl-devel
- libevent-devel

<!--
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
-->