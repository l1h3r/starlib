use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Neg;

use crate::core::Float;
use crate::traits::Cos;
use crate::traits::Number;
use crate::traits::Pi;
use crate::traits::Radian;
use crate::traits::Sin;

pub type AngleF = Angle<Float>;
pub type AngleI = Angle<i32>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Angle<T> {
  radians: T,
}

impl<T> Angle<T>
where
  T: Number + Pi,
{
  pub const PI: Self = Self::new(T::PI);

  pub const PI2: Self = Self::new(T::PI2);
}

impl<T> Angle<T>
where
  T: Number,
{
  pub const ZERO: Self = Self::new(T::ZERO);

  #[inline]
  pub const fn new(radians: T) -> Self {
    Self { radians }
  }

  #[inline]
  pub fn from_degrees(degrees: T) -> Self
  where
    T: Radian,
  {
    Self::new(T::from_degrees(degrees))
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
}

impl<T> Debug for Angle<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Angle[{}]", self.radians)
  }
}

impl<T> Display for Angle<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{}]", self.radians)
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
    Self::new(self.to_radians().neg())
  }
}
