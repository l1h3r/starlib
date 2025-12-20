use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use crate::core::Float;

// -----------------------------------------------------------------------------
// Number
// -----------------------------------------------------------------------------

pub trait Number:
  Copy
  + Debug
  + Display
  + PartialEq
  + PartialOrd
  + Add<Output = Self>
  + Sub<Output = Self>
  + Mul<Output = Self>
  + Div<Output = Self>
  + Rem<Output = Self>
{
  const ZERO: Self;

  const ONE: Self;
  const TWO: Self;

  fn sqrt(self) -> Self;

  fn round(self) -> Self;

  fn floor(self) -> Self;

  fn ceil(self) -> Self;

  fn div_euclid(self, other: Self) -> Self;

  fn rem_euclid(self, other: Self) -> Self;

  fn to_f32(self) -> f32;

  fn to_i32(self) -> i32;

  #[inline]
  fn to_float(self) -> Float {
    Float::from_f32(<Self as Number>::to_f32(self))
  }

  #[inline]
  fn max(self, other: Self) -> Self {
    if self > other { self } else { other }
  }

  #[inline]
  fn min(self, other: Self) -> Self {
    if self < other { self } else { other }
  }
}

// -----------------------------------------------------------------------------
// Implementations
// -----------------------------------------------------------------------------

impl Number for Float {
  const ZERO: Self = Self::from_f32(0.0);

  const ONE: Self = Self::from_f32(1.0);
  const TWO: Self = Self::from_f32(2.0);

  #[inline]
  fn sqrt(self) -> Self {
    Self::from_f32(<f32 as Number>::sqrt(*self))
  }

  #[inline]
  fn round(self) -> Self {
    Self::from_f32(<f32 as Number>::round(*self))
  }

  #[inline]
  fn floor(self) -> Self {
    Self::from_f32(<f32 as Number>::floor(*self))
  }

  #[inline]
  fn ceil(self) -> Self {
    Self::from_f32(<f32 as Number>::ceil(*self))
  }

  #[inline]
  fn div_euclid(self, other: Self) -> Self {
    Self::from_f32(<f32 as Number>::div_euclid(*self, *other))
  }

  #[inline]
  fn rem_euclid(self, other: Self) -> Self {
    Self::from_f32(<f32 as Number>::rem_euclid(*self, *other))
  }

  #[inline]
  fn to_f32(self) -> f32 {
    *self
  }

  #[inline]
  fn to_i32(self) -> i32 {
    <f32 as Number>::to_i32(*self)
  }

  #[inline]
  fn to_float(self) -> Float {
    self
  }
}

impl Number for f32 {
  const ZERO: Self = 0.0;

  const ONE: Self = 1.0;
  const TWO: Self = 2.0;

  #[inline]
  fn sqrt(self) -> Self {
    f32::sqrt(self)
  }

  #[inline]
  fn round(self) -> Self {
    (self + 0.5).floor()
  }

  #[inline]
  fn floor(self) -> Self {
    f32::floor(self)
  }

  #[inline]
  fn ceil(self) -> Self {
    f32::ceil(self)
  }

  #[inline]
  fn div_euclid(self, other: Self) -> Self {
    f32::div_euclid(self, other)
  }

  #[inline]
  fn rem_euclid(self, other: Self) -> Self {
    f32::rem_euclid(self, other)
  }

  #[inline]
  fn to_f32(self) -> f32 {
    self
  }

  #[inline]
  fn to_i32(self) -> i32 {
    self as i32
  }
}

impl Number for u32 {
  const ZERO: Self = 0;

  const ONE: Self = 1;
  const TWO: Self = 2;

  #[inline]
  fn sqrt(self) -> Self {
    u32::isqrt(self)
  }

  #[inline]
  fn round(self) -> Self {
    self
  }

  #[inline]
  fn floor(self) -> Self {
    self
  }

  #[inline]
  fn ceil(self) -> Self {
    self
  }

  #[inline]
  fn div_euclid(self, other: Self) -> Self {
    u32::div_euclid(self, other)
  }

  #[inline]
  fn rem_euclid(self, other: Self) -> Self {
    u32::rem_euclid(self, other)
  }

  #[inline]
  fn to_f32(self) -> f32 {
    self as f32
  }

  #[inline]
  fn to_i32(self) -> i32 {
    self as i32
  }
}

impl Number for i32 {
  const ZERO: Self = 0;

  const ONE: Self = 1;
  const TWO: Self = 2;

  #[inline]
  fn sqrt(self) -> Self {
    i32::isqrt(self)
  }

  #[inline]
  fn round(self) -> Self {
    self
  }

  #[inline]
  fn floor(self) -> Self {
    self
  }

  #[inline]
  fn ceil(self) -> Self {
    self
  }

  #[inline]
  fn div_euclid(self, other: Self) -> Self {
    i32::div_euclid(self, other)
  }

  #[inline]
  fn rem_euclid(self, other: Self) -> Self {
    i32::rem_euclid(self, other)
  }

  #[inline]
  fn to_f32(self) -> f32 {
    self as f32
  }

  #[inline]
  fn to_i32(self) -> i32 {
    self
  }
}
