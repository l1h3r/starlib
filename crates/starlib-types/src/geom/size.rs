use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::core::Float;
use crate::traits::Number;

pub type SizeF = Size<Float>;
pub type SizeI = Size<i32>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Size<T> {
  pub w: T,
  pub h: T,
}

impl<T> Size<T>
where
  T: Number,
{
  pub const ZERO: Self = Self::splat(T::ZERO);

  pub const ONE: Self = Self::splat(T::ONE);

  #[inline]
  pub const fn new(w: T, h: T) -> Self {
    Self { w, h }
  }

  #[inline]
  pub const fn splat(value: T) -> Self {
    Self::new(value, value)
  }

  #[inline]
  pub const fn from_array([w, h]: [T; 2]) -> Self {
    Self::new(w, h)
  }

  #[inline]
  pub fn map<F>(self, f: F) -> Self
  where
    F: Fn(T) -> T,
  {
    Self::new(f(self.w), f(self.h))
  }

  #[inline]
  pub fn area(self) -> T {
    self.w.mul(self.h)
  }

  #[inline]
  pub fn ceil(self) -> Self {
    Self::new(self.w.ceil(), self.h.ceil())
  }

  #[inline]
  pub fn floor(self) -> Self {
    Self::new(self.w.floor(), self.h.floor())
  }

  #[inline]
  pub fn round(self) -> Self {
    Self::new(self.w.round(), self.h.round())
  }

  #[inline]
  pub fn max(self, other: Self) -> Self {
    Self::new(self.w.max(other.w), self.h.max(other.h))
  }

  #[inline]
  pub fn min(self, other: Self) -> Self {
    Self::new(self.w.min(other.w), self.h.min(other.h))
  }

  #[inline]
  pub const fn to_array(self) -> [T; 2] {
    [self.w, self.h]
  }

  #[inline]
  pub fn to_f32(self) -> Size<f32> {
    Size::new(self.w.to_f32(), self.h.to_f32())
  }

  #[inline]
  pub fn to_i32(self) -> Size<i32> {
    Size::new(self.w.to_i32(), self.h.to_i32())
  }

  #[inline]
  pub fn to_float(self) -> Size<Float> {
    Size::new(self.w.to_float(), self.h.to_float())
  }
}

impl<T> Default for Size<T>
where
  T: Number,
{
  #[inline]
  fn default() -> Self {
    Self::ZERO
  }
}

impl<T> Debug for Size<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Size[{}x{}]", self.w, self.h)
  }
}

impl<T> Display for Size<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{}x{}]", self.w, self.h)
  }
}
