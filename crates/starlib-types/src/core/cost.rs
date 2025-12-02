use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::core::Float;

// -----------------------------------------------------------------------------
// Cost
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cost {
  minerals: u32,
  vespene: u32,
  supply: Float,
}

impl Cost {
  #[inline]
  pub const fn new(minerals: u32, vespene: u32, supply: Float) -> Self {
    Self {
      minerals,
      vespene,
      supply,
    }
  }

  #[inline]
  pub const fn minerals(&self) -> u32 {
    self.minerals
  }

  #[inline]
  pub const fn vespene(&self) -> u32 {
    self.vespene
  }

  #[inline]
  pub const fn supply(&self) -> Float {
    self.supply
  }
}

impl Debug for Cost {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Cost<{}M / {}V / {}S>",
      self.minerals, self.vespene, self.supply,
    )
  }
}

impl Display for Cost {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "C<{}M / {}V / {}S>",
      self.minerals, self.vespene, self.supply,
    )
  }
}
