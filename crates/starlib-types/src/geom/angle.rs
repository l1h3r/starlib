use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use core::ops::Neg;

use crate::core::Float;
use crate::traits::Cos;
use crate::traits::Number;
use crate::traits::Pi;
use crate::traits::Radian;
use crate::traits::Sin;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Angle<T> {
  radians: T,
}

impl<T> Angle<T>
where
  T: Number + Pi,
{
  pub const PI: Self = Self::from_radians(T::PI);

  pub const PI2: Self = Self::from_radians(T::PI2);
}

impl<T> Angle<T>
where
  T: Number,
{
  pub const ZERO: Self = Self::from_radians(T::ZERO);

  #[inline]
  pub const fn from_radians(radians: T) -> Self {
    Self { radians }
  }

  #[inline]
  pub fn from_degrees(degrees: T) -> Self
  where
    T: Radian,
  {
    Self::from_radians(T::from_degrees(degrees))
  }

  #[inline]
  pub fn map<F>(self, f: F) -> Self
  where
    F: Fn(T) -> T,
  {
    Self::from_radians(f(self.radians))
  }

  #[inline]
  pub fn cos(self) -> T
  where
    T: Cos,
  {
    self.radians.cos()
  }

  #[inline]
  pub fn sin(self) -> T
  where
    T: Sin,
  {
    self.radians.sin()
  }

  #[inline]
  pub const fn to_radians(self) -> T {
    self.radians
  }

  #[inline]
  pub fn to_degrees(self) -> T
  where
    T: Radian,
  {
    T::into_degrees(self.radians)
  }

  #[inline]
  pub fn to_i32(self) -> Angle<i32> {
    Angle::from_radians(self.radians.to_i32())
  }

  #[inline]
  pub fn to_f32(self) -> Angle<f32> {
    Angle::from_radians(self.radians.to_f32())
  }

  #[inline]
  pub fn to_float(self) -> Angle<Float> {
    Angle::from_radians(self.radians.to_float())
  }
}

impl<T> Default for Angle<T>
where
  T: Number,
{
  #[inline]
  fn default() -> Self {
    Self::ZERO
  }
}

impl<T> Debug for Angle<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Angle<{}>", self.radians)
  }
}

impl<T> Display for Angle<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(self, f)
  }
}

op!(Angle[radians], Add::add(+), AddAssign::add_assign);
op!(Angle[radians], Sub::sub(-), SubAssign::sub_assign);
op!(Angle[radians], Mul::mul(*), MulAssign::mul_assign);
op!(Angle[radians], Div::div(/), DivAssign::div_assign);
op!(Angle[radians], Rem::rem(%), RemAssign::rem_assign);

impl<T> Neg for Angle<T>
where
  T: Number + Neg<Output = T>,
{
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self::from_radians(self.to_radians().neg())
  }
}
