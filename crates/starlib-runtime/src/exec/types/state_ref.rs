use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Deref;

use starlib_types::GameState;

use crate::exec::Argument;
use crate::exec::World;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// State Ref
// -----------------------------------------------------------------------------

#[repr(transparent)]
pub struct StateRef<'world>(&'world GameState);

impl Debug for StateRef<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&**self, f)
  }
}

impl Deref for StateRef<'_> {
  type Target = GameState;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl AsRef<GameState> for StateRef<'_> {
  #[inline]
  fn as_ref(&self) -> &GameState {
    self.deref()
  }
}

impl Argument for StateRef<'_> {
  type Item<'world> = StateRef<'world>;

  #[inline]
  fn setup(_world: &mut World) -> Result<(), String> {
    Ok(()) // do nothing - game state is late-binding
  }

  #[inline]
  fn fetch<'world>(world: WorldRef<'world>) -> <Self as Argument>::Item<'world> {
    let Some(state) = world.into_inner().state() else {
      panic!("game state not found");
    };

    StateRef(state)
  }
}
