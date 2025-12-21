use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

use crate::core::Float;
use crate::geom::Point;
use crate::traits::Ceil;
use crate::traits::Euclid;
use crate::traits::Floor;
use crate::traits::Number;
use crate::traits::Round;
use crate::traits::Sqrt;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Point3D<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Point3D<T>
where
  T: Number,
{
  pub const ZERO: Self = Self::splat(T::ZERO);

  pub const ONE: Self = Self::splat(T::ONE);

  #[inline]
  pub const fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }

  #[inline]
  pub const fn splat(value: T) -> Self {
    Self::new(value, value, value)
  }

  #[inline]
  pub const fn from_array([x, y, z]: [T; 3]) -> Self {
    Self::new(x, y, z)
  }

  #[inline]
  pub fn map<F>(self, f: F) -> Self
  where
    F: Fn(T) -> T,
  {
    Self::new(f(self.x), f(self.y), f(self.z))
  }

  #[inline]
  pub const fn xy(self) -> Point<T> {
    Point::new(self.x, self.y)
  }

  #[inline]
  pub const fn xz(self) -> Point<T> {
    Point::new(self.x, self.z)
  }

  #[inline]
  pub const fn yz(self) -> Point<T> {
    Point::new(self.y, self.z)
  }

  #[inline]
  pub fn dot(self, other: Self) -> T {
    (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
  }

  #[inline]
  pub fn cross(self, other: Self) -> Self {
    Self {
      x: self.y * other.z - other.y * self.z,
      y: self.z * other.x - other.z * self.x,
      z: self.x * other.y - other.x * self.y,
    }
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
  pub fn ceil(self) -> Self
  where
    T: Ceil,
  {
    Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
  }

  #[inline]
  pub fn floor(self) -> Self
  where
    T: Floor,
  {
    Self::new(self.x.floor(), self.y.floor(), self.z.floor())
  }

  #[inline]
  pub fn round(self) -> Self
  where
    T: Round,
  {
    Self::new(self.x.round(), self.y.round(), self.z.round())
  }

  #[inline]
  pub fn max(self, other: Self) -> Self {
    Self::new(
      self.x.max(other.x),
      self.y.max(other.y),
      self.z.max(other.z),
    )
  }

  #[inline]
  pub fn min(self, other: Self) -> Self {
    Self::new(
      self.x.min(other.x),
      self.y.min(other.y),
      self.z.min(other.z),
    )
  }

  #[inline]
  pub fn clamp(self, min: Self, max: Self) -> Self {
    self.max(min).min(max)
  }

  #[inline]
  pub fn div_euclid(self, other: Self) -> Self
  where
    T: Euclid,
  {
    Self::new(
      self.x.div_euclid(other.x),
      self.y.div_euclid(other.y),
      self.z.div_euclid(other.z),
    )
  }

  #[inline]
  pub fn rem_euclid(self, other: Self) -> Self
  where
    T: Euclid,
  {
    Self::new(
      self.x.rem_euclid(other.x),
      self.y.rem_euclid(other.y),
      self.z.rem_euclid(other.z),
    )
  }

  #[inline]
  pub const fn to_array(self) -> [T; 3] {
    [self.x, self.y, self.z]
  }

  #[inline]
  pub fn to_i32(self) -> Point3D<i32> {
    Point3D::new(self.x.to_i32(), self.y.to_i32(), self.z.to_i32())
  }

  #[inline]
  pub fn to_f32(self) -> Point3D<f32> {
    Point3D::new(self.x.to_f32(), self.y.to_f32(), self.z.to_f32())
  }

  #[inline]
  pub fn to_float(self) -> Point3D<Float> {
    Point3D::new(self.x.to_float(), self.y.to_float(), self.z.to_float())
  }
}

impl<T> Default for Point3D<T>
where
  T: Number,
{
  #[inline]
  fn default() -> Self {
    Self::ZERO
  }
}

impl<T> Debug for Point3D<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Point3D<{},{},{}>", self.x, self.y, self.z)
  }
}

impl<T> Display for Point3D<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(self, f)
  }
}

op!(Point3D[x, y, z], Add::add(+), AddAssign::add_assign);
op!(Point3D[x, y, z], Sub::sub(-), SubAssign::sub_assign);
op!(Point3D[x, y, z], Mul::mul(*), MulAssign::mul_assign);
op!(Point3D[x, y, z], Div::div(/), DivAssign::div_assign);
op!(Point3D[x, y, z], Rem::rem(%), RemAssign::rem_assign);
