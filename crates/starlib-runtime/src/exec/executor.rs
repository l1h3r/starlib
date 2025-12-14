use std::sync::Arc;

use starlib_types::GameState;

use crate::exec::Resource;
use crate::exec::System;
use crate::exec::Systems;
use crate::exec::World;

// -----------------------------------------------------------------------------
// Executor
// -----------------------------------------------------------------------------

pub struct Executor {
  queue: Systems,
  world: World,
}

impl Executor {
  #[inline]
  pub fn new() -> Self {
    Self {
      queue: Systems::new(),
      world: World::new(),
    }
  }

  #[inline]
  pub fn setup(&mut self) -> Result<(), String> {
    self.queue.setup(&mut self.world)
  }

  #[inline]
  pub fn run(&mut self, state: Arc<GameState>) {
    self.world.for_each(|resource| {
      resource.tick(&state);
    });

    self.world.with(state, |world| {
      self.queue.run(world.world_ref());
    });
  }

  #[inline]
  pub fn insert_resource<R>(&mut self, resource: R)
  where
    R: Resource,
  {
    self.world.insert(resource);
  }

  #[inline]
  pub fn insert_system<S>(&mut self, system: S)
  where
    S: System,
  {
    self.queue.insert(system);
  }
}
