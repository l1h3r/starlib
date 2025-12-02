use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::sync::Arc;

use starlib_proto::Observation;
use starlib_proto::ResponsePing;
use starlib_proto::Score as ProtoScore;

use crate::bridge::Ability;
use crate::bridge::AbilityData;
use crate::bridge::AbilityTarget;
use crate::bridge::Alert;
use crate::bridge::BuffData;
use crate::bridge::DisplayType;
use crate::bridge::Effect;
use crate::bridge::EffectData;
use crate::bridge::PowerSource;
use crate::bridge::Race;
use crate::bridge::RadarRing;
use crate::bridge::Score;
use crate::bridge::ScoreInfo;
use crate::bridge::ScoreType;
use crate::bridge::Target;
use crate::bridge::Unit;
use crate::bridge::UnitTypeData;
use crate::bridge::UpgradeData;
use crate::core::PlayerTag;
use crate::core::Point3DF;
use crate::core::UnitTag;
use crate::game::Blip;
use crate::game::CommandQueue;
use crate::game::GameData;
use crate::game::GameInfo;
use crate::game::UnitIter;
use crate::game::Units;
use crate::grid::PixelGrid;
use crate::grid::pixels;
use crate::stable::AbilityId;
use crate::stable::BuffId;
use crate::stable::EffectId;
use crate::stable::UnitTypeId;
use crate::stable::UpgradeId;
use crate::traits::ExtendProto;
use crate::traits::IntoSc2;

pub struct GameState {
  // ---------------------------------------------------------------------------
  // Build Info
  // ---------------------------------------------------------------------------
  game_version: String,
  data_version: String,
  data_build: u32,
  base_build: u32,
  // ---------------------------------------------------------------------------
  // Game Data
  // ---------------------------------------------------------------------------
  game_info: Arc<GameInfo>,
  game_data: Arc<GameData>,
  // ---------------------------------------------------------------------------
  // Game State - Setup
  // ---------------------------------------------------------------------------
  player_tag: PlayerTag,
  player_race: Race,
  player_name: String,
  enemies: HashMap<PlayerTag, Race>,
  // ---------------------------------------------------------------------------
  // Game State - Common
  // ---------------------------------------------------------------------------
  ticks: u32,
  score: Score,
  // ---------------------------------------------------------------------------
  // Game State - Player
  // ---------------------------------------------------------------------------
  camera: Point3DF,
  minerals: u32,
  vespene: u32,
  food_cap: u32,
  food_used: u32,
  food_army: u32,
  food_workers: u32,
  idle_worker_count: u32,
  army_count: u32,
  warp_gate_count: u32,
  larva_count: u32,
  // ---------------------------------------------------------------------------
  // Game State - Map
  // ---------------------------------------------------------------------------
  map_layer: PixelGrid<u8>,
  map_creep: PixelGrid<pixels::Block>,
  // ---------------------------------------------------------------------------
  // Game State - Units
  // ---------------------------------------------------------------------------
  blips: Vec<Blip>,
  units: Vec<Unit>,
  dead_all: HashSet<UnitTag>,
  dead_now: HashSet<UnitTag>,
  type_map: HashMap<UnitTag, UnitTypeId>,
  tags_set: HashSet<UnitTag>,
  commands: CommandQueue,
  // ---------------------------------------------------------------------------
  // Game State - Misc.
  // ---------------------------------------------------------------------------
  abilities: Vec<Ability>,
  alerts: Vec<Alert>,
  effects: Vec<Effect>,
  power_sources: Vec<PowerSource>,
  radar: Vec<RadarRing>,
  upgrades: HashSet<UpgradeId>,
}

impl GameState {
  const CAP_SM: usize = 32;
  const CAP_MD: usize = 64;
  const CAP_LG: usize = 128;
  const CAP_XL: usize = 256;

  const INIT_MINERALS: u32 = 50;
  const INIT_FOOD_CAP: u32 = 14;
  const INIT_FOOD_USED: u32 = 12;
  const INIT_FOOD_WORKERS: u32 = 12;

  const MAX_PLAYERS: usize = 16;

