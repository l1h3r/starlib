use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

// -----------------------------------------------------------------------------
// Player Tag
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PlayerTag(u32);

impl PlayerTag {
  #[inline]
  pub const fn from_u32(this: u32) -> Self {
    Self(this)
  }

  #[inline]
  pub const fn into_u32(self) -> u32 {
    self.0
  }

  #[inline]
  pub fn into_option(self) -> Option<Self> {
    (self.0 != 0).then_some(self)
  }
}

impl Debug for PlayerTag {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.into_option() {
      Some(this) => write!(f, "PlayerTag<{}>", this.0),
      None => write!(f, "Null"),
    }
  }
}

impl Display for PlayerTag {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.into_option() {
      Some(this) => write!(f, "P<{}>", this.0),
      None => write!(f, "Null"),
    }
  }
}
