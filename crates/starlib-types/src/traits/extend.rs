use std::collections::BTreeSet;
use std::collections::HashSet;
use std::hash::Hash;

use crate::traits::FromProto;

// -----------------------------------------------------------------------------
// Extend Proto Helper
// -----------------------------------------------------------------------------

pub trait ExtendProto<T> {
  fn extend_proto<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = T>;
}

impl<T, U> ExtendProto<U> for Vec<T>
where
  T: FromProto<U>,
  U: Default,
{
  #[inline]
  fn extend_proto<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = U>,
  {
    self.extend(iter.into_iter().map(T::from_proto));
  }
}

impl<T, U> ExtendProto<U> for BTreeSet<T>
where
  T: FromProto<U> + Ord,
  U: Default,
{
  #[inline]
  fn extend_proto<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = U>,
  {
    self.extend(iter.into_iter().map(T::from_proto));
  }
}

impl<T, U> ExtendProto<U> for HashSet<T>
where
  T: FromProto<U> + Eq + Hash,
  U: Default,
{
  #[inline]
  fn extend_proto<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = U>,
  {
    self.extend(iter.into_iter().map(T::from_proto));
  }
}
