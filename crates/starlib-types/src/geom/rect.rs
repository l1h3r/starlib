use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

use crate::core::Float;
use crate::geom::Point;
use crate::geom::RectIter;
use crate::geom::Size;
use crate::traits::Ceil;
use crate::traits::Floor;
use crate::traits::Number;
use crate::traits::Round;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Rect<T> {
  pub min: Point<T>,
  pub max: Point<T>,
}

impl<T> Rect<T>
where
  T: Number,
{
  pub const ZERO: Self = Self::new(Point::ZERO, Point::ZERO);

  #[inline]
  pub const fn new(min: Point<T>, max: Point<T>) -> Self {
    Self { min, max }
  }

  #[inline]
  pub fn from_origin(origin: Point<T>, size: Size<T>) -> Self {
    Self::new(origin, Point::new(origin.x + size.w, origin.y + size.h))
  }

  #[inline]
  pub fn from_center(center: Point<T>, radius: T) -> Self {
    Self::new(center - radius, center + radius)
  }

  #[inline]
  pub const fn from_array([min, max]: [Point<T>; 2]) -> Self {
    Self::new(min, max)
  }

  #[inline]
  pub fn map<F>(self, f: F) -> Self
  where
    F: Fn(Point<T>) -> Point<T>,
  {
    Self::new(f(self.min), f(self.max))
  }

  #[inline]
  pub fn iter(&self) -> RectIter<'_, T> {
    RectIter::new(self)
  }

  #[inline]
  pub fn width(self) -> T {
    self.max.x - self.min.x
  }

  #[inline]
  pub fn height(self) -> T {
    self.max.y - self.min.y
  }

  #[inline]
  pub fn size(self) -> Size<T> {
    Size::from_array((self.max - self.min).to_array())
  }

  #[inline]
  pub fn area(self) -> T {
    self.size().area()
  }

  #[inline]
  pub fn center(self) -> Point<T> {
    (self.min + self.max) / T::TWO
  }

  #[inline]
  pub fn is_empty(self) -> bool {
    !(self.max.x > self.min.x && self.max.y > self.min.y)
  }

  #[inline]
  pub fn contains(self, other: Self) -> bool {
    other.is_empty() || self.contains_unchecked(other)
  }

  #[inline]
  pub fn contains_unchecked(self, other: Self) -> bool {
    (self.min.x <= other.min.x)
      & (other.max.x <= self.max.x)
      & (self.min.y <= other.min.y)
      & (other.max.y <= self.max.y)
  }

  #[inline]
  pub fn contains_exclusive(self, point: Point<T>) -> bool {
    (self.min.x <= point.x)
      & (point.x < self.max.x)
      & (self.min.y <= point.y)
      & (point.y < self.max.y)
  }

  #[inline]
  pub fn contains_inclusive(self, point: Point<T>) -> bool {
    (self.min.x <= point.x)
      & (point.x <= self.max.x)
      & (self.min.y <= point.y)
      & (point.y <= self.max.y)
  }

  #[inline]
  pub fn intersects(self, other: Self) -> bool {
    (self.min.x < other.max.x)
      & (self.max.x > other.min.x)
      & (self.min.y < other.max.y)
      & (self.max.y > other.min.y)
  }

  #[inline]
  pub fn round(self) -> Self
  where
    T: Round,
  {
    Self::new(self.min.round(), self.max.round())
  }

  #[inline]
  pub fn round_expand(self) -> Self
  where
    T: Ceil + Floor,
  {
    Self::new(self.min.floor(), self.max.ceil())
  }

  #[inline]
  pub fn round_shrink(self) -> Self
  where
    T: Ceil + Floor,
  {
    Self::new(self.min.ceil(), self.max.floor())
  }

  #[inline]
  pub fn intersection(self, other: Self) -> Option<Self> {
    let this: Self = self.intersection_unchecked(other);

    if this.is_empty() {
      return None;
    }

    Some(this)
  }

  #[inline]
  pub fn union(self, other: Self) -> Self {
    if other.is_empty() {
      return self;
    }

    if self.is_empty() {
      return other;
    }

    self.union_unchecked(other)
  }

  #[inline]
  pub fn intersection_unchecked(self, other: Self) -> Self {
    Self {
      min: Point::new(self.min.x.max(other.min.x), self.min.y.max(other.min.y)),
      max: Point::new(self.max.x.min(other.max.x), self.max.y.min(other.max.y)),
    }
  }

  #[inline]
  pub fn union_unchecked(self, other: Self) -> Self {
    Self {
      min: Point::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
      max: Point::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
    }
  }

  #[inline]
  pub fn inflate(self, w: T, h: T) -> Self {
    Self {
      min: Point::new(self.min.x - w, self.min.y - h),
      max: Point::new(self.max.x + w, self.max.y + h),
    }
  }

  #[inline]
  pub fn scale(self, x: T, y: T) -> Self {
    Self {
      min: Point::new(self.min.x * x, self.min.y * y),
      max: Point::new(self.max.x * x, self.max.y * y),
    }
  }

  #[inline]
  pub fn translate(self, by: Point<T>) -> Self {
    Self::new(self.min + by, self.max + by)
  }

  #[inline]
  pub const fn to_array(self) -> [Point<T>; 2] {
    [self.min, self.max]
  }

  #[inline]
  pub fn to_i32(self) -> Rect<i32> {
    Rect::new(self.min.to_i32(), self.max.to_i32())
  }

  #[inline]
  pub fn to_f32(self) -> Rect<f32> {
    Rect::new(self.min.to_f32(), self.max.to_f32())
  }

  #[inline]
  pub fn to_float(self) -> Rect<Float> {
    Rect::new(self.min.to_float(), self.max.to_float())
  }
}

impl<T> Default for Rect<T>
where
  T: Number,
{
  #[inline]
  fn default() -> Self {
    Self::ZERO
  }
}

impl<T> Debug for Rect<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "Rect<{}x{} @ {},{}>",
      self.width(),
      self.height(),
      self.min.x,
      self.min.y
    )
  }
}

impl<T> Display for Rect<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(self, f)
  }
}

impl<'a, T> IntoIterator for &'a Rect<T>
where
  T: Number,
{
  type Item = Point<T>;
  type IntoIter = RectIter<'a, T>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Rect<T>
where
  T: Number,
{
  type Item = Point<T>;
  type IntoIter = RectIter<'a, T>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
