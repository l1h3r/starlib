use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::FusedIterator;

use crate::game::UnitRef;
use crate::stable::UnitTypeId;

#[derive(Clone)]
pub struct FilterBuilder<I> {
  kind: UnitTypeId,
  iterator: I,
}

impl<I> FilterBuilder<I> {
  #[inline]
  pub(crate) const fn new(kind: UnitTypeId, iterator: I) -> Self {
    Self { kind, iterator }
  }
}

impl<I> Debug for FilterBuilder<I>
where
  I: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FilterBuilder")
      .field("kind", &self.kind)
      .field("iterator", &self.iterator)
      .finish()
  }
}

impl<'a, I> Iterator for FilterBuilder<I>
where
  I: Iterator<Item = UnitRef<'a>>,
{
  type Item = I::Item;

  #[inline]
  fn next(&mut self) -> Option<I::Item> {
    self.iterator.find(|unit| unit.kind().is_builder(self.kind))
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
      .map(to_usize(move |unit| unit.kind().is_builder(self.kind)))
      .sum()
  }
}

impl<'a, I> DoubleEndedIterator for FilterBuilder<I>
where
  I: Iterator<Item = UnitRef<'a>>,
  I: DoubleEndedIterator,
{
  #[inline]
  fn next_back(&mut self) -> Option<I::Item> {
    self
      .iterator
      .rfind(|unit| unit.kind().is_builder(self.kind))
  }
}

impl<'a, I> FusedIterator for FilterBuilder<I>
where
  I: Iterator<Item = UnitRef<'a>>,
  I: FusedIterator,
{
}
