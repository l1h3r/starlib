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
  owned_ids: TagSet<UnitTypeId>,
  enemy_ids: TagSet<UnitTypeId>,
  abilities: TagSet<AbilityId>,
}

impl UnitTracker {
  #[inline]
  pub fn new(game_data: Arc<GameData>) -> Self {
    Self {
      game_data,
      owned_ids: TagSet::new(),
      enemy_ids: TagSet::new(),
      abilities: TagSet::new(),
    }
  }

  #[inline]
  pub fn count_owned(&self, kind: UnitTypeId) -> usize {
    self.owned_ids.count(&kind)
  }

  #[inline]
  pub fn count_enemy(&self, kind: UnitTypeId) -> usize {
    self.enemy_ids.count(&kind)
  }

  #[inline]
  pub fn count_abilities(&self, kind: AbilityId) -> usize {
    self.abilities.count(&kind)
  }

  pub fn count_owned_alias(&self, kind: UnitTypeId) -> usize {
    match kind {
      // -----------------------------------------------------------------------
      // Zerg - Hatchery Buldings
      // -----------------------------------------------------------------------

      // -----------------------------------------------------------------------
      // Zerg - Lair Buldings
      // -----------------------------------------------------------------------

      // -----------------------------------------------------------------------
      // Zerg - Special Buildings
      // -----------------------------------------------------------------------

      // -----------------------------------------------------------------------
      // Zerg - Hatchery Units
      // -----------------------------------------------------------------------

      // -----------------------------------------------------------------------
      // Zerg - Larva Units
      // -----------------------------------------------------------------------
      UnitTypeId::Drone => self.count_owned(UnitTypeId::DroneBurrowed),
      UnitTypeId::Overlord => self.count_owned(UnitTypeId::OverlordTransport) + self.count_owned(UnitTypeId::Overseer),
      // -----------------------------------------------------------------------
      // Zerg - Morph Units
      // -----------------------------------------------------------------------

      // -----------------------------------------------------------------------
      // Zerg - Misc. Units
      // -----------------------------------------------------------------------

      // -----------------------------------------------------------------------
      // Fallback
      // -----------------------------------------------------------------------
      _ => 0,
    }
  }

  pub fn count_current(&self, kind: UnitTypeId) -> usize {
    self.count_owned(kind) + self.count_owned_alias(kind)
  }

  pub fn count_pending(&self, kind: UnitTypeId) -> usize {
    let Some(udata) = self.game_data.try_unit(kind) else {
      return 0;
    };

    if matches!(kind, UnitTypeId::Zergling) {
      self.count_abilities(udata.ability_id()) * 2
    } else {
      self.count_abilities(udata.ability_id())
    }
  }
}

impl Resource for UnitTracker {
  fn tick(&mut self, state: &GameState) {
    self.owned_ids.clear();
    self.enemy_ids.clear();
    self.abilities.clear();

    for unit in state.units() {
      // Ignore all hallucinations
      if unit.is_hallucination() {
        continue;
      }

      let kind: UnitTypeId = unit.kind();
      let utag: UnitTag = unit.tag();

      match unit.alliance() {
        Alliance::User => {
          self.owned_ids.add(kind, utag);

          for order in unit.orders() {
            self.abilities.add(order.ability_id(), utag);
          }
        }
        Alliance::Ally => {
          // do nothing
        }
        Alliance::Neutral => {
          // do nothing
        }
        Alliance::Enemy => {
          self.enemy_ids.add(kind, utag);
        }
      }
    }
  }
}
