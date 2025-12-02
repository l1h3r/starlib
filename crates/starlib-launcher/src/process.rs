use std::io::Error;
use std::net::IpAddr;
use std::process::Child;

use crate::builder::Builder;

// -----------------------------------------------------------------------------
// Process Error
// -----------------------------------------------------------------------------

define_error! {
  #[kind(ProcessErrorKind)]
  pub enum ProcessError {
    Build,
    Spawn,
  }
}

// -----------------------------------------------------------------------------
// Process Drop Mode
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum DropMode {
  Pass,
  Kill,
  Wait,
}

// -----------------------------------------------------------------------------
// Process Handle
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Process {
  pub(crate) mode: DropMode,
  pub(crate) host: IpAddr,
  pub(crate) port: u16,
  pub(crate) child: Child,
}

impl Process {
  #[inline]
  pub fn builder() -> Builder {
    Builder::new()
  }

  #[inline]
  pub const fn host(&self) -> IpAddr {
    self.host
  }

  #[inline]
  pub const fn port(&self) -> u16 {
    self.port
  }

  #[inline]
  pub fn pid(&self) -> u32 {
    self.child.id()
  }
}

impl Drop for Process {
  fn drop(&mut self) {
    match self.mode {
      DropMode::Pass => {
        // do nothing
      }
      DropMode::Kill => {
        debug!("> Terminating StarCraft II Process (Kill)");

        let _out: Result<(), Error> = self.child.kill();
        let _out: Result<(), Error> = self.child.wait().map(|_| ());

        debug!("> StarCraft II Process Terminated");
      }
      DropMode::Wait => {
        debug!("> Terminating StarCraft II Process (Wait)");

        let _out: Result<(), Error> = self.child.wait().map(|_| ());

        debug!("> StarCraft II Process Terminated");
      }
    }
  }
}
