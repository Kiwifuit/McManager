use std::ffi::{c_char, CString};

use log::{set_logger, set_max_level, Level};

type LogConsumer = extern "C" fn(*const c_char);

static mut CONSUMER: Option<LogConsumer> = None;
static LOGGER: FfiLogger = FfiLogger;

#[no_mangle]
pub extern "C" fn mcs_register_log_consumer(callback: LogConsumer) {
    unsafe { CONSUMER = Some(callback) };
}

#[no_mangle]
pub extern "C" fn mcs_init_logger() {
    set_logger(&LOGGER).unwrap();
    set_max_level(log::LevelFilter::Info);
}

struct FfiLogger;

impl log::Log for FfiLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        if let Some(cb) = unsafe { CONSUMER } {
            let msg = "test".to_string();
            let msg_to_pass = CString::new(msg).unwrap();

            cb(msg_to_pass.as_ptr());
        }
    }

    fn flush(&self) {}
}
