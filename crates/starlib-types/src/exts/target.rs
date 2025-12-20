use crate::bridge::Target;
use crate::core::UnitTag;
use crate::geom::PointF;

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
