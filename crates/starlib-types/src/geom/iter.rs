use core::iter::FusedIterator;

use crate::geom::Point;
use crate::geom::Rect;
use crate::traits::Number;

// -----------------------------------------------------------------------------
// Rect Iter
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct RectIter<'a, T> {
  item: Point<T>,
  rect: &'a Rect<T>,
}

impl<'a, T> RectIter<'a, T>
where
  T: Number,
{
  #[inline]
  pub(crate) const fn new(rect: &'a Rect<T>) -> Self {
    Self {
      item: rect.min,
      rect,
    }
  }
}

impl<'a, T> Iterator for RectIter<'a, T>
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

impl<'a, T> FusedIterator for RectIter<'a, T> where T: Number {}
