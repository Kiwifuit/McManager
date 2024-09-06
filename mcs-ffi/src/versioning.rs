use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn mcs_version() -> *const c_char {
    CString::new(format!(
        "mcs backend v{} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_SHA_SHORT")
    ))
    .unwrap()
    .into_raw()
}
