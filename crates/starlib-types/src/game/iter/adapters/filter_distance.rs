use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::FusedIterator;

use crate::game::UnitRef;
use crate::geom::PointF;

#[derive(Clone)]
pub struct FilterDistance<I> {
  to_point: PointF,
  distance: f32,
  iterator: I,
}

impl<I> FilterDistance<I> {
  #[inline]
  pub(crate) const fn new(to_point: PointF, distance: f32, iterator: I) -> Self {
    Self {
      to_point,
      distance,
      iterator,
    }
  }
}

impl<I> Debug for FilterDistance<I>
where
  I: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FilterDistance")
      .field("to_point", &self.to_point)
      .field("distance", &self.distance)
      .field("iterator", &self.iterator)
      .finish()
  }
}

impl<'a, I> Iterator for FilterDistance<I>
where
  I: Iterator<Item = UnitRef<'a>>,
{
  type Item = I::Item;

  #[inline]
  fn next(&mut self) -> Option<I::Item> {
    self
      .iterator
      .find(|unit| unit.in_range(self.to_point, self.distance))
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.iterator.size_hint().1)
  }

  #[inline]
  fn count(self) -> usize {
    #[inline]
    fn to_usize(mut f: impl FnMut(&UnitRef<'_>) -> bool) -> impl FnMut(UnitRef<'_>) -> usize {
      move |unit| f(&unit) as usize
    }

    let map_fn: _ = to_usize(move |unit| unit.in_range(self.to_point, self.distance));

    self.iterator.map(map_fn).sum()
  }
}

impl<'a, I> DoubleEndedIterator for FilterDistance<I>
where
  I: Iterator<Item = UnitRef<'a>>,
  I: DoubleEndedIterator,
{
  #[inline]
  fn next_back(&mut self) -> Option<I::Item> {
    self
      .iterator
      .rfind(|unit| unit.in_range(self.to_point, self.distance))
  }
}

impl<'a, I> FusedIterator for FilterDistance<I>
where
  I: Iterator<Item = UnitRef<'a>>,
  I: FusedIterator,
{
}
