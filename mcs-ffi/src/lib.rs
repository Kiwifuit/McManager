use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn libmcs_version() -> *const c_char {
    CString::new(format!("mcs backend v{}", env!("CARGO_PKG_VERSION")))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn libmcs_sha() -> *const c_char {
    CString::new(env!("GIT_SHA_LONG")).unwrap().into_raw()
}
