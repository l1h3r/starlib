use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::core::ControlFlow;
use crate::core::Operation;
use crate::exec::Argument;
use crate::exec::StateRef;
use crate::game::Cmp;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SupplyLeft<T>(pub T);

impl<T> Debug for SupplyLeft<T>
where
  T: Cmp<u32>,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "SupplyLeft({})", self.0)
  }
}

impl<T> Operation for SupplyLeft<T>
where
  T: Cmp<u32>,
{
  type Argv = StateRef<'static>;

  #[inline]
  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) -> ControlFlow {
    ControlFlow::from_bool(self.0.cmp(&input.food_remaining()))
  }
}

#[macro_export]
macro_rules! SupplyLeft {
  (== $inner:expr $(,)?) => {
    $crate::game::SupplyLeft($crate::game::Eq($inner))
  };
  (!= $inner:expr $(,)?) => {
    $crate::game::SupplyLeft($crate::game::Ne($inner))
  };
  (< $inner:expr $(,)?) => {
    $crate::game::SupplyLeft($crate::game::Lt($inner))
  };
  (<= $inner:expr $(,)?) => {
    $crate::game::SupplyLeft($crate::game::Le($inner))
  };
  (> $inner:expr $(,)?) => {
    $crate::game::SupplyLeft($crate::game::Gt($inner))
  };
  (>= $inner:expr $(,)?) => {
    $crate::game::SupplyLeft($crate::game::Ge($inner))
  };
}
