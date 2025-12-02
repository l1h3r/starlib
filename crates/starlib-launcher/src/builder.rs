use std::ffi::OsStr;
use std::ffi::OsString;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;
use std::process::Stdio;

use starlib_utils::pathlib;
use starlib_utils::version::Version;

use crate::process::DropMode;
use crate::process::Process;
use crate::process::ProcessError;
use crate::process::ProcessErrorKind;

// -----------------------------------------------------------------------------
// Display Mode
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum DisplayMode {
  Windowed = 0,
  Borderless = 1,
  Fullscreen = 2,
}

impl DisplayMode {
  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Windowed => "0",
      Self::Borderless => "1",
      Self::Fullscreen => "2",
    }
  }
}

// -----------------------------------------------------------------------------
// Renderer
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Renderer {
  EGL,
  OSMesa,
}

impl Renderer {
  #[inline]
  const fn arg(&self) -> &'static str {
    match self {
      Self::EGL => "-eglpath",
      Self::OSMesa => "-osmesapath",
    }
  }
}

// -----------------------------------------------------------------------------
// Builder
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Builder {
  // ---------------------------------------------------------------------------
  // Common
  // ---------------------------------------------------------------------------
  host: IpAddr,
  port: u16,
  // ---------------------------------------------------------------------------
  // Process Flags
  // ---------------------------------------------------------------------------
  data_dir: Option<PathBuf>,
  game_dir: PathBuf,
  temp_dir: Option<PathBuf>,
  work_dir: Option<PathBuf>,
  renderer: Option<(Renderer, PathBuf)>,
  display: Option<DisplayMode>,
  version: Option<Version>,
  verbose: bool,
  os_vars: Vec<OsString>,
  window_x: Option<u32>,
  window_y: Option<u32>,
  window_w: Option<u32>,
  window_h: Option<u32>,
  // ---------------------------------------------------------------------------
  // Process Config
  // ---------------------------------------------------------------------------
  stdin: Option<Stdio>,
  stdout: Option<Stdio>,
  stderr: Option<Stdio>,
  // ---------------------------------------------------------------------------
  // Process Handle
  // ---------------------------------------------------------------------------
  drop_mode: DropMode,
}

impl Builder {
  #[inline]
  pub fn new() -> Self {
    Self {
      host: IpAddr::V4(Ipv4Addr::LOCALHOST),
      port: 5000,
      data_dir: None,
      game_dir: pathlib::app(),
      temp_dir: None,
      work_dir: None,
      renderer: None,
      display: None,
      version: None,
      verbose: cfg!(debug_assertions),
      os_vars: Vec::new(),
      window_x: None,
      window_y: None,
      window_w: None,
      window_h: None,
      stdin: Some(Stdio::null()),
      stdout: Some(Stdio::inherit()),
      stderr: Some(Stdio::inherit()),
      drop_mode: DropMode::Kill,
    }
  }

  #[inline]
  pub fn host<T>(mut self, value: T) -> Self
  where
    T: Into<IpAddr>,
  {
    self.host = value.into();
    self
  }

  #[inline]
  pub const fn port(mut self, value: u16) -> Self {
    self.port = value;
    self
  }

  #[inline]
  pub fn data_dir<T>(mut self, value: &T) -> Self
  where
    T: AsRef<Path> + ?Sized,
  {
    self.data_dir = Some(value.as_ref().to_path_buf());
    self
  }

  #[inline]
  pub fn game_dir<T>(mut self, value: &T) -> Self
  where
    T: AsRef<Path> + ?Sized,
  {
    self.game_dir = value.as_ref().to_path_buf();
    self
  }

  #[inline]
  pub fn temp_dir<T>(mut self, value: &T) -> Self
  where
    T: AsRef<Path> + ?Sized,
  {
    self.temp_dir = Some(value.as_ref().to_path_buf());
    self
  }

  #[inline]
  pub fn work_dir<T>(mut self, value: &T) -> Self
  where
    T: AsRef<Path> + ?Sized,
  {
    self.work_dir = Some(value.as_ref().to_path_buf());
    self
  }

  #[inline]
  pub fn renderer(mut self, kind: Renderer, path: PathBuf) -> Self {
    self.renderer = Some((kind, path));
    self
  }

  #[inline]
  pub const fn display(mut self, value: DisplayMode) -> Self {
    self.display = Some(value);
    self
  }

  #[inline]
  pub const fn version(mut self, value: Version) -> Self {
    self.version = Some(value);
    self
  }

  #[inline]
  pub const fn verbose(mut self, value: bool) -> Self {
    self.verbose = value;
    self
  }

  #[inline]
  pub fn os_var<T>(mut self, value: T) -> Self
  where
    T: AsRef<OsStr>,
  {
    self.os_vars.push(value.as_ref().to_os_string());
    self
  }

  #[inline]
  pub fn os_vars<I, T>(self, value: I) -> Self
  where
    I: IntoIterator<Item = T>,
    T: AsRef<OsStr>,
  {
    value
      .into_iter()
      .fold(self, |this, item| this.os_var(item.as_ref()))
  }

  #[inline]
  pub const fn window_x(mut self, value: u32) -> Self {
    self.window_x = Some(value);
    self
  }

  #[inline]
  pub const fn window_y(mut self, value: u32) -> Self {
    self.window_y = Some(value);
    self
  }

  #[inline]
  pub const fn window_w(mut self, value: u32) -> Self {
    self.window_w = Some(value);
    self
  }

  #[inline]
  pub const fn window_h(mut self, value: u32) -> Self {
    self.window_h = Some(value);
    self
  }

