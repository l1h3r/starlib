use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::time::Duration;

use crate::bridge::AbilityData;
use crate::bridge::Alliance;
use crate::bridge::Attribute;
use crate::bridge::CloakState;
use crate::bridge::DisplayType;
use crate::bridge::Race;
use crate::bridge::Target;
use crate::bridge::Unit;
use crate::bridge::UnitOrder;
use crate::bridge::UnitTypeData;
use crate::bridge::Weapon;
use crate::core::Cost;
use crate::core::Float;
use crate::core::Percent;
use crate::core::PlayerTag;
use crate::core::Point3DF;
use crate::core::PointF;
use crate::core::Stat;
use crate::core::UnitTag;
use crate::game::GameState;
use crate::game::UnitIter;
use crate::stable::AbilityId;
use crate::stable::UnitTypeId;

#[derive(Clone, Copy)]
pub struct UnitRef<'a> {
  root: &'a GameState,
  data: &'a UnitTypeData,
  unit: &'a Unit,
}

impl<'a> UnitRef<'a> {
  #[inline]
  pub fn new(root: &'a GameState, unit: &'a Unit) -> Self {
    Self {
      root,
      data: root.unit(unit.kind()),
      unit,
    }
  }

  // ---------------------------------------------------------------------------
  // Actions
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn has_action(&self) -> bool {
    self.root.commands().contains(self.tag())
  }

  #[inline]
  pub fn action(&self, ability: AbilityId, target: Target, queue: bool) -> bool {
    self.root.action(self.tag(), ability, target, queue)
  }

  #[inline]
  pub fn train(&self, target: UnitTypeId, queue: bool) -> bool {
    let Some(unit) = self.root.try_unit(target) else {
      return false;
    };

    self.action(unit.ability_id(), Target::None, queue)
  }

  #[inline]
  pub fn smart<T>(&self, target: T, queue: bool) -> bool
  where
    T: Into<Target>,
  {
    self.action(AbilityId::Smart, target.into(), queue)
  }

  #[inline]
  pub fn stop(&self, queue: bool) -> bool {
    self.action(AbilityId::Stop, Target::None, queue)
  }

  #[inline]
  pub fn cheer(&self, queue: bool) -> bool {
    self.action(AbilityId::StopCheer, Target::None, queue)
  }

  #[inline]
  pub fn dance(&self, queue: bool) -> bool {
    self.action(AbilityId::StopDance, Target::None, queue)
  }

  #[inline]
  pub fn harvest_gather(&self, target: UnitTag, queue: bool) -> bool {
    self.action(AbilityId::HarvestGather, target.into(), queue)
  }

  #[inline]
  pub fn harvest_return(&self, queue: bool) -> bool {
    self.action(AbilityId::HarvestReturn, Target::None, queue)
  }

  #[inline]
  pub fn attack<T>(&self, target: T, queue: bool) -> bool
  where
    T: Into<Target>,
  {
    self.action(AbilityId::Attack, target.into(), queue)
  }

  #[inline]
  pub fn hold_position(&self, queue: bool) -> bool {
    self.action(AbilityId::HoldPosition, Target::None, queue)
  }

  #[inline]
  pub fn move_to<T>(&self, target: T, queue: bool) -> bool
  where
    T: Into<Target>,
  {
    self.action(AbilityId::Move, target.into(), queue)
  }

  #[inline]
  pub fn patrol<T>(&self, target: T, queue: bool) -> bool
  where
    T: Into<Target>,
  {
    self.action(AbilityId::Patrol, target.into(), queue)
  }

  // ---------------------------------------------------------------------------
  // Properties - General
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn cost(&self) -> Cost {
    self.data.cost()
  }

  // ---------------------------------------------------------------------------
  // Properties - Identifiers
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn name(&self) -> &str {
    self.data.name()
  }

  #[inline]
  pub fn kind(&self) -> UnitTypeId {
    self.data.id()
  }

  #[inline]
  pub fn tag(&self) -> UnitTag {
    self.unit.tag()
  }

  #[inline]
  pub fn owner(&self) -> PlayerTag {
    self.unit.owner()
  }

  #[inline]
  pub fn alias_id(&self) -> Option<UnitTypeId> {
    self.data.unit_alias_id().into_option()
  }

