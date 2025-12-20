use starlib_proto::Unit;

use crate::bridge::Alliance;
use crate::bridge::DisplayType;
use crate::geom::Point3DF;
use crate::traits::IntoSc2;

#[derive(Debug)]
pub struct Blip {
  display: DisplayType,
  alliance: Alliance,
  position: Point3DF,
}

impl Blip {
  #[inline]
  pub fn new(unit: &Unit) -> Self {
    Self {
      display: unit.display_type.into_sc2(),
      alliance: unit.alliance.into_sc2(),
      position: unit.pos.unwrap_or_default().into_sc2(),
    }
  }

  #[inline]
  pub const fn display(&self) -> DisplayType {
    self.display
  }

  #[inline]
  pub const fn alliance(&self) -> Alliance {
    self.alliance
  }

  #[inline]
  pub const fn position(&self) -> Point3DF {
    self.position
  }
}
