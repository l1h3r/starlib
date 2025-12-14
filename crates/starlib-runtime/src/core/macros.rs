macro_rules! define_list {
  ($name:ident, $trait:ident, $dyn_trait:ident, $into:ident::$into_method:ident $(,)?) => {
    #[repr(transparent)]
    pub struct $name(Vec<Box<dyn $dyn_trait>>);

    const _: () = {
      type Item = Box<dyn $dyn_trait>;

      impl $name {
        #[inline]
        pub const fn new() -> Self {
          Self(Vec::new())
        }

        #[inline]
        pub fn with_capacity(capacity: usize) -> Self {
          Self(Vec::with_capacity(capacity))
        }

        #[inline]
        pub const fn as_slice(&self) -> &[Item] {
          self.0.as_slice()
        }

        #[inline]
        pub fn push<T>(&mut self, value: T)
        where
          T: $into,
        {
          self.extend(Some(value));
        }

        #[inline]
        fn _iter<I, T>(iter: I) -> impl Iterator<Item = Item>
        where
          I: ::core::iter::IntoIterator<Item = T>,
          T: $into,
        {
          iter.into_iter().map($into::$into_method)
        }
      }

      impl ::core::default::Default for $name {
        #[inline]
        fn default() -> Self {
          Self(Vec::new())
        }
      }

      impl ::core::fmt::Debug for $name {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.write_str(concat!(stringify!($name), " "))?;
          f.debug_list().entries(self.0.iter()).finish()
        }
      }

      impl ::core::ops::Deref for $name {
        type Target = [Item];

        #[inline]
        fn deref(&self) -> &Self::Target {
          self.0.as_slice()
        }
      }

      impl ::core::ops::DerefMut for $name {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
          self.0.as_mut_slice()
        }
      }

      impl ::core::convert::AsRef<[Item]> for $name {
        #[inline]
        fn as_ref(&self) -> &[Item] {
          self.0.as_ref()
        }
      }

      impl ::core::convert::AsMut<[Item]> for $name {
        #[inline]
        fn as_mut(&mut self) -> &mut [Item] {
          self.0.as_mut()
        }
      }

      impl ::core::borrow::Borrow<[Item]> for $name {
        #[inline]
        fn borrow(&self) -> &[Item] {
          self.0.borrow()
        }
      }

      impl ::core::borrow::BorrowMut<[Item]> for $name {
        #[inline]
        fn borrow_mut(&mut self) -> &mut [Item] {
          self.0.borrow_mut()
        }
      }

      impl<I> ::core::ops::Index<I> for $name
      where
        I: ::core::slice::SliceIndex<[Item]>,
      {
        type Output = <I as ::core::slice::SliceIndex<[Item]>>::Output;

        #[inline]
        fn index(&self, index: I) -> &Self::Output {
          self.0.index(index)
        }
      }

      impl<I> ::core::ops::IndexMut<I> for $name
      where
        I: ::core::slice::SliceIndex<[Item]>,
      {
        #[inline]
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
          self.0.index_mut(index)
        }
      }

      impl<T> ::core::iter::FromIterator<T> for $name
      where
        T: $into,
      {
        #[inline]
        fn from_iter<I>(iter: I) -> Self
        where
          I: IntoIterator<Item = T>,
        {
          Self(Self::_iter(iter).collect())
        }
      }

      impl<T> ::core::iter::Extend<T> for $name
      where
        T: $into,
      {
        #[inline]
        fn extend<I>(&mut self, iter: I)
        where
          I: ::core::iter::IntoIterator<Item = T>,
        {
          self.0.extend(Self::_iter(iter));
        }
      }

      impl ::core::iter::IntoIterator for $name {
        type Item = Item;
        type IntoIter = ::std::vec::IntoIter<Self::Item>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
          self.0.into_iter()
        }
      }

      impl<'a> ::core::iter::IntoIterator for &'a $name {
        type Item = &'a Item;
        type IntoIter = ::core::slice::Iter<'a, Item>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
          self.0.iter()
        }
      }

      impl<'a> ::core::iter::IntoIterator for &'a mut $name {
        type Item = &'a mut Item;
        type IntoIter = ::core::slice::IterMut<'a, Item>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
          self.0.iter_mut()
        }
      }
    };
  };
}
