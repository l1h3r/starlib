use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use starlib_types::UnitTypeId;

use crate::core::ControlFlow;
use crate::core::Operation;
use crate::exec::Argument;
use crate::exec::Mut;
use crate::exec::Ref;
use crate::exec::StateRef;
use crate::game::CostTracker;
use crate::game::UnitTracker;

#[derive(Clone, Copy)]
pub struct Build(pub UnitTypeId, pub usize);

impl Debug for Build {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Build({}, {})", self.0, self.1)
  }
}

impl Operation for Build {
  type Argv = (
    StateRef<'static>,
    Mut<'static, CostTracker>,
    Ref<'static, UnitTracker>,
  );

  #[inline]
  fn run(&mut self, (state, mut cost, unit): <Self::Argv as Argument>::Item<'_>) -> ControlFlow {
    ControlFlow::Stop
  }
}

#[macro_export]
macro_rules! Build {
  ($utype:expr $(,)?) => {
    $crate::game::Build($utype, 1)
  };
  ($utype:expr, $count:expr $(,)?) => {
    $crate::game::Build($utype, $count)
  };
}
