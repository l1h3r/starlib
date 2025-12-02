use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fmt::DebugStruct;
use std::fmt::Formatter;
use std::fmt::Result;
use std::sync::Mutex;
use std::sync::TryLockError;

use starlib_proto::Action;
use starlib_proto::ActionRaw;
use starlib_proto::ActionRawUnitCommand;
use starlib_proto::ActionRawVariant;

use crate::bridge::Target;
use crate::core::UnitTag;
use crate::stable::AbilityId;
use crate::traits::IntoProto;

#[inline]
fn to_action((ability, target, queue): (AbilityId, Target, bool), tags: Vec<UnitTag>) -> Action {
  Action {
    action_raw: Some(ActionRaw {
      action: Some(ActionRawVariant::UnitCommand(ActionRawUnitCommand {
        ability_id: ability.into_proto(),
        target: target.into_proto(),
        unit_tags: tags.into_proto(),
        queue_command: queue,
      })),
    }),
    ..Default::default()
  }
}

// -----------------------------------------------------------------------------
// Queue State
// -----------------------------------------------------------------------------

struct State {
  dirty: BTreeSet<UnitTag>,
  queue: BTreeMap<(AbilityId, Target, bool), Vec<UnitTag>>,
}

impl State {
  #[inline]
  fn push(&mut self, slot: (AbilityId, Target, bool), unit: UnitTag) {
    self.dirty.insert(unit);
    self.queue.entry(slot).or_default().push(unit);
  }

  #[inline]
  fn drain<F>(&mut self, mut f: F)
  where
    F: FnMut(Action),
  {
    if self.queue.is_empty() {
      return;
    }

    while let Some((slot, tags)) = self.queue.pop_first() {
      f(to_action(slot, tags));
    }

    self.dirty.clear();
  }
}

// -----------------------------------------------------------------------------
// Command Queue
// -----------------------------------------------------------------------------

#[repr(transparent)]
pub struct CommandQueue {
  inner: Mutex<State>,
}

impl CommandQueue {
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: Mutex::new(State {
        dirty: BTreeSet::new(),
        queue: BTreeMap::new(),
      }),
    }
  }

  #[inline]
  pub fn push(&self, slot: (AbilityId, Target, bool), unit: UnitTag) {
    self.with(|guard| guard.push(slot, unit));
  }

  #[inline]
  pub fn drain<F>(&self, f: F)
  where
    F: FnMut(Action),
  {
    self.with(|guard| guard.drain(f));
  }

  #[inline]
  pub fn contains(&self, unit: UnitTag) -> bool {
    self.with(|guard| guard.dirty.contains(&unit))
  }

  #[inline]
  fn with<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&mut State) -> R,
    R: Default,
  {
    match self.inner.try_lock() {
      Ok(mut guard) => f(&mut guard),
      Err(TryLockError::Poisoned(error)) => f(&mut error.into_inner()),
      Err(TryLockError::WouldBlock) => Default::default(),
    }
  }
}

impl Debug for CommandQueue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut debug: DebugStruct<'_, '_> = f.debug_struct("CommandQueue");

    match self.inner.try_lock() {
      Ok(guard) => {
        debug.field("dirty", &guard.dirty);
        debug.field("queue", &guard.queue);
      }
      Err(TryLockError::Poisoned(error)) => {
        debug.field("dirty", &error.get_ref().dirty);
        debug.field("queue", &error.get_ref().queue);
      }
      Err(TryLockError::WouldBlock) => {
        debug.field("dirty", &"<locked>");
        debug.field("queue", &"<locked>");
      }
    }

    debug.finish()
  }
}