  pub fn new(
    player_tag: PlayerTag,
    player_race: Race,
    player_name: String,
    ping_info: ResponsePing,
    game_info: GameInfo,
    game_data: GameData,
  ) -> Self {
    Self {
      game_version: ping_info.game_version,
      data_version: ping_info.data_version,
      data_build: ping_info.data_build,
      base_build: ping_info.base_build,

      game_info: Arc::new(game_info),
      game_data: Arc::new(game_data),

      player_tag,
      player_race,
      player_name,
      enemies: HashMap::with_capacity(Self::MAX_PLAYERS - 1),

      ticks: 0,
      score: <ProtoScore as Default>::default().into_sc2(),

      camera: Default::default(),
      minerals: Self::INIT_MINERALS,
      vespene: 0,
      food_cap: Self::INIT_FOOD_CAP,
      food_used: Self::INIT_FOOD_USED,
      food_army: 0,
      food_workers: Self::INIT_FOOD_WORKERS,
      idle_worker_count: 0,
      army_count: 0,
      warp_gate_count: 0,
      larva_count: 0,

      map_layer: PixelGrid::new(Default::default()),
      map_creep: PixelGrid::new(Default::default()),

      blips: Vec::with_capacity(Self::CAP_MD),
      units: Vec::with_capacity(Self::CAP_XL),
      dead_all: HashSet::with_capacity(Self::CAP_XL),
      dead_now: HashSet::with_capacity(Self::CAP_MD),
      type_map: HashMap::with_capacity(Self::CAP_XL),
      tags_set: HashSet::with_capacity(Self::CAP_XL),
      commands: CommandQueue::new(),

      abilities: Vec::with_capacity(Self::CAP_SM),
      alerts: Vec::with_capacity(Self::CAP_SM),
      effects: Vec::with_capacity(Self::CAP_LG),
      power_sources: Vec::with_capacity(Self::CAP_MD),
      radar: Vec::with_capacity(Self::CAP_SM),
      upgrades: HashSet::with_capacity(Self::CAP_SM),
    }
  }

  pub fn update<const INIT: bool>(&mut self, observation: &Observation) {
    self.blips.clear();
    self.units.clear();

    self.dead_all.extend(self.dead_now.drain());
    self.tags_set.clear();

    self.abilities.clear();
    self.alerts.clear();
    self.effects.clear();
    self.power_sources.clear();
    self.radar.clear();
    self.upgrades.clear();

    self.ticks = observation.game_loop;

    if let Some(score) = observation.score {
      self.score = score.into_sc2();
    }

    self
      .abilities
      .extend_proto(observation.abilities.iter().copied());

    self.alerts.extend_proto(observation.alerts.iter().copied());

    if let Some(player) = observation.player_common {
      self.minerals = player.minerals;
      self.vespene = player.vespene;
      self.food_cap = player.food_cap;
      self.food_used = player.food_used;
      self.food_army = player.food_army;
      self.food_workers = player.food_workers;
      self.idle_worker_count = player.idle_worker_count;
      self.army_count = player.army_count;
      self.warp_gate_count = player.warp_gate_count;
      self.larva_count = player.larva_count;
    }

    if let Some(raw) = observation.raw_data.as_ref() {
      self.effects.extend_proto(raw.effects.iter().cloned());
      self.radar.extend_proto(raw.radar.iter().copied());

      if let Some(event) = raw.event.as_ref() {
        self.dead_now.extend_proto(event.dead_units.iter().copied());
        self.dead_all.extend(self.dead_now.iter().copied());
      }

      if let Some(map) = raw.map_state.as_ref() {
        if let Some(data) = map.visibility.as_ref() {
          self.map_layer.update(data);
        }

        if let Some(data) = map.creep.as_ref() {
          self.map_creep.update(data);
        }
      }

      if let Some(player) = raw.player.as_ref() {
        if let Some(camera) = player.camera {
          self.camera = camera.into_sc2();
        }

        self
          .power_sources
          .extend_proto(player.power_sources.iter().copied());

        self
          .upgrades
          .extend_proto(player.upgrade_ids.iter().copied());
      }

      for unit in raw.units.iter() {
        if unit.is_blip {
          self.blips.push(Blip::new(unit));
          continue;
        }

        let core: Unit = unit.clone().into_sc2();

        self.type_map.insert(core.tag(), core.kind());
        self.tags_set.insert(core.tag());

        if core.display() == DisplayType::Placeholder {
          println!("TODO: Placeholder");
        } else {
          self.units.push(core);
        }
      }
    }

    if matches!(self.player_race, Race::Zerg) {
      // Workaround
      //
      // https://github.com/Blizzard/s2client-proto/issues/123
      let count: usize = self.units().owned().half_supply().count();

      self.food_used += count as u32 % 2;
      self.food_army += count as u32 % 2;
    }

    if INIT {
      self.enemies = self
        .units()
        .enemies()
        .map(|unit| (unit.owner(), unit.race()))
        .collect();

      if let Some(townhall) = self.units().owned().townhalls().next() {
        let point: Point3DF = townhall.point();

        if let Some(info) = Arc::get_mut(&mut self.game_info) {
          info.update(point.into_2d());
        } else {
          eprintln!("GameInfo has multiple references");
        }
      }
    }
  }

