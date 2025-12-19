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

  fn as_f32(self) -> f32;

  #[inline]
  fn as_float(self) -> Float {
    Float::from_f32(<Self as Number>::as_f32(self))
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
    Self::from_f32(f32::sqrt(*self))
  }

  #[inline]
  fn as_f32(self) -> f32 {
    self.into_f32()
  }

  #[inline]
  fn as_float(self) -> Float {
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
  fn as_f32(self) -> f32 {
    self
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
  fn as_f32(self) -> f32 {
    self as f32
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
  fn as_f32(self) -> f32 {
    self as f32
  }
}
