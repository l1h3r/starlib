use crate::stable::UnitTypeId;

impl UnitTypeId {
  // ---------------------------------------------------------------------------
  // Resources
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_mineral(self) -> bool {
    matches!(
      self,
      Self::BattleStationMineralField
        | Self::BattleStationMineralField750
        | Self::LabMineralField
        | Self::LabMineralField750
        | Self::MineralField
        | Self::MineralField450
        | Self::MineralField750
        | Self::MineralFieldOpaque
        | Self::MineralFieldOpaque900
        | Self::PurifierMineralField
        | Self::PurifierMineralField750
        | Self::PurifierRichMineralField
        | Self::PurifierRichMineralField750
        | Self::RichMineralField
        | Self::RichMineralField750
    )
  }

  #[inline]
  pub const fn is_vespene(self) -> bool {
    matches!(
      self,
      Self::ProtossVespeneGeyser
        | Self::PurifierVespeneGeyser
        | Self::RichVespeneGeyser
        | Self::ShakurasVespeneGeyser
        | Self::SpacePlatformGeyser
        | Self::VespeneGeyser
    )
  }

  // ---------------------------------------------------------------------------
  // Building - Townhall
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_townhall(self) -> bool {
    self.is_townhall_terran() || self.is_townhall_zerg() || self.is_townhall_protoss()
  }

  #[inline]
  pub const fn is_townhall_terran(self) -> bool {
    matches!(
      self,
      Self::CommandCenter
        | Self::OrbitalCommand
        | Self::PlanetaryFortress
        | Self::CommandCenterFlying
        | Self::OrbitalCommandFlying
    )
  }

  #[inline]
  pub const fn is_townhall_zerg(self) -> bool {
    matches!(self, Self::Hatchery | Self::Lair | Self::Hive)
  }

  #[inline]
  pub const fn is_townhall_protoss(self) -> bool {
    matches!(self, Self::Nexus)
  }

  // ---------------------------------------------------------------------------
  // Building - Supply
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_supply(self) -> bool {
    self.is_supply_terran() || self.is_supply_zerg() || self.is_supply_protoss()
  }

  #[inline]
  pub const fn is_supply_terran(self) -> bool {
    matches!(self, Self::SupplyDepot)
  }

  #[inline]
  pub const fn is_supply_zerg(self) -> bool {
    matches!(
      self,
      Self::Overlord | Self::OverlordTransport | Self::Overseer | Self::OverseerSiegeMode
    )
  }

  #[inline]
  pub const fn is_supply_protoss(self) -> bool {
    matches!(self, Self::Pylon)
  }

  // ---------------------------------------------------------------------------
  // Building - Vespene Harvester
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_harvester(self) -> bool {
    self.is_harvester_terran() || self.is_harvester_zerg() || self.is_harvester_protoss()
  }

  #[inline]
  pub const fn is_harvester_terran(self) -> bool {
    matches!(self, Self::Refinery | Self::RefineryRich)
  }

  #[inline]
  pub const fn is_harvester_zerg(self) -> bool {
    matches!(self, Self::Extractor | Self::ExtractorRich)
  }

  #[inline]
  pub const fn is_harvester_protoss(self) -> bool {
    matches!(self, Self::Assimilator | Self::AssimilatorRich)
  }

  // ---------------------------------------------------------------------------
  // Building - Terran Add-on
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_techlab(self) -> bool {
    matches!(
      self,
      Self::TechLab | Self::BarracksTechLab | Self::FactoryTechLab | Self::StarportTechLab
    )
  }

  #[inline]
  pub const fn is_reactor(self) -> bool {
    matches!(
      self,
      Self::Reactor | Self::BarracksReactor | Self::FactoryReactor | Self::StarportReactor
    )
  }

  // ---------------------------------------------------------------------------
  // Unit - Worker
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_worker(self) -> bool {
    self.is_worker_terran() || self.is_worker_zerg() || self.is_worker_protoss()
  }

  #[inline]
  pub const fn is_worker_terran(self) -> bool {
    matches!(self, Self::SCV)
  }

