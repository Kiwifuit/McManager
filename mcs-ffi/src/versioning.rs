use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn libmcs_version() -> *const c_char {
    CString::new(format!(
        "mcs backend v{} ● mcs {}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_SHA_LONG")
    ))
    .unwrap()
    .into_raw()
}