  pub(crate) fn action(
    &self,
    unit: UnitTag,
    ability: AbilityId,
    target: Target,
    queue: bool,
  ) -> bool {
    match self.ability(ability).target() {
      AbilityTarget::None if !matches!(target, Target::None) => {
        eprintln!("> Invalid Target (expected none, received {target:?})");
      }
      AbilityTarget::Point if !matches!(target, Target::Point(_)) => {
        eprintln!("> Invalid Target (expected point, received {target:?})");
      }
      AbilityTarget::Unit if !matches!(target, Target::Unit(_)) => {
        eprintln!("> Invalid Target (expected unit, received {target:?})");
      }
      AbilityTarget::PointOrUnit if !matches!(target, Target::Point(_) | Target::Unit(_)) => {
        eprintln!("> Invalid Target (expected point/unit, received {target:?})");
      }
      AbilityTarget::PointOrNone if !matches!(target, Target::Point(_) | Target::None) => {
        eprintln!("> Invalid Target (expected point/none, received {target:?})");
      }
      _ => {}
    }

    if let Some(cost) = self.game_data.ability_cost(ability) {
      if cost.minerals() > self.minerals {
        return false;
      }

      if cost.vespene() > self.vespene {
        return false;
      }

      if cost.supply().into_f32() > self.food_remaining() as f32 {
        return false;
      }
    }

    self.commands.push((ability, target, queue), unit);

    true
  }

  // ---------------------------------------------------------------------------
  // Build Info
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn game_version(&self) -> &str {
    self.game_version.as_str()
  }

  #[inline]
  pub const fn data_version(&self) -> &str {
    self.data_version.as_str()
  }

  #[inline]
  pub const fn data_build(&self) -> u32 {
    self.data_build
  }

  #[inline]
  pub const fn base_build(&self) -> u32 {
    self.base_build
  }

  // ---------------------------------------------------------------------------
  // Game Data
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn game_info(&self) -> &Arc<GameInfo> {
    &self.game_info
  }

  #[inline]
  pub const fn game_data(&self) -> &Arc<GameData> {
    &self.game_data
  }

  #[inline]
  pub fn ability(&self, id: AbilityId) -> &AbilityData {
    self.game_data.ability(id)
  }

  #[inline]
  pub fn buff(&self, id: BuffId) -> &BuffData {
    self.game_data.buff(id)
  }

  #[inline]
  pub fn effect(&self, id: EffectId) -> &EffectData {
    self.game_data.effect(id)
  }

  #[inline]
  pub fn unit(&self, id: UnitTypeId) -> &UnitTypeData {
    self.game_data.unit(id)
  }

  #[inline]
  pub fn upgrade(&self, id: UpgradeId) -> &UpgradeData {
    self.game_data.upgrade(id)
  }

  #[inline]
  pub fn try_ability(&self, id: AbilityId) -> Option<&AbilityData> {
    self.game_data.try_ability(id)
  }

  #[inline]
  pub fn try_buff(&self, id: BuffId) -> Option<&BuffData> {
    self.game_data.try_buff(id)
  }

  #[inline]
  pub fn try_effect(&self, id: EffectId) -> Option<&EffectData> {
    self.game_data.try_effect(id)
  }

  #[inline]
  pub fn try_unit(&self, id: UnitTypeId) -> Option<&UnitTypeData> {
    self.game_data.try_unit(id)
  }

  #[inline]
  pub fn try_upgrade(&self, id: UpgradeId) -> Option<&UpgradeData> {
    self.game_data.try_upgrade(id)
  }

  // ---------------------------------------------------------------------------
  // Game State - Setup
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn player_tag(&self) -> PlayerTag {
    self.player_tag
  }

  #[inline]
  pub const fn player_race(&self) -> Race {
    self.player_race
  }

  #[inline]
  pub const fn player_name(&self) -> &str {
    self.player_name.as_str()
  }

  #[inline]
  pub const fn enemies(&self) -> &HashMap<PlayerTag, Race> {
    &self.enemies
  }

  // ---------------------------------------------------------------------------
  // Game State - Common
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn ticks(&self) -> u32 {
    self.ticks
  }

  #[inline]
  pub fn score_type(&self) -> ScoreType {
    self.score.score_type()
  }

  #[inline]
  pub fn score_data(&self) -> i32 {
    self.score.score_data()
  }

  #[inline]
  pub fn score_info(&self) -> &ScoreInfo {
    self.score.score_info()
  }

  // ---------------------------------------------------------------------------
  // Game State - Player
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn camera(&self) -> Point3DF {
    self.camera
  }