  #[inline]
  pub const fn window(self, x: u32, y: u32, w: u32, h: u32) -> Self {
    self.window_x(x).window_y(y).window_w(w).window_h(h)
  }

  #[inline]
  pub fn stdin<T>(mut self, value: T) -> Self
  where
    T: Into<Stdio>,
  {
    self.stdin = Some(value.into());
    self
  }

  #[inline]
  pub fn stdout<T>(mut self, value: T) -> Self
  where
    T: Into<Stdio>,
  {
    self.stdout = Some(value.into());
    self
  }

  #[inline]
  pub fn stderr<T>(mut self, value: T) -> Self
  where
    T: Into<Stdio>,
  {
    self.stderr = Some(value.into());
    self
  }

  #[inline]
  pub const fn drop_mode(mut self, value: DropMode) -> Self {
    self.drop_mode = value;
    self
  }

  pub fn output(mut self) -> Result<Output, ProcessError> {
    self
      .execute()?
      .output()
      .map_err(|error| ProcessError::new(error, ProcessErrorKind::Spawn))
  }

  pub fn status(mut self) -> Result<ExitStatus, ProcessError> {
    self
      .execute()?
      .status()
      .map_err(|error| ProcessError::new(error, ProcessErrorKind::Spawn))
  }

  pub fn spawn(mut self) -> Result<Process, ProcessError> {
    let child: Child = self
      .execute()?
      .spawn()
      .map_err(|error| ProcessError::new(error, ProcessErrorKind::Spawn))?;

    Ok(Process {
      mode: self.drop_mode,
      host: self.host,
      port: self.port,
      child,
    })
  }

  fn execute(&mut self) -> Result<Command, ProcessError> {
    debug!("> Executing StarCraft II Command");

    let path: PathBuf = self.to_binary()?;
    let hash: Option<&str> = self.version.as_ref().and_then(Version::hash_data);

    debug!("> > Using Binary");
    debug!("    {}", path.display());

    if let Some(hash) = hash {
      debug!("> > Using Data Version");
      debug!("    {hash}");
    }

    let mut command: Command = self.to_command(path.as_path(), hash);

    if let Some(value) = self.stdin.take() {
      command.stdin(value);
    }

    if let Some(value) = self.stdout.take() {
      command.stdout(value);
    }

    if let Some(value) = self.stderr.take() {
      command.stderr(value);
    }

    debug!("> > StarCraft II Command");
    debug!("    {command:?}");

    Ok(command)
  }

  fn to_binary(&self) -> Result<PathBuf, ProcessError> {
    let base: Option<u32> = self.version.as_ref().and_then(Version::base_build);
    let path: PathBuf = self.game_dir.join("Versions");

    let base: u32 = match base {
      Some(value) => value,
      None => {
        debug!("> > Scanning for Versions");
        debug!("    {}", path.display());

        match pathlib::BuildIter::latest(path.as_path()) {
          Ok(latest) => latest,
          Err(error) => return Err(ProcessError::new(error, ProcessErrorKind::Build)),
        }
      }
    };

    debug!("> > Using Base Version");
    debug!("    {base}");

    let dir: String = format!("Base{base}");
    let bin: PathBuf = path.join(dir).join(pathlib::bin());

    Ok(bin)
  }

  fn to_command(&self, binary: &Path, version: Option<&str>) -> Command {
    let mut command: Command = Command::new(binary);

    command.args(self.os_vars.as_slice());

    command.arg("-listen");
    command.arg(self.host.to_string());
    command.arg("-port");
    command.arg(self.port.to_string());

    if let Some(hash) = version {
      command.arg("-dataVersion");
      command.arg(hash);
    }

    if let Some((kind, ref path)) = self.renderer {
      command.arg(kind.arg());
      command.arg(path.as_path());
    }

    if let Some(mode) = self.display {
      command.arg("-displayMode");
      command.arg(mode.as_str());
    }

    if self.verbose {
      command.arg("-verbose");
    }

    if let Some(x) = self.window_x {
      command.arg("-windowx");
      command.arg(x.to_string());
    }

    if let Some(y) = self.window_y {
      command.arg("-windowy");
      command.arg(y.to_string());
    }

    if let Some(w) = self.window_w {
      command.arg("-windowwidth");
      command.arg(w.to_string());
    }

    if let Some(h) = self.window_h {
      command.arg("-windowheight");
      command.arg(h.to_string());
    }

    if let Some(ref value) = self.data_dir {
      command.arg("-dataDir");
      command.arg(value.as_path());
    }

    if let Some(ref value) = self.temp_dir {
      command.arg("-tempDir");
      command.arg(value.as_path());
    }

    if let Some(ref value) = self.work_dir {
      command.current_dir(value.as_path());
    } else if let Some(path) = pathlib::cwd() {
      command.current_dir(self.game_dir.join(path));
    } else {
      command.current_dir(self.game_dir.as_path());
    }

    command
  }
}

impl Clone for Builder {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      host: self.host,
      port: self.port,
      data_dir: self.data_dir.clone(),
      game_dir: self.game_dir.clone(),
      temp_dir: self.temp_dir.clone(),
      work_dir: self.work_dir.clone(),
      renderer: self.renderer.clone(),
      display: self.display,
      version: self.version,
      verbose: self.verbose,
      os_vars: self.os_vars.clone(),
      window_x: self.window_x,
      window_y: self.window_y,
      window_w: self.window_w,
      window_h: self.window_h,
      stdin: Some(Stdio::null()),
      stdout: Some(Stdio::inherit()),
      stderr: Some(Stdio::inherit()),
      drop_mode: self.drop_mode,
    }
  }
}
