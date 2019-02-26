//! Rust bindings for [Transmission](https://transmissionbt.com/) made using [Rust-Bindgen](https://github.com/rust-lang/rust-bindgen).
//!
//! Bindings are generated with [Rust-Bindgen](https://github.com/rust-lang/rust-bindgen)
//! which means there are a number of quirks.
//!
//! - Enums are a constants in the form of `enum_name_ENUM_FIELD`
//! - Functions are named the same as the C code and don't follow Rust naming schemes.
//! - Uses C strings. See `CStr` in the Rust standard library.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate curl_sys;
extern crate libevent_sys;
extern crate libz_sys;
extern crate openssl_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test;
