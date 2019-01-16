use super::*;

use std::ffi;
use std::ptr;

#[test]
fn constant_access() {
    assert_eq!(TR_TRANSMISSION_H, 1);
}

#[test]
fn session_init() {
    let settings = ptr::null_mut();
    let name = ffi::CStr::from_bytes_with_nul("testing".as_bytes()).unwrap();
    unsafe {
        tr_variantInitDict(settings, 0);
        tr_sessionGetDefaultSettings(settings);
        let configDir = tr_getDefaultConfigDir(name.as_ptr());
        let session = tr_sessionInit(configDir, 1, settings);

        tr_sessionClose(session);
        tr_variantFree(settings);
    }
}