  #[inline]
  pub const fn is_worker_zerg(self) -> bool {
    matches!(self, Self::Drone | Self::DroneBurrowed)
  }

  #[inline]
  pub const fn is_worker_protoss(self) -> bool {
    matches!(self, Self::Probe)
  }

  // ---------------------------------------------------------------------------
  // Unit - Builder
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_builder(self, of: Self) -> bool {
    match of {
      // Terran - Ground Units
      Self::AutoTurret => matches!(self, Self::Raven),
      Self::Cyclone => matches!(self, Self::Factory),
      Self::Ghost => matches!(self, Self::Barracks),
      Self::Hellion => matches!(self, Self::Factory),
      Self::HellionTank => matches!(self, Self::Factory),
      Self::Marauder => matches!(self, Self::Barracks),
      Self::Marine => matches!(self, Self::Barracks),
      Self::Reaper => matches!(self, Self::Barracks),
      Self::SCV => self.is_townhall_terran(),
      Self::SiegeTank => matches!(self, Self::Factory),
      Self::Thor => matches!(self, Self::Factory),
      Self::WidowMine => matches!(self, Self::Factory),
      // Terran - Air Units
      Self::Banshee => matches!(self, Self::Starport),
      Self::Battlecruiser => matches!(self, Self::Starport),
      Self::Liberator => matches!(self, Self::Starport),
      Self::Medivac => matches!(self, Self::Starport),
      Self::Raven => matches!(self, Self::Starport),
      Self::VikingFighter => matches!(self, Self::Starport),
      // Terran - Buildings
      Self::Armory => self.is_worker_terran(),
      Self::Barracks => self.is_worker_terran(),
      Self::Bunker => self.is_worker_terran(),
      Self::CommandCenter => self.is_worker_terran(),
      Self::EngineeringBay => self.is_worker_terran(),
      Self::Factory => self.is_worker_terran(),
      Self::FusionCore => self.is_worker_terran(),
      Self::GhostAcademy => self.is_worker_terran(),
      Self::MissileTurret => self.is_worker_terran(),
      Self::OrbitalCommand => matches!(self, Self::CommandCenter),
      Self::PlanetaryFortress => matches!(self, Self::CommandCenter),
      Self::Refinery => self.is_worker_terran(),
      Self::SensorTower => self.is_worker_terran(),
      Self::Starport => self.is_worker_terran(),
      Self::SupplyDepot => self.is_worker_terran(),
      // Zerg - Ground Units
      Self::Baneling => matches!(self, Self::Zergling),
      Self::Changeling => matches!(self, Self::Overseer | Self::OverseerSiegeMode),
      Self::Drone => matches!(self, Self::Larva),
      Self::Hydralisk => matches!(self, Self::Larva),
      Self::Infestor => matches!(self, Self::Larva),
      Self::LocustMPFlying => matches!(self, Self::SwarmHostMP | Self::SwarmHostBurrowedMP),
      Self::LurkerMP => matches!(self, Self::Hydralisk),
      Self::Queen => self.is_townhall_zerg(),
      Self::Ravager => matches!(self, Self::Roach),
      Self::Roach => matches!(self, Self::Larva),
      Self::SwarmHostMP => matches!(self, Self::Larva),
      Self::Ultralisk => matches!(self, Self::Larva),
      Self::Zergling => matches!(self, Self::Larva),
      // Zerg - Air Units
      Self::BroodLord => matches!(self, Self::Corruptor),
      Self::Corruptor => matches!(self, Self::Larva),
      Self::Mutalisk => matches!(self, Self::Larva),
      Self::Overlord => matches!(self, Self::Larva),
      Self::OverlordTransport => matches!(self, Self::Overlord),
      Self::Overseer => matches!(self, Self::Overlord | Self::OverlordTransport),
      Self::Viper => matches!(self, Self::Larva),
      // Zerg - Buildings
      Self::BanelingNest => self.is_worker_zerg(),
      Self::CreepTumor => matches!(self, Self::Queen | Self::CreepTumorBurrowed),
      Self::CreepTumorQueen => matches!(self, Self::Queen),
      Self::EvolutionChamber => self.is_worker_zerg(),
      Self::Extractor => self.is_worker_zerg(),
      Self::GreaterSpire => matches!(self, Self::Spire),
      Self::Hatchery => self.is_worker_zerg(),
      Self::Hive => matches!(self, Self::Lair),
      Self::HydraliskDen => self.is_worker_zerg(),
      Self::InfestationPit => self.is_worker_zerg(),
      Self::Lair => matches!(self, Self::Hatchery),
      Self::LurkerDenMP => self.is_worker_zerg(),
      Self::NydusCanal => matches!(self, Self::NydusNetwork),
      Self::NydusNetwork => self.is_worker_zerg(),
      Self::RoachWarren => self.is_worker_zerg(),
      Self::SpawningPool => self.is_worker_zerg(),
      Self::SpineCrawler => self.is_worker_zerg(),
      Self::Spire => self.is_worker_zerg(),
      Self::SporeCrawler => self.is_worker_zerg(),
      Self::UltraliskCavern => self.is_worker_zerg(),
      // Protoss - Ground Units
      Self::Adept => matches!(self, Self::Gateway | Self::WarpGate),
      Self::Colossus => matches!(self, Self::RoboticsFacility),
      Self::DarkTemplar => matches!(self, Self::Gateway | Self::WarpGate),
      Self::Disruptor => matches!(self, Self::RoboticsFacility),
      Self::HighTemplar => matches!(self, Self::Gateway | Self::WarpGate),
      Self::Immortal => matches!(self, Self::RoboticsFacility),
      Self::Probe => self.is_townhall_protoss(),
      Self::Sentry => matches!(self, Self::Gateway | Self::WarpGate),
      Self::Stalker => matches!(self, Self::Gateway | Self::WarpGate),
      Self::Zealot => matches!(self, Self::Gateway | Self::WarpGate),
      // Protoss - Air Units
      Self::Carrier => matches!(self, Self::Stargate),
      Self::Mothership => self.is_townhall_protoss(),
      Self::Observer => matches!(self, Self::RoboticsFacility),
      Self::Oracle => matches!(self, Self::Stargate),
      Self::OracleStasisTrap => matches!(self, Self::Oracle),
      Self::Phoenix => matches!(self, Self::Stargate),
      Self::Tempest => matches!(self, Self::Stargate),
      Self::VoidRay => matches!(self, Self::Stargate),
      Self::WarpPrism => matches!(self, Self::RoboticsFacility),
      // Protoss - Buildings
      Self::Assimilator => self.is_worker_protoss(),
      Self::CyberneticsCore => self.is_worker_protoss(),
      Self::DarkShrine => self.is_worker_protoss(),
      Self::FleetBeacon => self.is_worker_protoss(),
      Self::Forge => self.is_worker_protoss(),
      Self::Gateway => self.is_worker_protoss(),
      Self::Nexus => self.is_worker_protoss(),
      Self::PhotonCannon => self.is_worker_protoss(),
      Self::Pylon => self.is_worker_protoss(),
      Self::RoboticsBay => self.is_worker_protoss(),
      Self::RoboticsFacility => self.is_worker_protoss(),
      Self::ShieldBattery => self.is_worker_protoss(),
      Self::Stargate => self.is_worker_protoss(),
      Self::TemplarArchive => self.is_worker_protoss(),
      Self::TwilightCouncil => self.is_worker_protoss(),
      _ => false,
    }
  }

  // ---------------------------------------------------------------------------
  // Misc.
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn is_half_supply(self) -> bool {
    matches!(
      self,
      Self::Zergling
        | Self::ZerglingBurrowed
        | Self::Baneling
        | Self::BanelingBurrowed
        | Self::BanelingCocoon
    )
  }

  #[inline]
  pub const fn is_watchtower(self) -> bool {
    matches!(self, Self::XelNagaTower)
  }

  #[inline]
  pub const fn is_larva(self) -> bool {
    matches!(self, UnitTypeId::Larva)
  }

  #[inline]
  pub const fn is_egg(self) -> bool {
    matches!(self, UnitTypeId::Egg)
  }
}
