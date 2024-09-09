use std::{
    borrow::Cow,
    ffi::{c_char, CStr},
};

use denji::{MinecraftServer, ServerSoftware};
use log::info;

type FFIString = *const c_char;

#[repr(C)]
pub struct ServerEntry {
    path: FFIString,
    eula: bool,
    software: ServerEntrySoftware,
}

#[repr(C)]
pub struct ServerEntrySoftware {
    server: FFIString,
    server_version: FFIString,
    loader_version: FFIString,
    modpack: FFIString,
}

#[no_mangle]
#[expect(clippy::not_unsafe_ptr_arg_deref)]
pub fn build_server(
    name: FFIString,
    server_version: FFIString,
    loader: FFIString,
    loader_version: FFIString,
) -> *const ServerEntry {
    // `l*` variables are local `Cow<'_, str>`
    // strings constructed by `get_string()`
    let l_name = unsafe { get_string(name) };
    // let l_server_version = unsafe { get_string(server_version) };
    // let l_loader = unsafe { get_string(loader) };
    // let l_loader_version = unsafe { get_string(loader_version) };

    info!("Building server {}", l_name);
    // let server = MinecraftServer::new(
    //     ServerSoftware::from(l_loader),
    //     l_loader_version,
    //     l_server_version,
    //     "./",
    // );

    Box::into_raw(Box::new(ServerEntry {
        path: std::ptr::null(),
        eula: false,
        software: ServerEntrySoftware {
            server: loader,
            server_version,
            loader_version,
            modpack: std::ptr::null(),
        },
    }))
}

unsafe fn get_string<'a>(raw: FFIString) -> Cow<'a, str> {
    String::from_utf8_lossy(unsafe { CStr::from_ptr(raw) }.to_bytes())
}
