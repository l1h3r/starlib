use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use crate::core::Float;

pub trait Cast {
  fn to_f32(self) -> f32;
  fn to_i32(self) -> i32;
  fn to_float(self) -> Float;
}

pub trait Zero {
  const ZERO: Self;
}

pub trait One {
  const ONE: Self;
}

pub trait Two {
  const TWO: Self;
}

pub trait Sqrt {
  fn sqrt(self) -> Self;
}

pub trait Euclid {
  fn div_euclid(self, other: Self) -> Self;
  fn rem_euclid(self, other: Self) -> Self;
}

pub trait Pi {
  const PI: Self;
}

pub trait Ceil {
  fn ceil(self) -> Self;
}

pub trait Floor {
  fn floor(self) -> Self;
}

pub trait Round {
  fn round(self) -> Self;
}

pub trait Sin {
  fn sin(self) -> Self;
}

pub trait Cos {
  fn cos(self) -> Self;
}

pub trait Tan {
  fn tan(self) -> Self;
}

pub trait Radian {
  fn from_degrees(degrees: Self) -> Self;
  fn into_degrees(radians: Self) -> Self;
}

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
  + Zero
  + One
  + Two
  + Cast
{
  #[inline]
  fn max(self, other: Self) -> Self {
    if self > other { self } else { other }
  }

  #[inline]
  fn min(self, other: Self) -> Self {
    if self < other { self } else { other }
  }
}

impl<T> Number for T
where
  T: Copy,
  T: Debug,
  T: Display,
  T: PartialEq,
  T: PartialOrd,
  T: Add<Output = Self>,
  T: Sub<Output = Self>,
  T: Mul<Output = Self>,
  T: Div<Output = Self>,
  T: Rem<Output = Self>,
  T: Zero,
  T: One,
  T: Two,
  T: Cast,
{
}

// -----------------------------------------------------------------------------
// Implementations - f32
// -----------------------------------------------------------------------------

impl Cast for f32 {
  #[inline]
  fn to_f32(self) -> f32 {
    self
  }

  #[inline]
  fn to_i32(self) -> i32 {
    self as i32
  }

  #[inline]
  fn to_float(self) -> Float {
    Float::from_f32(self)
  }
}

impl Zero for f32 {
  const ZERO: Self = 0.0;
}

impl One for f32 {
  const ONE: Self = 1.0;
}

impl Two for f32 {
  const TWO: Self = 2.0;
}

impl Sqrt for f32 {
  #[inline]
  fn sqrt(self) -> Self {
    f32::sqrt(self)
  }
}

impl Euclid for f32 {
  #[inline]
  fn div_euclid(self, other: Self) -> Self {
    f32::div_euclid(self, other)
  }

  #[inline]
  fn rem_euclid(self, other: Self) -> Self {
    f32::rem_euclid(self, other)
  }
}

impl Pi for f32 {
  const PI: Self = ::core::f32::consts::PI;
}

impl Ceil for f32 {
  #[inline]
  fn ceil(self) -> Self {
    f32::ceil(self)
  }
}

impl Floor for f32 {
  #[inline]
  fn floor(self) -> Self {
    f32::floor(self)
  }
}

impl Round for f32 {
  #[inline]
  fn round(self) -> Self {
    f32::round(self)
  }
}

impl Sin for f32 {
  #[inline]
  fn sin(self) -> Self {
    f32::sin(self)
  }
}

impl Cos for f32 {
  #[inline]
  fn cos(self) -> Self {
    f32::cos(self)
  }
}

impl Tan for f32 {
  #[inline]
  fn tan(self) -> Self {
    f32::tan(self)
  }
}

impl Radian for f32 {
  #[inline]
  fn from_degrees(degrees: Self) -> Self {
    f32::to_radians(degrees)
  }

  #[inline]
  fn into_degrees(radians: Self) -> Self {
    f32::to_degrees(radians)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Float
// -----------------------------------------------------------------------------

impl Cast for Float {
  #[inline]
  fn to_f32(self) -> f32 {
    *self
  }

  #[inline]
  fn to_i32(self) -> i32 {
    <f32 as Cast>::to_i32(*self)
  }

  #[inline]
  fn to_float(self) -> Float {
    self
  }
}

impl Zero for Float {
  const ZERO: Self = Self::from_f32(<f32 as Zero>::ZERO);
}

impl One for Float {
  const ONE: Self = Self::from_f32(<f32 as One>::ONE);
}

impl Two for Float {
  const TWO: Self = Self::from_f32(<f32 as Two>::TWO);
}

impl Sqrt for Float {
  #[inline]
  fn sqrt(self) -> Self {
    Self::from_f32(<f32 as Sqrt>::sqrt(*self))
  }
}

impl Euclid for Float {
  #[inline]
  fn div_euclid(self, other: Self) -> Self {
    Self::from_f32(<f32 as Euclid>::div_euclid(*self, *other))
  }

  #[inline]
  fn rem_euclid(self, other: Self) -> Self {
    Self::from_f32(<f32 as Euclid>::rem_euclid(*self, *other))
  }
}

impl Pi for Float {
  const PI: Self = Float::from_f32(<f32 as Pi>::PI);
}

impl Ceil for Float {
  #[inline]
  fn ceil(self) -> Self {
    Self::from_f32(<f32 as Ceil>::ceil(*self))
  }
}

impl Floor for Float {
  #[inline]
  fn floor(self) -> Self {
    Self::from_f32(<f32 as Floor>::floor(*self))
  }
}

impl Round for Float {
  #[inline]
  fn round(self) -> Self {
    Self::from_f32(<f32 as Round>::round(*self))
  }
}

impl Sin for Float {
  #[inline]
  fn sin(self) -> Self {
    Self::from_f32(<f32 as Sin>::sin(*self))
  }
}

impl Cos for Float {
  #[inline]
  fn cos(self) -> Self {
    Self::from_f32(<f32 as Cos>::cos(*self))
  }
}

impl Tan for Float {
  #[inline]
  fn tan(self) -> Self {
    Self::from_f32(<f32 as Tan>::tan(*self))
  }
}

impl Radian for Float {
  #[inline]
  fn from_degrees(degrees: Self) -> Self {
    Self::from_f32(<f32 as Radian>::from_degrees(*degrees))
  }

  #[inline]
  fn into_degrees(radians: Self) -> Self {
    Self::from_f32(<f32 as Radian>::into_degrees(*radians))
  }
}

// -----------------------------------------------------------------------------
// Implementations - i32
// -----------------------------------------------------------------------------

impl Cast for i32 {
  #[inline]
  fn to_f32(self) -> f32 {
    self as f32
  }

  #[inline]
  fn to_i32(self) -> i32 {
    self
  }

  #[inline]
  fn to_float(self) -> Float {
    Float::from_f32(self.to_f32())
  }
}

impl Zero for i32 {
  const ZERO: Self = 0;
}

impl One for i32 {
  const ONE: Self = 1;
}

impl Two for i32 {
  const TWO: Self = 2;
}

impl Sqrt for i32 {
  #[inline]
  fn sqrt(self) -> Self {
    i32::isqrt(self)
  }
}

impl Euclid for i32 {
  #[inline]
  fn div_euclid(self, other: Self) -> Self {
    i32::div_euclid(self, other)
  }

  #[inline]
  fn rem_euclid(self, other: Self) -> Self {
    i32::rem_euclid(self, other)
  }
}
