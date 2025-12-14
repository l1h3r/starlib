use std::sync::Arc;

use starlib_types::AbilityId;
use starlib_types::Alliance;
use starlib_types::GameData;
use starlib_types::GameState;
use starlib_types::UnitTag;
use starlib_types::UnitTypeId;

use crate::exec::Resource;
use crate::game::TagSet;

#[derive(Debug)]
pub struct UnitTracker {
  game_data: Arc<GameData>,
  owned_tags: TagSet<UnitTypeId>,
  enemy_tags: TagSet<UnitTypeId>,
  orders: TagSet<AbilityId>,
}

impl UnitTracker {
  #[inline]
  pub fn new(game_data: Arc<GameData>) -> Self {
    Self {
      game_data,
      owned_tags: TagSet::new(),
      enemy_tags: TagSet::new(),
      orders: TagSet::new(),
    }
  }

  #[inline]
  fn owned(&self, kind: UnitTypeId) -> usize {
    self.owned_tags.count(&kind)
  }

  #[inline]
  fn enemy(&self, kind: UnitTypeId) -> usize {
    self.enemy_tags.count(&kind)
  }

  #[inline]
  fn orders(&self, kind: AbilityId) -> usize {
    self.orders.count(&kind)
  }

  fn owned_alias(&self, kind: UnitTypeId) -> usize {
    match kind {
      // -----------------------------------------------------------------------
      // Zerg Larva Units
      // -----------------------------------------------------------------------
      UnitTypeId::Drone => self.owned(UnitTypeId::DroneBurrowed),
      UnitTypeId::Overlord => self.owned(UnitTypeId::OverlordTransport) + self.owned(UnitTypeId::Overseer),
      // -----------------------------------------------------------------------
      // Fallback
      // -----------------------------------------------------------------------
      _ => 0,
    }
  }

  pub fn count_current(&self, kind: UnitTypeId) -> usize {
    self.owned(kind) + self.owned_alias(kind)
  }

  pub fn count_pending(&self, kind: UnitTypeId) -> usize {
    let Some(udata) = self.game_data.try_unit(kind) else {
      return 0;
    };

    self.orders(udata.ability_id())
  }
}

impl Resource for UnitTracker {
  fn tick(&mut self, state: &GameState) {
    self.owned_tags.clear();
    self.enemy_tags.clear();
    self.orders.clear();

    for unit in state.units() {
      // Ignore all hallucinations
      if unit.is_hallucination() {
        continue;
      }

      let kind: UnitTypeId = unit.kind();
      let utag: UnitTag = unit.tag();

      match unit.alliance() {
        Alliance::User => {
          self.owned_tags.add(kind, utag);

          for order in unit.orders() {
            self.orders.add(order.ability_id(), utag);
          }
        }
        Alliance::Ally => {
          // do nothing
        }
        Alliance::Neutral => {
          // do nothing
        }
        Alliance::Enemy => {
          self.enemy_tags.add(kind, utag);
        }
      }
    }
  }
}
