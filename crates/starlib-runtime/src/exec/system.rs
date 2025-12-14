use std::any::TypeId;
use std::any::type_name;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::exec::Argument;
use crate::exec::World;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// Systems
// -----------------------------------------------------------------------------

pub struct Systems {
  index: HashMap<TypeId, usize>,
  inner: Vec<Box<dyn DynSystem>>,
}

impl Systems {
  #[inline]
  pub fn new() -> Self {
    Self {
      index: HashMap::new(),
      inner: Vec::new(),
    }
  }

  #[inline]
  pub fn insert<S>(&mut self, system: S)
  where
    S: IntoSystem,
  {
    if let Entry::Vacant(entry) = self.index.entry(TypeId::of::<S::Item>()) {
      entry.insert(self.inner.len());
    } else {
      panic!("system entry already exists: {}", type_name::<S::Item>());
    }

    self.inner.push(system.into_boxed_system());
  }

  #[inline]
  pub fn setup(&mut self, world: &mut World) -> Result<(), String> {
    for system in self.inner.iter_mut() {
      system.setup(world)?;
    }

    Ok(())
  }

  #[inline]
  pub fn run(&mut self, world: WorldRef<'_>) {
    for system in self.inner.iter_mut() {
      system.run(world);
    }
  }
}

// -----------------------------------------------------------------------------
// Dyn System
// -----------------------------------------------------------------------------

pub trait DynSystem {
  fn run(&mut self, world: WorldRef<'_>);
  fn setup(&mut self, world: &mut World) -> Result<(), String>;
}

impl<T> DynSystem for T
where
  T: System,
{
  #[inline]
  fn run(&mut self, world: WorldRef<'_>) {
    <T as System>::run(self, <T::Argv as Argument>::fetch(world))
  }

  #[inline]
  fn setup(&mut self, world: &mut World)  -> Result<(), String> {
    <T as System>::setup(self, world)
  }
}

// -----------------------------------------------------------------------------
// System
// -----------------------------------------------------------------------------

pub trait System: Send + Sync + 'static {
  type Argv: Argument;

  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>);

  #[inline]
  fn setup(&mut self, world: &mut World) -> Result<(), String> {
    <Self::Argv as Argument>::setup(world)
  }

  #[inline]
  fn boxed(self) -> Box<dyn DynSystem>
  where
    Self: Sized,
  {
    Box::new(self)
  }
}

impl System for Box<dyn DynSystem + Send + Sync> {
  type Argv = WorldRef<'static>;

  #[inline]
  fn setup(&mut self, world: &mut World) -> Result<(), String> {
    (**self).setup(world)
  }

  #[inline]
  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) {
    (**self).run(input)
  }

  #[inline]
  fn boxed(self) -> Box<dyn DynSystem> {
    self
  }
}

// -----------------------------------------------------------------------------
// IntoSystem
// -----------------------------------------------------------------------------

pub trait IntoSystem {
  type Item: System;

  fn into_system(self) -> Self::Item;

  #[inline]
  fn into_boxed_system(self) -> Box<dyn DynSystem>
  where
    Self: Sized,
  {
    self.into_system().boxed()
  }
}

impl<T> IntoSystem for T
where
  T: System,
{
  type Item = T;

  #[inline]
  fn into_system(self) -> Self::Item {
    self
  }
}
