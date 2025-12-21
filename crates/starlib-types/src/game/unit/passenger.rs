use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::bridge::Passenger;
use crate::core::Stat;
use crate::core::UnitTag;
use crate::stable::UnitTypeId;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PassengerRef<'a> {
  unit: &'a Passenger,
}

impl<'a> PassengerRef<'a> {
  #[inline]
  pub const fn new(unit: &'a Passenger) -> Self {
    Self { unit }
  }

  // ---------------------------------------------------------------------------
  // Properties - Identifiers
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn tag(&self) -> UnitTag {
    self.unit.tag()
  }

  #[inline]
  pub fn kind(&self) -> UnitTypeId {
    self.unit.kind()
  }

  // ---------------------------------------------------------------------------
  // Properties - Stats
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn health(&self) -> Stat<f32> {
    Stat::new(self.unit.health(), self.unit.health_max())
  }

  #[inline]
  pub fn shield(&self) -> Stat<f32> {
    Stat::new(self.unit.shield(), self.unit.shield_max())
  }

  #[inline]
  pub fn energy(&self) -> Stat<f32> {
    Stat::new(self.unit.energy(), self.unit.energy_max())
  }
}

impl Debug for PassengerRef<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("PassengerRef")
      .field("tag", &self.tag())
      .field("kind", &self.kind())
      .field("health", &self.health())
      .field("shield", &self.shield())
      .field("energy", &self.energy())
      .finish()
  }
}
