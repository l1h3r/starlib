use crate::bridge::Race;
use crate::stable::UnitTypeId;

impl Race {
  #[inline]
  pub const fn is_townhall(self, kind: UnitTypeId) -> bool {
    match self {
      Self::NoRace => false,
      Self::Terran => kind.is_townhall_terran(),
      Self::Zerg => kind.is_townhall_zerg(),
      Self::Protoss => kind.is_townhall_protoss(),
      Self::Random => kind.is_townhall(),
    }
  }

  #[inline]
  pub const fn is_supply(self, kind: UnitTypeId) -> bool {
    match self {
      Self::NoRace => false,
      Self::Terran => kind.is_supply_terran(),
      Self::Zerg => kind.is_supply_zerg(),
      Self::Protoss => kind.is_supply_protoss(),
      Self::Random => kind.is_supply(),
    }
  }

  #[inline]
  pub const fn is_harvester(self, kind: UnitTypeId) -> bool {
    match self {
      Self::NoRace => false,
      Self::Terran => kind.is_harvester_terran(),
      Self::Zerg => kind.is_harvester_zerg(),
      Self::Protoss => kind.is_harvester_protoss(),
      Self::Random => kind.is_harvester(),
    }
  }

  #[inline]
  pub const fn is_worker(self, kind: UnitTypeId) -> bool {
    match self {
      Self::NoRace => false,
      Self::Terran => kind.is_worker_terran(),
      Self::Zerg => kind.is_worker_zerg(),
      Self::Protoss => kind.is_worker_protoss(),
      Self::Random => kind.is_worker(),
    }
  }
}
