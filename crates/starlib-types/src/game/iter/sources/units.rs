use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::iter::FusedIterator;
use std::slice::Iter;

use crate::bridge::Unit;
use crate::game::GameState;
use crate::game::UnitRef;

#[derive(Clone)]
pub struct Units<'a> {
  root: &'a GameState,
  iter: Iter<'a, Unit>,
}

impl<'a> Units<'a> {
  #[inline]
  pub(crate) fn new(root: &'a GameState, data: &'a [Unit]) -> Self {
    Self {
      root,
      iter: data.iter(),
    }
  }

  #[inline]
  pub fn as_slice(&self) -> &'a [Unit] {
    self.iter.as_slice()
  }
}

impl Debug for Units<'_> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("Units ")?;
    f.debug_list().entries(self.as_slice()).finish()
  }
}

impl AsRef<[Unit]> for Units<'_> {
  #[inline]
  fn as_ref(&self) -> &[Unit] {
    self.as_slice()
  }
}

impl<'a> Iterator for Units<'a> {
  type Item = UnitRef<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|unit| UnitRef::new(self.root, unit))
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }

  #[inline]
  fn count(self) -> usize {
    self.iter.count()
  }

  #[inline]
  fn nth(&mut self, n: usize) -> Option<UnitRef<'a>> {
    self.iter.nth(n).map(|unit| UnitRef::new(self.root, unit))
  }

  #[inline]
  fn last(self) -> Option<UnitRef<'a>> {
    self.iter.last().map(|unit| UnitRef::new(self.root, unit))
  }

  #[inline]
  fn fold<B, F>(self, init: B, mut f: F) -> B
  where
    F: FnMut(B, Self::Item) -> B,
  {
    self
      .iter
      .fold(init, move |acc, item| f(acc, UnitRef::new(self.root, item)))
  }

  #[inline]
  fn for_each<F>(self, mut f: F)
  where
    Self: Sized,
    F: FnMut(Self::Item),
  {
    self
      .iter
      .for_each(move |item| f(UnitRef::new(self.root, item)))
  }

  #[inline]
  fn all<F>(&mut self, mut f: F) -> bool
  where
    Self: Sized,
    F: FnMut(Self::Item) -> bool,
  {
    self.iter.all(|item| f(UnitRef::new(self.root, item)))
  }

  #[inline]
  fn any<F>(&mut self, mut f: F) -> bool
  where
    Self: Sized,
    F: FnMut(Self::Item) -> bool,
  {
    self.iter.any(|item| f(UnitRef::new(self.root, item)))
  }

  #[inline]
  fn find<F>(&mut self, mut f: F) -> Option<Self::Item>
  where
    Self: Sized,
    F: FnMut(&Self::Item) -> bool,
  {
    self
      .iter
      .by_ref()
      .map(|item| UnitRef::new(self.root, item))
      .find(|item| f(item))
  }

  #[inline]
  fn find_map<B, F>(&mut self, mut f: F) -> Option<B>
  where
    Self: Sized,
    F: FnMut(Self::Item) -> Option<B>,
  {
    self.iter.find_map(|item| f(UnitRef::new(self.root, item)))
  }

  #[inline]
  fn position<F>(&mut self, mut f: F) -> Option<usize>
  where
    Self: Sized,
    F: FnMut(Self::Item) -> bool,
  {
    self.iter.position(|item| f(UnitRef::new(self.root, item)))
  }

  #[inline]
  fn rposition<F>(&mut self, mut f: F) -> Option<usize>
  where
    Self: Sized,
    F: FnMut(Self::Item) -> bool,
  {
    self.iter.rposition(|item| f(UnitRef::new(self.root, item)))
  }
}

impl<'a> ExactSizeIterator for Units<'a> {
  #[inline]
  fn len(&self) -> usize {
    self.iter.len()
  }
}

impl<'a> DoubleEndedIterator for Units<'a> {
  #[inline]
  fn next_back(&mut self) -> Option<UnitRef<'a>> {
    self
      .iter
      .next_back()
      .map(|unit| UnitRef::new(self.root, unit))
  }

  #[inline]
  fn nth_back(&mut self, step: usize) -> Option<UnitRef<'a>> {
    self
      .iter
      .nth_back(step)
      .map(|unit| UnitRef::new(self.root, unit))
  }
}

impl<'a> FusedIterator for Units<'a> {}
