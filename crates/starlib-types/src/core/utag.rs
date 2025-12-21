use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

// -----------------------------------------------------------------------------
// Unit Tag
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UnitTag(u64);

impl UnitTag {
  #[inline]
  pub const fn from_u64(this: u64) -> Self {
    Self(this)
  }

  #[inline]
  pub const fn into_u64(self) -> u64 {
    self.0
  }

  #[inline]
  pub fn into_option(self) -> Option<Self> {
    (self.0 != 0).then_some(self)
  }
}

impl Debug for UnitTag {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.into_option() {
      Some(this) => write!(f, "UnitTag<{}>", this.0),
      None => write!(f, "Null"),
    }
  }
}

impl Display for UnitTag {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.into_option() {
      Some(this) => write!(f, "U<{}>", this.0),
      None => write!(f, "Null"),
    }
  }
}
