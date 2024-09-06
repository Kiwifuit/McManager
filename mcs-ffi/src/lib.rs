use log::info;

pub mod logging;
pub mod modrinth;
pub mod versioning;

#[no_mangle]
pub extern "C" fn test() {
    info!("hi");
    info!("hello");
    info!("i am burger");
    info!("yippeeee");
}
