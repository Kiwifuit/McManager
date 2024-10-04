use chrono::Local;
use log::{set_logger, set_max_level, Level, Record};
use serde::Serialize;
use std::ffi::{c_char, CString};

type LogConsumer = extern "C" fn(*const c_char);

static mut CONSUMER: Option<LogConsumer> = None;
static LOGGER: FfiLogger = FfiLogger;

#[no_mangle]
pub extern "C" fn dms_register_logger(callback: LogConsumer) {
  unsafe { CONSUMER = Some(callback) };
}

#[no_mangle]
pub extern "C" fn dms_init_logger() {
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
      let log = LogRecord::from(record);
      let log_cs = CString::new(serde_json::to_string(&log).unwrap()).unwrap();

      cb(log_cs.as_ptr());
    }
  }

  fn flush(&self) {}
}

#[derive(Serialize)]
struct LogRecord<'a> {
  time: i64,
  module: &'a str,
  message: String,
  level: Level,
}

impl<'a> From<&Record<'a>> for LogRecord<'a> {
  fn from(value: &Record<'a>) -> Self {
    Self {
      time: Local::now().timestamp(),
      module: value.module_path().unwrap_or("<module>"),
      level: value.level(),
      message: format!("{}", value.args()),
    }
  }
}
