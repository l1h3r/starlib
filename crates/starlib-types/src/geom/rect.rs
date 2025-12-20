use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::FusedIterator;

use crate::core::Float;
use crate::geom::Point;
use crate::geom::Size;
use crate::traits::Number;

pub type RectF = Rect<Float>;
pub type RectI = Rect<i32>;

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
  pub fn from_center(point: Point<T>, radius: T) -> Self {
    Self::new(point - radius, point + radius)
  }

  #[inline]
  pub fn map<F>(self, f: F) -> Self
  where
    F: Fn(Point<T>) -> Point<T>,
  {
    Self::new(f(self.min), f(self.max))
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
  pub fn intersection(self, other: Self) -> Option<Self> {
    let this: Self = self.intersection_unchecked(other);

    if this.is_empty() {
      return None;
    }

    Some(this)
  }

  #[inline]
  pub fn intersection_unchecked(self, other: Self) -> Self {
    Self {
      min: Point::new(self.min.x.max(other.min.x), self.min.y.max(other.min.y)),
      max: Point::new(self.max.x.min(other.max.x), self.max.y.min(other.max.y)),
    }
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
  pub fn union_unchecked(self, other: Self) -> Self {
    Self {
      min: Point::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
      max: Point::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
    }
  }

  #[inline]
  pub fn inflate(self, width: T, height: T) -> Self {
    Self {
      min: Point::new(self.min.x - width, self.min.y - height),
      max: Point::new(self.max.x + width, self.max.y + height),
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
  pub fn round(self) -> Self {
    Self::new(self.min.round(), self.max.round())
  }

  #[inline]
  pub fn round_expand(self) -> Self {
    Self::new(self.min.floor(), self.max.ceil())
  }

  #[inline]
  pub fn round_shrink(self) -> Self {
    Self::new(self.min.ceil(), self.max.floor())
  }

  #[inline]
  pub fn to_f32(self) -> Rect<f32> {
    Rect::new(self.min.to_f32(), self.max.to_f32())
  }

  #[inline]
  pub fn to_i32(self) -> Rect<i32> {
    Rect::new(self.min.to_i32(), self.max.to_i32())
  }

  #[inline]
  pub fn to_float(self) -> Rect<Float> {
    Rect::new(self.min.to_float(), self.max.to_float())
  }

  #[inline]
  pub fn iter(&self) -> Points<'_, T> {
    Points::new(self)
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
    write!(f, "Rect[{} @ {}]", self.size(), self.min)
  }
}

impl<T> Display for Rect<T>
where
  T: Number,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{} @ {}]", self.size(), self.min)
  }
}

// -----------------------------------------------------------------------------
// Rect Iter
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Points<'a, T> {
  item: Point<T>,
  rect: &'a Rect<T>,
}

impl<'a, T> Points<'a, T>
where
  T: Number,
{
  #[inline]
  pub const fn new(rect: &'a Rect<T>) -> Self {
    Self {
      item: rect.min,
      rect,
    }
  }
}

impl<'a, T> Iterator for Points<'a, T>
where
  T: Number,
{
  type Item = Point<T>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let next: Point<T> = self.item;

    if next.y > self.rect.max.y {
      return None;
    }

    self.item.x = self.item.x + T::ONE;

    if self.item.x > self.rect.max.x {
      self.item.x = self.rect.min.x;
      self.item.y = self.item.y + T::ONE;
    }

    Some(next)
  }
}

impl<'a, T> FusedIterator for Points<'a, T> where T: Number {}

impl<'a, T> IntoIterator for &'a Rect<T>
where
  T: Number,
{
  type Item = Point<T>;
  type IntoIter = Points<'a, T>;

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
  type IntoIter = Points<'a, T>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
