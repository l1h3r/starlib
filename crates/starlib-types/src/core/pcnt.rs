use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::Float;

// -----------------------------------------------------------------------------
// Percent
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Percent(Float);

impl Percent {
  #[inline]
  pub const fn from_f32(this: f32) -> Self {
    Self(Float::from_f32(this))
  }

  #[inline]
  pub const fn into_f32(self) -> f32 {
    self.0.into_f32()
  }

  #[inline]
  pub const fn into_f32_scaled(self) -> f32 {
    (self.0.into_f32() * 100.0).min(100.0)
  }
}

impl Debug for Percent {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(&self.into_f32_scaled(), f)
  }
}

impl Display for Percent {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Display::fmt(&self.into_f32_scaled(), f)
  }
}

impl Deref for Percent {
  type Target = Float;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Percent {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
