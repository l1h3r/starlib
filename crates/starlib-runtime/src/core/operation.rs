use std::fmt::Debug;

use crate::exec::Argument;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// Control Flow
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ControlFlow {
  Next = 1,
  Stop = 0,
}

impl ControlFlow {
  #[inline]
  pub const fn from_bool(value: bool) -> Self {
    if value { Self::Next } else { Self::Stop }
  }

  #[inline]
  pub const fn is_next(self) -> bool {
    matches!(self, Self::Next)
  }

  #[inline]
  pub const fn is_stop(self) -> bool {
    matches!(self, Self::Stop)
  }

  #[inline]
  pub const fn then(self, other: Self) -> Self {
    match self {
      Self::Next => other,
      Self::Stop => self,
    }
  }
}

// -----------------------------------------------------------------------------
// Dyn Operation
// -----------------------------------------------------------------------------

pub trait DynOperation: Debug + Send + Sync + 'static {
  fn setup(&self) -> Result<(), String>;
  fn run(&mut self, world: WorldRef<'_>) -> ControlFlow;
}

impl<T> DynOperation for T
where
  T: Operation,
{
  #[inline]
  fn setup(&self) -> Result<(), String> {
    <T as Operation>::setup(self)
  }

  #[inline]
  fn run(&mut self, world: WorldRef<'_>) -> ControlFlow {
    <T as Operation>::run(self, <T::Argv as Argument>::fetch(world))
  }
}

// -----------------------------------------------------------------------------
// Operation
// -----------------------------------------------------------------------------

pub trait Operation: Debug + Send + Sync + 'static {
  type Argv: Argument;

  fn setup(&self) -> Result<(), String> {
    Ok(()) // do nothing
  }

  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) -> ControlFlow;

  #[inline]
  fn boxed(self) -> Box<dyn DynOperation + Send + Sync>
  where
    Self: Sized,
  {
    Box::new(self)
  }
}

impl Operation for Box<dyn DynOperation + Send + Sync> {
  type Argv = WorldRef<'static>;

  #[inline]
  fn setup(&self) -> Result<(), String> {
    (**self).setup()
  }

  #[inline]
  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) -> ControlFlow {
    (**self).run(input)
  }

  #[inline]
  fn boxed(self) -> Box<dyn DynOperation + Send + Sync> {
    self
  }
}

// -----------------------------------------------------------------------------
// IntoOperation
// -----------------------------------------------------------------------------

pub trait IntoOperation {
  type Item: Operation;

  fn into_operation(self) -> Self::Item;

  #[inline]
  fn into_boxed_operation(self) -> Box<dyn DynOperation>
  where
    Self: Sized,
  {
    self.into_operation().boxed()
  }
}

impl<T> IntoOperation for T
where
  T: Operation,
{
  type Item = T;

  #[inline]
  fn into_operation(self) -> Self::Item {
    self
  }
}
