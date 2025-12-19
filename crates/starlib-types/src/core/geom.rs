use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::core::Float;
use crate::traits::Number;

// -----------------------------------------------------------------------------
// Point
// -----------------------------------------------------------------------------

pub type PointF = Point<Float>;
pub type PointI = Point<i32>;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  #[inline]
  pub fn into_3d(self, z: T) -> Point3D<T> {
    Point3D::new(self.x, self.y, z)
  }
}

impl<T> Point<T>
where
  T: Number,
{
  #[inline]
  pub fn dot(self, other: Self) -> T {
    self.x.mul(other.x).add(self.y.mul(other.y))
  }

  #[inline]
  pub fn length(self) -> T {
    self.dot(self).sqrt()
  }

  #[inline]
  pub fn distance(self, other: Self) -> T {
    (self - other).length()
  }

  #[inline]
  pub fn towards(self, other: Self, offset: T) -> Self {
    self + (other - self) / self.distance(other) * offset
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

// -----------------------------------------------------------------------------
// Bin Ops
// -----------------------------------------------------------------------------

macro_rules! op {
  ($trait:ident::$method:ident($op:tt), $assign_trait:ident::$assign_method:ident) => {
    impl<T> ::core::ops::$trait for Point<T>
    where
      T: $crate::traits::Number,
    {
      type Output = Self;

      #[inline]
      fn $method(self, other: Self) -> Self::Output {
        Point {
          x: self.x $op other.x,
          y: self.y $op other.y,
        }
      }
    }

    impl<T> ::core::ops::$trait<T> for Point<T>
    where
      T: $crate::traits::Number,
    {
      type Output = Self;

      #[inline]
      fn $method(self, other: T) -> Self::Output {
        Point {
          x: self.x $op other,
          y: self.y $op other,
        }
      }
    }

    binop_ref!(Point<T: $crate::traits::Number>, $trait::$method, $assign_trait::$assign_method);
    binop_ref!(Point<T: $crate::traits::Number>, T, $trait::$method, $assign_trait::$assign_method);
  };
}

op!(Add::add(+), AddAssign::add_assign);
op!(Sub::sub(-), SubAssign::sub_assign);
op!(Mul::mul(*), MulAssign::mul_assign);
op!(Div::div(/), DivAssign::div_assign);
op!(Rem::rem(%), RemAssign::rem_assign);

// -----------------------------------------------------------------------------
// Point3D
// -----------------------------------------------------------------------------

pub type Point3DF = Point3D<Float>;
pub type Point3DI = Point3D<i32>;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Point3D<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Point3D<T> {
  #[inline]
  pub const fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }

  #[inline]
  pub fn into_2d(self) -> Point<T> {
    Point::new(self.x, self.y)
  }
}

impl<T> Debug for Point3D<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Point3D[{},{},{}]", self.x, self.y, self.z)
  }
}

impl<T> Display for Point3D<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{},{},{}]", self.x, self.y, self.z)
  }
}

// -----------------------------------------------------------------------------
// Size
// -----------------------------------------------------------------------------

pub type SizeF = Size<Float>;
pub type SizeI = Size<i32>;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Size<T> {
  pub w: T,
  pub h: T,
}

impl<T> Size<T> {
  #[inline]
  pub const fn new(w: T, h: T) -> Self {
    Self { w, h }
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

// -----------------------------------------------------------------------------
// Rect
// -----------------------------------------------------------------------------

pub type RectF = Rect<Float>;
pub type RectI = Rect<i32>;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Rect<T> {
  pub x: T,
  pub y: T,
  pub w: T,
  pub h: T,
}

impl<T> Rect<T> {
  #[inline]
  pub const fn new(x: T, y: T, w: T, h: T) -> Self {
    Self { x, y, w, h }
  }

  #[inline]
  pub fn center(self) -> Point<T>
  where
    T: Number,
  {
    Point::new(self.x + self.w / T::TWO, self.y + self.h / T::TWO)
  }
}

impl<T> Debug for Rect<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Rect[{}x{} @ {},{}]", self.w, self.h, self.x, self.y)
  }
}

impl<T> Display for Rect<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{}x{} @ {},{}]", self.w, self.h, self.x, self.y)
  }
}
