use std::ffi::{c_char, CString};

pub extern "C" fn libmcs_version() -> *const c_char {
    CString::new(format!("mcs backend v{}", env!("CARGO_PKG_VERSION")))
        .unwrap()
        .into_raw()
}
