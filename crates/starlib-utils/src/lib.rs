//! StarLib Utilities

mod logger;

pub mod error;

pub mod pathlib;
pub mod version;

pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::trace;
pub use log::warn;

pub use self::logger::Logger;
