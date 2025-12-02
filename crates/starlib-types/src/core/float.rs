use core::cmp::Ordering;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::LowerExp;
use core::fmt::Result as FmtResult;
use core::fmt::UpperExp;
use core::hash::Hash;
use core::hash::Hasher;
use core::num::ParseFloatError;
use core::ops::Deref;
use core::ops::DerefMut;
use core::str::FromStr;

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Float(f32);

impl Float {
  #[inline]
  pub const fn from_f32(this: f32) -> Self {
    Self(this)
  }

  #[inline]
  pub const fn into_f32(self) -> f32 {
    self.0
  }

  #[inline]
  pub fn epsilon_eq(self, other: impl Into<f32>) -> bool {
    (self.0 - other.into()).abs() < f32::EPSILON
  }
}

impl Debug for Float {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.0, f)
  }
}

impl Display for Float {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl LowerExp for Float {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    LowerExp::fmt(&self.0, f)
  }
}

impl UpperExp for Float {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    UpperExp::fmt(&self.0, f)
  }
}

impl Deref for Float {
  type Target = f32;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Float {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Hash for Float {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.to_bits().hash(state);
  }
}

impl PartialEq for Float {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.total_cmp(other).is_eq()
  }
}

impl PartialEq<f32> for Float {
  #[inline]
  fn eq(&self, other: &f32) -> bool {
    self.total_cmp(other).is_eq()
  }
}

impl PartialEq<Float> for f32 {
  #[inline]
  fn eq(&self, other: &Float) -> bool {
    self.total_cmp(other).is_eq()
  }
}

impl Eq for Float {}

impl PartialOrd for Float {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialOrd<f32> for Float {
  #[inline]
  fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
    Some(self.total_cmp(other))
  }
}

impl PartialOrd<Float> for f32 {
  #[inline]
  fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
    Some(self.total_cmp(other))
  }
}

impl Ord for Float {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.total_cmp(other)
  }
}

impl From<f32> for Float {
  #[inline]
  fn from(other: f32) -> Self {
    Self::from_f32(other)
  }
}

impl From<Float> for f32 {
  #[inline]
  fn from(other: Float) -> Self {
    Float::into_f32(other)
  }
}

impl FromStr for Float {
  type Err = ParseFloatError;

  #[inline]
  fn from_str(string: &str) -> Result<Self, Self::Err> {
    f32::from_str(string).map(Self)
  }
}

// -----------------------------------------------------------------------------
// Bin Ops
// -----------------------------------------------------------------------------

macro_rules! op {
  ($trait:ident::$method:ident($op:tt), $assign_trait:ident::$assign_method:ident) => {
    impl ::core::ops::$trait for Float {
      type Output = Self;

      #[inline]
      fn $method(self, other: Self) -> Self::Output {
        Self(self.0 $op other.0)
      }
    }

    impl ::core::ops::$trait<f32> for Float {
      type Output = Self;

      #[inline]
      fn $method(self, other: f32) -> Self::Output {
        Self(self.0 $op other)
      }
    }

    binop_ref!(Float, $trait::$method, $assign_trait::$assign_method);
    binop_ref!(Float, f32, $trait::$method, $assign_trait::$assign_method);
  };
}

op!(Add::add(+), AddAssign::add_assign);
op!(Sub::sub(-), SubAssign::sub_assign);
op!(Mul::mul(*), MulAssign::mul_assign);
op!(Div::div(/), DivAssign::div_assign);
op!(Rem::rem(%), RemAssign::rem_assign);
