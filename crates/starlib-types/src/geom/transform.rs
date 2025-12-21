use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

use crate::core::Float;
use crate::geom::Angle;
use crate::geom::Point;
use crate::traits::Cos;
use crate::traits::Number;
use crate::traits::Sin;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Transform<T> {
  pub m11: T,
  pub m12: T,
  pub m21: T,
  pub m22: T,
  pub m31: T,
  pub m32: T,
}

impl<T> Transform<T>
where
  T: Number,
{
  pub const IDENTITY: Self = Self::translation(T::ZERO, T::ZERO);

  #[inline]
  pub const fn new(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Self {
    Self {
      m11,
      m12,
      m21,
      m22,
      m31,
      m32,
    }
  }

  #[inline]
  pub const fn from_array([m11, m12, m21, m22, m31, m32]: [T; 6]) -> Self {
    Self::new(m11, m12, m21, m22, m31, m32)
  }

  #[inline]
  pub fn rotation(theta: Angle<T>) -> Self
  where
    T: Cos + Sin,
  {
    let cos: T = theta.to_radians().cos();
    let sin: T = theta.to_radians().sin();

    Self::new(cos, sin, T::ZERO - sin, cos, T::ZERO, T::ZERO)
  }

  #[inline]
  pub const fn scale(x: T, y: T) -> Self {
    Self::new(x, T::ZERO, T::ZERO, y, T::ZERO, T::ZERO)
  }

  #[inline]
  pub const fn translation(x: T, y: T) -> Self {
    Self::new(T::ONE, T::ZERO, T::ZERO, T::ONE, x, y)
  }

  #[inline]
  pub fn then(&self, other: &Self) -> Self {
    Self::new(
      self.m11 * other.m11 + self.m12 * other.m21,
      self.m11 * other.m12 + self.m12 * other.m22,
      self.m21 * other.m11 + self.m22 * other.m21,
      self.m21 * other.m12 + self.m22 * other.m22,
      self.m31 * other.m11 + self.m32 * other.m21 + other.m31,
      self.m31 * other.m12 + self.m32 * other.m22 + other.m32,
    )
  }

  #[inline]
  pub fn transform(&self, point: Point<T>) -> Point<T> {
    Point::new(
      point.x * self.m11 + point.y * self.m21 + self.m31,
      point.x * self.m12 + point.y * self.m22 + self.m32,
    )
  }

  #[inline]
  pub const fn to_array(self) -> [T; 6] {
    [self.m11, self.m12, self.m21, self.m22, self.m31, self.m32]
  }

  #[inline]
  pub fn to_i32(self) -> Transform<i32> {
    Transform::new(
      self.m11.to_i32(),
      self.m12.to_i32(),
      self.m21.to_i32(),
      self.m22.to_i32(),
      self.m31.to_i32(),
      self.m32.to_i32(),
    )
  }

  #[inline]
  pub fn to_f32(self) -> Transform<f32> {
    Transform::new(
      self.m11.to_f32(),
      self.m12.to_f32(),
      self.m21.to_f32(),
      self.m22.to_f32(),
      self.m31.to_f32(),
      self.m32.to_f32(),
    )
  }

  #[inline]
  pub fn to_float(self) -> Transform<Float> {
    Transform::new(
      self.m11.to_float(),
      self.m12.to_float(),
      self.m21.to_float(),
      self.m22.to_float(),
      self.m31.to_float(),
      self.m32.to_float(),
    )
  }
}

impl<T> Debug for Transform<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Transform<{},{},{},{},{},{}>",
      self.m11, self.m12, self.m21, self.m22, self.m31, self.m32
    )
  }
}

impl<T> Display for Transform<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(self, f)
  }
}
