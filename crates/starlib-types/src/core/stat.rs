use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::core::Percent;
use crate::traits::Number;

// -----------------------------------------------------------------------------
// Stat
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stat<T> {
  current: T,
  maximum: T,
}

impl<T> Stat<T> {
  #[inline]
  pub const fn new(current: T, maximum: T) -> Self {
    Self { current, maximum }
  }
}

impl<T> Stat<T>
where
  T: Number,
{
  #[inline]
  pub const fn current(&self) -> T {
    self.current
  }

  #[inline]
  pub const fn maximum(&self) -> T {
    self.maximum
  }

  #[inline]
  pub fn percent(&self) -> Percent {
    Percent::from_f32(self.current.to_f32() / self.maximum.to_f32())
  }
}

impl<T> Debug for Stat<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Stat<{}/{} - {}%>",
      self.current,
      self.maximum,
      self.percent()
    )
  }
}

impl<T> Display for Stat<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "S<{}/{} - {}%>",
      self.current,
      self.maximum,
      self.percent()
    )
  }
}
