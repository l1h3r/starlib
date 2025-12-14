use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::hash::Hash;

use starlib_types::UnitTag;

#[repr(transparent)]
pub struct TagSet<T> {
  inner: HashMap<T, HashSet<UnitTag>>,
}

impl<T> TagSet<T> {
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: HashMap::new(),
    }
  }

  #[inline]
  pub fn clear(&mut self) {
    for tags in self.inner.values_mut() {
      tags.clear();
    }
  }

  #[inline]
  pub fn count<Q>(&self, id: &Q) -> usize
  where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
  {
    self.inner.get(id).map(HashSet::len).unwrap_or(0)
  }

  #[inline]
  pub fn add(&mut self, kind: T, utag: UnitTag)
  where
    T: Hash + Eq,
  {
    self.inner.entry(kind).or_default().insert(utag);
  }
}

impl<T> Debug for TagSet<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(&self.inner, f)
  }
}
