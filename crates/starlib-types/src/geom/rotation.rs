use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use core::ops::Neg;

use crate::core::Float;
use crate::geom::Angle;
use crate::geom::Point;
use crate::geom::Transform;
use crate::traits::Cos;
use crate::traits::Number;
use crate::traits::Radian;
use crate::traits::Sin;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Rotation<T> {
  angle: Angle<T>,
}

impl<T> Rotation<T>
where
  T: Number,
{
  pub const IDENTITY: Self = Self::from_radians(T::ZERO);

  #[inline]
  pub const fn new(angle: Angle<T>) -> Self {
    Self { angle }
  }

  #[inline]
  pub const fn from_radians(radians: T) -> Self {
    Self::new(Angle::from_radians(radians))
  }

  #[inline]
  pub fn from_degrees(degrees: T) -> Self
  where
    T: Radian,
  {
    Self::new(Angle::from_degrees(degrees))
  }

  #[inline]
  pub const fn angle(self) -> Angle<T> {
    self.angle
  }

  #[inline]
  pub fn inverse(self) -> Self
  where
    T: Neg<Output = T>,
  {
    Self::new(-self.angle)
  }

  #[inline]
  pub fn then(self, other: &Self) -> Self {
    Self::new(self.angle + other.angle)
  }

  #[inline]
  pub fn transform(self, point: Point<T>) -> Point<T>
  where
    T: Cos + Sin,
  {
    let cos: T = self.angle.cos();
    let sin: T = self.angle.sin();

    Point::new(point.x * cos - point.y * sin, point.y * cos + point.x * sin)
  }

  #[inline]
  pub fn to_i32(self) -> Rotation<i32> {
    Rotation::new(self.angle.to_i32())
  }

  #[inline]
  pub fn to_f32(self) -> Rotation<f32> {
    Rotation::new(self.angle.to_f32())
  }

  #[inline]
  pub fn to_float(self) -> Rotation<Float> {
    Rotation::new(self.angle.to_float())
  }

  #[inline]
  pub fn to_transform(self) -> Transform<T>
  where
    T: Cos + Sin,
  {
    Transform::rotation(self.angle)
  }
}

impl<T> Debug for Rotation<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Rotation<{}>", self.angle.to_radians())
  }
}

impl<T> Display for Rotation<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(self, f)
  }
}