  #[inline]
  pub const fn minerals(&self) -> u32 {
    self.minerals
  }

  #[inline]
  pub const fn vespene(&self) -> u32 {
    self.vespene
  }

  #[inline]
  pub const fn food_cap(&self) -> u32 {
    self.food_cap
  }

  #[inline]
  pub const fn food_used(&self) -> u32 {
    self.food_used
  }

  #[inline]
  pub const fn food_army(&self) -> u32 {
    self.food_army
  }

  #[inline]
  pub const fn food_workers(&self) -> u32 {
    self.food_workers
  }

  #[inline]
  pub const fn food_remaining(&self) -> u32 {
    self.food_cap.saturating_sub(self.food_used)
  }

  #[inline]
  pub const fn idle_worker_count(&self) -> u32 {
    self.idle_worker_count
  }

  #[inline]
  pub const fn army_count(&self) -> u32 {
    self.army_count
  }

  #[inline]
  pub const fn warp_gate_count(&self) -> u32 {
    self.warp_gate_count
  }

  #[inline]
  pub const fn larva_count(&self) -> u32 {
    self.larva_count
  }

  // ---------------------------------------------------------------------------
  // Game State - Map
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn map_layer(&self) -> &PixelGrid<u8> {
    &self.map_layer
  }

  #[inline]
  pub const fn map_creep(&self) -> &PixelGrid<pixels::Block> {
    &self.map_creep
  }

  // ---------------------------------------------------------------------------
  // Game State - Units
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn blips(&self) -> &[Blip] {
    self.blips.as_slice()
  }

  #[inline]
  pub fn units(&self) -> Units<'_> {
    Units::new(self, self.units.as_slice())
  }

  #[inline]
  pub const fn dead_all(&self) -> &HashSet<UnitTag> {
    &self.dead_all
  }

  #[inline]
  pub const fn dead_now(&self) -> &HashSet<UnitTag> {
    &self.dead_now
  }

  #[inline]
  pub const fn type_map(&self) -> &HashMap<UnitTag, UnitTypeId> {
    &self.type_map
  }

  #[inline]
  pub const fn tags_set(&self) -> &HashSet<UnitTag> {
    &self.tags_set
  }

  #[inline]
  pub const fn commands(&self) -> &CommandQueue {
    &self.commands
  }

  // ---------------------------------------------------------------------------
  // Game State - Misc.
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn abilities(&self) -> &[Ability] {
    self.abilities.as_slice()
  }

  #[inline]
  pub const fn alerts(&self) -> &[Alert] {
    self.alerts.as_slice()
  }

  #[inline]
  pub const fn effects(&self) -> &[Effect] {
    self.effects.as_slice()
  }

  #[inline]
  pub const fn power_sources(&self) -> &[PowerSource] {
    self.power_sources.as_slice()
  }

  #[inline]
  pub const fn radar(&self) -> &[RadarRing] {
    self.radar.as_slice()
  }

  #[inline]
  pub const fn upgrades(&self) -> &HashSet<UpgradeId> {
    &self.upgrades
  }
}

impl Debug for GameState {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("GameState")
      .field("game_version", &self.game_version)
      .field("data_version", &self.data_version)
      .field("data_build", &self.data_build)
      .field("base_build", &self.base_build)
      .field("game_info", &self.game_info)
      .field("game_data", &"...")
      .field("player_tag", &self.player_tag)
      .field("player_race", &self.player_race)
      .field("player_name", &self.player_name)
      .field("enemies", &self.enemies)
      .field("ticks", &self.ticks)
      .field("score", &"...")
      .field("camera", &self.camera)
      .field("minerals", &self.minerals)
      .field("vespene", &self.vespene)
      .field("food_cap", &self.food_cap)
      .field("food_used", &self.food_used)
      .field("food_army", &self.food_army)
      .field("food_workers", &self.food_workers)
      .field("idle_worker_count", &self.idle_worker_count)
      .field("army_count", &self.army_count)
      .field("warp_gate_count", &self.warp_gate_count)
      .field("larva_count", &self.larva_count)
      .field("map_layer", &self.map_layer)
      .field("map_creep", &self.map_creep)
      .field("blips", &self.blips)
      .field("units", &Vec::from_iter(self.units()))
      .field("dead_all", &self.dead_all)
      .field("dead_now", &self.dead_now)
      .field("type_map", &self.type_map)
      .field("tags_set", &self.tags_set)
      .field("abilities", &self.abilities)
      .field("alerts", &self.alerts)
      .field("effects", &self.effects)
      .field("power_sources", &self.power_sources)
      .field("radar", &self.radar)
      .field("upgrades", &self.upgrades)
      .finish()
  }
}
