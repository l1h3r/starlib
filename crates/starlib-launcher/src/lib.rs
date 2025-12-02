//! StarLib Process Launcher

#[macro_use]
extern crate starlib_utils;

mod builder;
mod process;

pub use self::builder::Builder;
pub use self::builder::DisplayMode;
pub use self::builder::Renderer;

pub use self::process::DropMode;
pub use self::process::Process;
pub use self::process::ProcessError;
pub use self::process::ProcessErrorKind;
