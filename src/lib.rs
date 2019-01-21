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
