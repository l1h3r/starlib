use log::Level;
use log::LevelFilter;
use log::Log;
use log::Metadata;
use log::Record;
use std::fmt::Arguments;

pub struct Logger;

impl Logger {
  pub fn init_debug() -> bool {
    Self::init(LevelFilter::Debug)
  }

  pub fn init_trace() -> bool {
    Self::init(LevelFilter::Trace)
  }

  fn init(max_level: LevelFilter) -> bool {
    log::set_logger(&Self)
      .map(|()| log::set_max_level(max_level))
      .is_ok()
  }
}

impl Log for Logger {
  #[inline]
  fn enabled(&self, _metadata: &Metadata) -> bool {
    true
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      let kind: Level = record.level();
      let name: &str = record.module_path().unwrap_or("::");
      let data: &Arguments<'_> = record.args();

      match record.line() {
        Some(line) => {
          println!("[{kind}][{name}:{line}]: {data}");
        }
        None => {
          println!("[{kind}][{name}]: {data}");
        }
      }
    }
  }

  #[inline]
  fn flush(&self) {
    // do nothing
  }
}
