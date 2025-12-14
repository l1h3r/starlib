use crate::core::ControlFlow;
use crate::core::DynOperation;
use crate::core::IntoOperation;
use crate::core::Operation;
use crate::exec::Argument;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// Parallel List
// -----------------------------------------------------------------------------

define_list!(
  Par,
  Operation,
  DynOperation,
  IntoOperation::into_boxed_operation,
);

impl Operation for Par {
  type Argv = WorldRef<'static>;

  #[inline]
  fn setup(&self) -> Result<(), String> {
    for operation in self.iter() {
      operation.setup()?;
    }

    Ok(())
  }

  #[inline]
  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) -> ControlFlow {
    let mut flow: ControlFlow = ControlFlow::Next;

    for operation in self.iter_mut() {
      flow = flow.then(operation.run(input));
    }

    flow
  }
}

#[macro_export]
macro_rules! par {
  () => {
    $crate::core::Par::new()
  };
  ($($expr:expr),+ $(,)?) => {
    $crate::core::Par::from_iter([
      $($crate::core::Operation::boxed($expr),)+
    ])
  };
}

pub use par;

// -----------------------------------------------------------------------------
// Sequential List
// -----------------------------------------------------------------------------

define_list!(
  Seq,
  Operation,
  DynOperation,
  IntoOperation::into_boxed_operation,
);

impl Operation for Seq {
  type Argv = WorldRef<'static>;

  #[inline]
  fn setup(&self) -> Result<(), String> {
    for operation in self.iter() {
      operation.setup()?;
    }

    Ok(())
  }

  #[inline]
  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) -> ControlFlow {
    for operation in self.iter_mut() {
      match operation.run(input) {
        ControlFlow::Next => continue,
        ControlFlow::Stop => return ControlFlow::Stop,
      }
    }

    ControlFlow::Next
  }
}

#[macro_export]
macro_rules! seq {
  () => {
    $crate::core::Seq::new()
  };
  ($($expr:expr),+ $(,)?) => {
    $crate::core::Seq::from_iter([
      $($crate::core::Operation::boxed($expr),)+
    ])
  };
}

pub use seq;
