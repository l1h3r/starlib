use std::marker::PhantomData;

use crate::exec::World;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// Argument
// -----------------------------------------------------------------------------

pub trait Argument {
  type Item<'world>: Argument;

  fn setup(world: &mut World) -> Result<(), String>;

  fn fetch<'world>(world: WorldRef<'world>) -> Self::Item<'world>;
}

// -----------------------------------------------------------------------------
// Argument (tuple)
// -----------------------------------------------------------------------------

macro_rules! param {
  ($($name:ident),*) => {
    impl<$($name: Argument),*> Argument for ($($name,)*) {
      type Item<'world> = ($(<$name as Argument>::Item<'world>,)*);

      #[allow(unused_variables, reason = "macro-generated code")]
      #[inline]
      fn setup(world: &mut World) -> Result<(), String> {
        $(
          let _: () = <$name as Argument>::setup(world)?;
        )*

        Ok(())
      }

      #[allow(unused_variables, reason = "macro-generated code")]
      #[inline]
      fn fetch<'world>(world: WorldRef<'world>) -> Self::Item<'world> {
        ($(<$name as Argument>::fetch(world),)*)
      }
    }
  };
}

param!();
param!(T1);
param!(T1, T2);
param!(T1, T2, T3);
param!(T1, T2, T3, T4);
param!(T1, T2, T3, T4, T5);
param!(T1, T2, T3, T4, T5, T6);
param!(T1, T2, T3, T4, T5, T6, T7);
param!(T1, T2, T3, T4, T5, T6, T7, T8);

// -----------------------------------------------------------------------------
// Argument (PhantomData)
// -----------------------------------------------------------------------------

impl<T> Argument for PhantomData<T>
where
  T: ?Sized,
{
  type Item<'world> = Self;

  #[inline]
  fn setup(_world: &mut World) -> Result<(), String> {
    Ok(()) // do nothing
  }

  #[inline]
  fn fetch<'world>(_world: WorldRef<'world>) -> Self::Item<'world> {
    PhantomData
  }
}
