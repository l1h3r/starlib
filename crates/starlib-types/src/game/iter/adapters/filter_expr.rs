macro_rules! define_filter {
  ($name:ident, |$unit:ident| $expr:expr) => {
    #[derive(Clone)]
    #[repr(transparent)]
    pub struct $name<I>(I);

    impl<I> $name<I> {
      #[inline]
      pub(crate) const fn new(inner: I) -> Self {
        Self(inner)
      }
    }

    impl<I> ::core::fmt::Debug for $name<I>
    where
      I: ::core::fmt::Debug,
    {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(stringify!($name)).field(&self.0).finish()
      }
    }

    impl<'a, I> ::core::iter::Iterator for $name<I>
    where
      I: ::core::iter::Iterator<Item = $crate::game::UnitRef<'a>>,
    {
      type Item = I::Item;

      #[inline]
      fn next(&mut self) -> Option<I::Item> {
        self.0.find(|$unit| $expr)
      }

      #[inline]
      fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.0.size_hint().1)
      }

      #[inline]
      fn count(self) -> usize {
        #[inline]
        fn to_usize(
          mut f: impl FnMut(&$crate::game::UnitRef<'_>) -> bool,
        ) -> impl FnMut($crate::game::UnitRef<'_>) -> usize {
          move |unit| f(&unit) as usize
        }

        self.0.map(to_usize(|$unit| $expr)).sum()
      }
    }

    impl<'a, I> ::core::iter::DoubleEndedIterator for $name<I>
    where
      I: ::core::iter::Iterator<Item = $crate::game::UnitRef<'a>>,
      I: ::core::iter::DoubleEndedIterator,
    {
      #[inline]
      fn next_back(&mut self) -> Option<I::Item> {
        self.0.rfind(|$unit| $expr)
      }
    }

    impl<'a, I> ::core::iter::FusedIterator for $name<I>
    where
      I: ::core::iter::Iterator<Item = $crate::game::UnitRef<'a>>,
      I: ::core::iter::FusedIterator,
    {
    }
  };
}

define_filter!(FilterBuildComplete, |unit| unit.build_complete());
define_filter!(FilterBuildPending, |unit| !unit.build_complete());

define_filter!(FilterMineral, |unit| unit.kind().is_mineral());
define_filter!(FilterVespene, |unit| unit.kind().is_vespene());

define_filter!(FilterTownhalls, |unit| unit.kind().is_townhall());
define_filter!(FilterSuppliers, |unit| unit.kind().is_supply());
define_filter!(FilterHarvesters, |unit| unit.kind().is_harvester());
define_filter!(FilterTechlabs, |unit| unit.kind().is_techlab());
define_filter!(FilterReactors, |unit| unit.kind().is_reactor());
define_filter!(FilterWorkers, |unit| unit.kind().is_worker());

define_filter!(FilterWatchtowers, |unit| unit.kind().is_watchtower());
define_filter!(FilterHalfSupply, |unit| unit.kind().is_half_supply());
define_filter!(FilterLarva, |unit| unit.kind().is_larva());
define_filter!(FilterEggs, |unit| unit.kind().is_egg());
