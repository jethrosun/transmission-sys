use super::*;

use std::ffi;
use std::mem;
use std::ptr;

#[test]
fn constant_access() {
    assert_eq!(INT8_MAX, 127);
}

#[test]
fn ctor_new_null() {
    unsafe {
        tr_ctorNew(ptr::null());
    }
}

#[test]
fn session_init() {
    let name = ffi::CStr::from_bytes_with_nul(b"testing\0").unwrap();
    unsafe {
        let settings: *mut tr_variant = mem::uninitialized();
        tr_variantInitDict(settings, 0);
        tr_sessionGetDefaultSettings(settings);
        let configDir = tr_getDefaultConfigDir(name.as_ptr());
        let session = tr_sessionInit(configDir, true, settings);

        tr_sessionClose(session);
        tr_variantFree(settings);
    }
}