  #[inline]
  pub fn alias(&self) -> Option<&UnitTypeData> {
    self.alias_id().and_then(|id| self.root.try_unit(id))
  }

  // ---------------------------------------------------------------------------
  // Properties - Race
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn race(&self) -> Race {
    self.data.race()
  }

  #[inline]
  pub fn is_terran(&self) -> bool {
    matches!(self.race(), Race::Terran)
  }

  #[inline]
  pub fn is_zerg(&self) -> bool {
    matches!(self.race(), Race::Zerg)
  }

  #[inline]
  pub fn is_protoss(&self) -> bool {
    matches!(self.race(), Race::Protoss)
  }

  #[inline]
  pub fn is_random_race(&self) -> bool {
    matches!(self.race(), Race::Random)
  }

  // ---------------------------------------------------------------------------
  // Properties - Alliance
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn alliance(&self) -> Alliance {
    self.unit.alliance()
  }

  #[inline]
  pub fn is_owned(&self) -> bool {
    matches!(self.alliance(), Alliance::User)
  }

  #[inline]
  pub fn is_ally(&self) -> bool {
    matches!(self.alliance(), Alliance::Ally)
  }

  #[inline]
  pub fn is_neutral(&self) -> bool {
    matches!(self.alliance(), Alliance::Neutral)
  }

  #[inline]
  pub fn is_enemy(&self) -> bool {
    matches!(self.alliance(), Alliance::Enemy)
  }

  // ---------------------------------------------------------------------------
  // Properties - Display Type
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn display(&self) -> DisplayType {
    self.unit.display()
  }

  #[inline]
  pub fn is_visible(&self) -> bool {
    matches!(self.display(), DisplayType::Visible)
  }

  #[inline]
  pub fn is_snapshot(&self) -> bool {
    matches!(self.display(), DisplayType::Snapshot)
  }

  #[inline]
  pub fn is_hidden(&self) -> bool {
    matches!(self.display(), DisplayType::Hidden)
  }

  #[inline]
  pub fn is_placeholder(&self) -> bool {
    matches!(self.display(), DisplayType::Placeholder)
  }

  // ---------------------------------------------------------------------------
  // Properties - Cloak State
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn cloak(&self) -> CloakState {
    self.unit.cloak()
  }

  #[inline]
  pub fn is_cloaked_unknown(&self) -> bool {
    matches!(self.cloak(), CloakState::CloakedUnknown)
  }

  #[inline]
  pub fn is_cloaked(&self) -> bool {
    matches!(self.cloak(), CloakState::Cloaked)
  }

  #[inline]
  pub fn is_cloaked_detected(&self) -> bool {
    matches!(self.cloak(), CloakState::CloakedDetected)
  }

  #[inline]
  pub fn is_not_cloaked(&self) -> bool {
    matches!(self.cloak(), CloakState::NotCloaked)
  }

  #[inline]
  pub fn is_cloaked_allied(&self) -> bool {
    matches!(self.cloak(), CloakState::CloakedAllied)
  }

  // ---------------------------------------------------------------------------
  // Properties - Tech Requirement
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn tech_requirement_id(&self) -> Option<UnitTypeId> {
    self.data.tech_requirement_id().into_option()
  }

  #[inline]
  pub fn tech_requirement(&self) -> Option<&UnitTypeData> {
    self
      .tech_requirement_id()
      .and_then(|id| self.root.try_unit(id))
  }

  #[inline]
  pub fn tech_requirement_alias_ids(&self) -> &[UnitTypeId] {
    self.data.tech_alias_ids()
  }

  #[inline]
  pub fn tech_requirement_alias(&self) -> impl Iterator<Item = &UnitTypeData> {
    self
      .data
      .tech_alias_ids()
      .iter()
      .filter_map(|id| self.root.try_unit(*id))
  }

  #[inline]
  pub fn tech_requirement_addon(&self) -> bool {
    self.data.require_attached()
  }

  // ---------------------------------------------------------------------------
  // Properties - Build Process
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn creator_id(&self) -> Option<AbilityId> {
    self.data.ability_id().into_option()
  }

  #[inline]
  pub fn creator(&self) -> Option<&AbilityData> {
    self.creator_id().and_then(|id| self.root.try_ability(id))
  }

  #[inline]
  pub fn build_time(&self) -> Duration {
    Duration::from_secs_f32(self.data.build_time().into_f32())
  }

