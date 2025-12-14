use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use starlib_types::UnitTypeId;

use crate::core::ControlFlow;
use crate::core::Operation;
use crate::exec::Argument;
use crate::exec::Ref;
use crate::game::UnitTracker;

#[derive(Clone, Copy)]
pub struct Exists(pub UnitTypeId, pub usize);

impl Debug for Exists {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Exists({}, {})", self.0, self.1)
  }
}

impl Operation for Exists {
  type Argv = Ref<'static, UnitTracker>;

  #[inline]
  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) -> ControlFlow {
    ControlFlow::from_bool(input.count_current(self.0) >= self.1)
  }
}

#[macro_export]
macro_rules! Exists {
  ($utype:expr $(,)?) => {
    $crate::game::Exists($utype, 1)
  };
  ($utype:expr, $count:expr $(,)?) => {
    $crate::game::Exists($utype, $count)
  };
}
