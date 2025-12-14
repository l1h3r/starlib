use starlib_types::GameState;

use crate::exec::Resource;
use crate::game::TryCost;

#[derive(Debug)]
pub struct CostTracker {
  minerals_held: u32,
  minerals_used: u32,
  vespene_held: u32,
  vespene_used: u32,
  supply_held: u32,
  supply_used: u32,
}

impl CostTracker {
  #[inline]
  pub const fn new() -> Self {
    Self {
      minerals_held: 0,
      minerals_used: 0,
      vespene_held: 0,
      vespene_used: 0,
      supply_held: 0,
      supply_used: 0,
    }
  }

  #[inline]
  pub const fn minerals_held(&self) -> u32 {
    self.minerals_held
  }

  #[inline]
  pub const fn minerals_used(&self) -> u32 {
    self.minerals_used
  }

  #[inline]
  pub const fn minerals(&self) -> u32 {
    self.minerals_held.saturating_sub(self.minerals_used)
  }

  #[inline]
  pub const fn vespene_held(&self) -> u32 {
    self.vespene_held
  }

  #[inline]
  pub const fn vespene_used(&self) -> u32 {
    self.vespene_used
  }

  #[inline]
  pub const fn vespene(&self) -> u32 {
    self.vespene_held.saturating_sub(self.vespene_used)
  }

  #[inline]
  pub const fn supply_held(&self) -> u32 {
    self.supply_held
  }

  #[inline]
  pub const fn supply_used(&self) -> u32 {
    self.supply_used
  }

  #[inline]
  pub const fn supply(&self) -> u32 {
    self.supply_held.saturating_sub(self.supply_used)
  }

  #[inline]
  pub fn can_afford(&self, data: impl TryCost) -> bool {
    let Some(cost) = data.try_cost() else {
      return true;
    };

    cost.minerals() <= self.minerals()
      && cost.vespene() <= self.vespene()
      && cost.supply() <= self.supply()
  }

  #[inline]
  pub fn reserve(&mut self, cost: impl TryCost) {
    let Some(cost) = cost.try_cost() else {
      return;
    };

    self.minerals_used = self.minerals_used.saturating_add(cost.minerals());
    self.vespene_used = self.vespene_used.saturating_add(cost.vespene());
    self.supply_used = self.supply_used.saturating_add(cost.supply());
  }
}

impl Resource for CostTracker {
  fn tick(&mut self, state: &GameState) {
    self.minerals_held = state.minerals();
    self.vespene_held = state.vespene();
    self.supply_held = state.food_remaining();

    self.minerals_used = 0;
    self.vespene_used = 0;
    self.supply_used = 0;
  }
}