  #[inline]
  pub fn build_duration(&self) -> Duration {
    self
      .build_time()
      .mul_f32(1.0 - self.build_progress().into_f32())
  }

  #[inline]
  pub fn build_progress(&self) -> Percent {
    self.unit.progress()
  }

  #[inline]
  pub fn build_complete(&self) -> bool {
    self.build_progress().epsilon_eq(1.0)
  }

  #[inline]
  pub fn build_pending(&self) -> bool {
    !self.build_complete()
  }

  // ---------------------------------------------------------------------------
  // Properties - Position
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn point(&self) -> Point3DF {
    self.unit.point()
  }

  #[inline]
  pub fn point_2d(&self) -> PointF {
    self.point().into_2d()
  }

  #[inline]
  pub fn in_range(&self, point: PointF, range: f32) -> bool {
    self.point_2d().distance(point) <= range
  }

  #[inline]
  pub fn facing(&self) -> Float {
    self.unit.facing()
  }

  #[inline]
  pub fn radius(&self) -> Float {
    self.unit.radius()
  }

  // ---------------------------------------------------------------------------
  // Properties - Attributes
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn attributes(&self) -> &[Attribute] {
    self.data.attributes()
  }

  #[inline]
  pub fn is_light(&self) -> bool {
    self.data.is_light()
  }

  #[inline]
  pub fn is_armored(&self) -> bool {
    self.data.is_armored()
  }

  #[inline]
  pub fn is_biological(&self) -> bool {
    self.data.is_biological()
  }

  #[inline]
  pub fn is_mechanical(&self) -> bool {
    self.data.is_mechanical()
  }

  #[inline]
  pub fn is_robotic(&self) -> bool {
    self.data.is_robotic()
  }

  #[inline]
  pub fn is_psionic(&self) -> bool {
    self.data.is_psionic()
  }

  #[inline]
  pub fn is_massive(&self) -> bool {
    self.data.is_massive()
  }

  #[inline]
  pub fn is_structure(&self) -> bool {
    self.data.is_structure()
  }

  #[inline]
  pub fn is_hover(&self) -> bool {
    self.data.is_hover()
  }

  #[inline]
  pub fn is_heroic(&self) -> bool {
    self.data.is_heroic()
  }

  #[inline]
  pub fn is_summoned(&self) -> bool {
    self.data.is_summoned()
  }

  // ---------------------------------------------------------------------------
  // Properties - Conditions
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn available(&self) -> bool {
    self.data.available()
  }

  #[inline]
  pub fn has_vespene(&self) -> bool {
    self.data.has_vespene()
  }

  #[inline]
  pub fn has_minerals(&self) -> bool {
    self.data.has_minerals()
  }

  #[inline]
  pub fn is_selected(&self) -> bool {
    self.unit.is_selected()
  }

  #[inline]
  pub fn is_on_screen(&self) -> bool {
    self.unit.is_on_screen()
  }

  #[inline]
  pub fn is_blip(&self) -> bool {
    self.unit.is_blip()
  }

  #[inline]
  pub fn is_powered(&self) -> bool {
    self.unit.is_powered()
  }

  #[inline]
  pub fn is_active(&self) -> bool {
    self.unit.is_active()
  }

  // Not relevant for snapshots
  #[inline]
  pub fn is_flying(&self) -> bool {
    self.unit.is_flying()
  }

  // Not relevant for snapshots
  #[inline]
  pub fn is_burrowed(&self) -> bool {
    self.unit.is_burrowed()
  }

  // Not relevant for snapshots
  #[inline]
  pub fn is_hallucination(&self) -> bool {
    self.unit.is_hallucination()
  }

  // ---------------------------------------------------------------------------
  // Properties - Stats
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn sight_range(&self) -> Float {
    self.data.sight_range()
  }

  #[inline]
  pub fn detect_range(&self) -> Float {
    self.unit.detect_range()
  }

  #[inline]
  pub fn radar_range(&self) -> Float {
    self.unit.radar_range()
  }

  #[inline]
  pub fn weapons(&self) -> &[Weapon] {
    self.data.weapons()
  }

  #[inline]
  pub fn armor(&self) -> Float {
    self.data.armor()
  }

  #[inline]
  pub fn speed(&self) -> Float {
    self.data.movement_speed()
  }

