use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn dms_version() -> *const c_char {
    CString::new(format!(
        "dms backend v{} ({}, {})",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_SHA_SHORT"),
        env!("CRATE_FEATURES")
    ))
    .unwrap()
    .into_raw()
}
