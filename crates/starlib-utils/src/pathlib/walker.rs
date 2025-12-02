use std::fs::FileType;
use std::io::Error;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::IntoIter;
use walkdir::WalkDir;

use crate::debug;

// -----------------------------------------------------------------------------
// Build Version Iterator
// -----------------------------------------------------------------------------

#[repr(transparent)]
pub struct BuildIter(IntoIter);

impl BuildIter {
  const PREFIX: &str = "Base";

  #[inline]
  pub fn new<P>(root: P) -> Self
  where
    P: AsRef<Path>,
  {
    Self(Self::walkdir(root.as_ref()).into_iter())
  }

  pub fn latest<P>(root: P) -> Result<u32, Error>
  where
    P: AsRef<Path>,
  {
    let mut value: Option<u32> = None;

    for entry in Self::new(root) {
      let build: u32 = entry?;

      let other: u32 = match value {
        Some(value) => value.max(build),
        None => build,
      };

      value = Some(other);
    }

    value.ok_or_else(|| Error::other("failed to find build version"))
  }

  fn walkdir(root: &Path) -> WalkDir {
    WalkDir::new(root)
      .contents_first(false)
      .follow_links(false)
      .follow_root_links(true)
      .max_depth(1)
      .max_open(1)
      .min_depth(1)
      .same_file_system(false)
  }
}

impl Iterator for BuildIter {
  type Item = Result<u32, Error>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let entry: DirEntry = match self.0.next() {
        Some(Ok(value)) => value,
        Some(Err(error)) => return Some(Err(error.into_io_error()?)),
        None => return None,
      };

      let ftype: FileType = entry.file_type();

      if !ftype.is_dir() {
        continue;
      }

      let build: Option<Result<u32, _>> = entry
        .file_name()
        .to_str()
        .and_then(|name| name.strip_prefix(Self::PREFIX))
        .map(str::parse);

      if let Some(Ok(build)) = build {
        return Some(Ok(build));
      }

      debug!("> Version Scan Failed at {}", entry.path().display());
    }
  }
}
