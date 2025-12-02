use crate::bridge::Target;
use crate::core::PointF;
use crate::core::UnitTag;

impl From<PointF> for Target {
  #[inline]
  fn from(other: PointF) -> Self {
    Target::Point(other)
  }
}

impl From<UnitTag> for Target {
  #[inline]
  fn from(other: UnitTag) -> Self {
    Target::Unit(other)
  }
}
