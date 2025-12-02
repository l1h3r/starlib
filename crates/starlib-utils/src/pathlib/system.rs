use std::env;
use std::path::Path;
use std::path::PathBuf;

#[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
mod os {
  use super::*;

  #[inline]
  pub(crate) fn app() -> PathBuf {
    compile_error!("no `os::app` implementation for target os")
  }

  #[inline]
  pub(crate) fn bin() -> &'static Path {
    compile_error!("no `os::bin` implementation for target os")
  }

  #[inline]
  pub(crate) fn cwd() -> Option<&'static Path> {
    compile_error!("no `os::cwd` implementation for target os")
  }

  #[inline]
  pub(crate) fn info() -> Option<&'static Path> {
    compile_error!("no `os::info` implementation for target os")
  }
}

#[cfg(target_os = "linux")]
mod os {
  use super::*;

  #[inline]
  pub(crate) fn app() -> PathBuf {
    env::home_dir().expect("home_dir").join("StarCraftII")
  }

  #[cfg(target_arch = "x86_64")]
  #[inline]
  pub(crate) fn bin() -> &'static Path {
    Path::new("SC2_x64")
  }

  #[cfg(target_arch = "x86")]
  #[inline]
  pub(crate) fn bin() -> &'static Path {
    Path::new("SC2")
  }

  #[inline]
  pub(crate) fn cwd() -> Option<&'static Path> {
    None
  }

  #[inline]
  pub(crate) fn info() -> Option<&'static Path> {
    None
  }
}

#[cfg(target_os = "macos")]
mod os {
  use super::*;

  #[inline]
  pub(crate) fn app() -> PathBuf {
    PathBuf::from("/Applications/Starcraft II")
  }

  #[inline]
  pub(crate) fn bin() -> &'static Path {
    Path::new("SC2.app/Contents/MacOS/SC2")
  }

  #[inline]
  pub(crate) fn cwd() -> Option<&'static Path> {
    None
  }

  #[inline]
  pub(crate) fn info() -> Option<&'static Path> {
    Some(Path::new(
      "Library/Application Support/Blizzard/StarCraft II/ExecuteInfo.txt",
    ))
  }
}

#[cfg(windows)]
mod os {
  use super::*;

  #[inline]
  pub(crate) fn app() -> PathBuf {
    PathBuf::from("C:\\Program Files (x86)\\StarCraft II")
  }

  #[inline]
  pub(crate) fn bin() -> &'static str {
    "SC2_x64.exe"
  }

  #[inline]
  pub(crate) fn cwd() -> Option<&'static Path> {
    Some(Path::new("Support64"))
  }

  #[inline]
  pub(crate) fn info() -> Option<&'static Path> {
    Some(Path::new("Documents\\StarCraft II\\ExecuteInfo.txt"))
  }
}

#[inline]
pub fn app() -> PathBuf {
  env::var_os("STARLIB_SC2PATH")
    .map(PathBuf::from)
    .unwrap_or_else(os::app)
}

#[inline]
pub fn bin() -> &'static Path {
  os::bin()
}

#[inline]
pub fn cwd() -> Option<&'static Path> {
  os::cwd()
}

#[inline]
pub fn info() -> Option<&'static Path> {
  os::info()
}
