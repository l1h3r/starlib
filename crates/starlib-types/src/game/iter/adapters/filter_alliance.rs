use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::FusedIterator;

use crate::bridge::Alliance;
use crate::game::UnitRef;

#[derive(Clone)]
pub struct FilterAlliance<I> {
  alliance: Alliance,
  iterator: I,
}

impl<I> FilterAlliance<I> {
  #[inline]
  pub(crate) const fn new(alliance: Alliance, iterator: I) -> Self {
    Self { alliance, iterator }
  }
}

impl<I> Debug for FilterAlliance<I>
where
  I: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FilterAlliance")
      .field("alliance", &self.alliance)
      .field("iterator", &self.iterator)
      .finish()
  }
}

impl<'a, I> Iterator for FilterAlliance<I>
where
  I: Iterator<Item = UnitRef<'a>>,
{
  type Item = I::Item;

  #[inline]
  fn next(&mut self) -> Option<I::Item> {
    self.iterator.find(|unit| unit.alliance() == self.alliance)
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

    self
      .iterator
      .map(to_usize(move |unit| unit.alliance() == self.alliance))
      .sum()
  }
}

impl<'a, I> DoubleEndedIterator for FilterAlliance<I>
where
  I: Iterator<Item = UnitRef<'a>>,
  I: DoubleEndedIterator,
{
  #[inline]
  fn next_back(&mut self) -> Option<I::Item> {
    self.iterator.rfind(|unit| unit.alliance() == self.alliance)
  }
}

impl<'a, I> FusedIterator for FilterAlliance<I>
where
  I: Iterator<Item = UnitRef<'a>>,
  I: FusedIterator,
{
}
