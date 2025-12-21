use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::core::Float;
use crate::geom::Point3D;
use crate::traits::Ceil;
use crate::traits::Euclid;
use crate::traits::Floor;
use crate::traits::Number;
use crate::traits::Round;
use crate::traits::Sqrt;

pub type PointF = Point<Float>;
pub type PointI = Point<i32>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T>
where
  T: Number,
{
  pub const ZERO: Self = Self::splat(T::ZERO);

  pub const ONE: Self = Self::splat(T::ONE);

  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  #[inline]
  pub const fn splat(value: T) -> Self {
    Self::new(value, value)
  }

  #[inline]
  pub const fn from_array([x, y]: [T; 2]) -> Self {
    Self::new(x, y)
  }

  #[inline]
  pub fn map<F>(self, f: F) -> Self
  where
    F: Fn(T) -> T,
  {
    Self::new(f(self.x), f(self.y))
  }

  #[inline]
  pub fn dot(self, other: Self) -> T {
    (self.x * other.x) + (self.y * other.y)
  }

  #[inline]
  pub fn cross(self, other: Self) -> T {
    (self.x * other.y) - (self.y * other.x)
  }

  #[inline]
  pub fn length(self) -> T
  where
    T: Sqrt,
  {
    self.dot(self).sqrt()
  }

  #[inline]
  pub fn length_squared(self) -> T {
    self.dot(self)
  }

  #[inline]
  pub fn distance(self, other: Self) -> T
  where
    T: Sqrt,
  {
    (self - other).length()
  }

  #[inline]
  pub fn distance_squared(self, other: Self) -> T {
    (self - other).length_squared()
  }

  #[inline]
  pub fn div_euclid(self, other: Self) -> Self
  where
    T: Euclid,
  {
    Self::new(self.x.div_euclid(other.x), self.y.div_euclid(other.y))
  }

  #[inline]
  pub fn rem_euclid(self, other: Self) -> Self
  where
    T: Euclid,
  {
    Self::new(self.x.rem_euclid(other.x), self.y.rem_euclid(other.y))
  }

  #[inline]
  pub fn ceil(self) -> Self
  where
    T: Ceil,
  {
    Self::new(self.x.ceil(), self.y.ceil())
  }

  #[inline]
  pub fn floor(self) -> Self
  where
    T: Floor,
  {
    Self::new(self.x.floor(), self.y.floor())
  }

  #[inline]
  pub fn round(self) -> Self
  where
    T: Round,
  {
    Self::new(self.x.round(), self.y.round())
  }

  #[inline]
  pub fn max(self, other: Self) -> Self {
    Self::new(self.x.max(other.x), self.y.max(other.y))
  }

  #[inline]
  pub fn min(self, other: Self) -> Self {
    Self::new(self.x.min(other.x), self.y.min(other.y))
  }

  #[inline]
  pub fn towards(self, other: Self, offset: T) -> Self
  where
    T: Sqrt,
  {
    self + (other - self) / self.distance(other) * offset
  }

  #[inline]
  pub const fn to_array(self) -> [T; 2] {
    [self.x, self.y]
  }

  #[inline]
  pub fn to_f32(self) -> Point<f32> {
    Point::new(self.x.to_f32(), self.y.to_f32())
  }

  #[inline]
  pub fn to_i32(self) -> Point<i32> {
    Point::new(self.x.to_i32(), self.y.to_i32())
  }

  #[inline]
  pub fn to_float(self) -> Point<Float> {
    Point::new(self.x.to_float(), self.y.to_float())
  }

  #[inline]
  pub const fn to_3d(self) -> Point3D<T> {
    Point3D::new(self.x, self.y, T::ZERO)
  }
}

impl<T> Default for Point<T>
where
  T: Number,
{
  #[inline]
  fn default() -> Self {
    Self::ZERO
  }
}

impl<T> Debug for Point<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Point[{},{}]", self.x, self.y)
  }
}

impl<T> Display for Point<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{},{}]", self.x, self.y)
  }
}

op!(Point[x, y], Add::add(+), AddAssign::add_assign);
op!(Point[x, y], Sub::sub(-), SubAssign::sub_assign);
op!(Point[x, y], Mul::mul(*), MulAssign::mul_assign);
op!(Point[x, y], Div::div(/), DivAssign::div_assign);
op!(Point[x, y], Rem::rem(%), RemAssign::rem_assign);