  // Not relevant for snapshots
  #[inline]
  pub fn health(&self) -> Stat<Float> {
    Stat::new(self.unit.health(), self.unit.health_max())
  }

  // Not relevant for snapshots
  #[inline]
  pub fn shield(&self) -> Stat<Float> {
    Stat::new(self.unit.shield(), self.unit.shield_max())
  }

  // Not relevant for snapshots
  #[inline]
  pub fn energy(&self) -> Stat<Float> {
    Stat::new(self.unit.energy(), self.unit.energy_max())
  }

  // ---------------------------------------------------------------------------
  // Properties - Upgrades
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn attack_upgrade_level(&self) -> u32 {
    self.unit.attack_upgrade_level() as u32
  }

  #[inline]
  pub fn armor_upgrade_level(&self) -> u32 {
    self.unit.armor_upgrade_level() as u32
  }

  #[inline]
  pub fn shield_upgrade_level(&self) -> u32 {
    self.unit.shield_upgrade_level() as u32
  }

  // ---------------------------------------------------------------------------
  // Properties - Food
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn food_required(&self) -> Float {
    self.data.food_required()
  }

  #[inline]
  pub fn food_provided(&self) -> Float {
    self.data.food_provided()
  }

  // ---------------------------------------------------------------------------
  // Properties - Cargo
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn cargo_size(&self) -> u32 {
    self.data.cargo_size()
  }

  #[inline]
  pub fn cargo_space(&self) -> Stat<i32> {
    Stat::new(self.unit.cargo_space_taken(), self.unit.cargo_space_max())
  }

  // ---------------------------------------------------------------------------
  // Properties - Resources
  // ---------------------------------------------------------------------------

  // Not relevant for snapshots
  #[inline]
  pub fn mineral_contents(&self) -> u32 {
    self.unit.mineral_contents() as u32
  }

  // Not relevant for snapshots
  #[inline]
  pub fn vespene_contents(&self) -> u32 {
    self.unit.vespene_contents() as u32
  }

  // Not relevant for enemies
  #[inline]
  pub fn assigned_harvesters(&self) -> u32 {
    self.unit.assigned_harvesters() as u32
  }

  // Not relevant for enemies
  #[inline]
  pub fn expected_harvesters(&self) -> u32 {
    self.unit.ideal_harvesters() as u32
  }

  // ---------------------------------------------------------------------------
  // Properties - Addon
  // ---------------------------------------------------------------------------

  // Not relevant for enemies
  #[inline]
  pub fn addon_tag(&self) -> Option<UnitTag> {
    self.unit.add_on_tag().into_option()
  }

  // Not relevant for enemies
  #[inline]
  pub fn addon(&self) -> Option<Self> {
    self
      .addon_tag()
      .and_then(|tag| self.root.units().find_by_tag(tag))
  }

  // Not relevant for enemies
  #[inline]
  pub fn addon_techlab(&self) -> Option<Self> {
    self.addon().filter(|unit| unit.kind().is_techlab())
  }

  // Not relevant for enemies
  #[inline]
  pub fn addon_reactor(&self) -> Option<Self> {
    self.addon().filter(|unit| unit.kind().is_reactor())
  }

  // ---------------------------------------------------------------------------
  // Properties - Orders
  // ---------------------------------------------------------------------------

  // Not relevant for enemies
  #[inline]
  pub fn orders(&self) -> &[UnitOrder] {
    self.unit.orders()
  }

  // Not relevant for enemies
  #[inline]
  pub fn is_idle(&self) -> bool {
    let orders: &[UnitOrder] = self.orders();

    match self.addon_reactor() {
      Some(_) => orders.len() < 2,
      None => orders.is_empty(),
    }
  }

  // Not relevant for enemies
  #[inline]
  pub fn is_busy(&self) -> bool {
    !self.is_idle()
  }
}

impl Debug for UnitRef<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("UnitRef")
      .field("kind", &self.kind())
      .field("cost", &self.cost())
      .field("owner", &self.owner())
      .field("tag", &self.tag())
      .field("alliance", &self.alliance())
      .field("display", &self.display())
      .field("cloak", &self.cloak())
      .field("world_position", &self.point())
      .field("build_progress", &self.build_progress())
      .field("attributes", &self.attributes())
      .field("addon_tag", &self.addon_tag())
      .field("orders", &self.orders())
      .finish_non_exhaustive()
  }
}
