use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::time::Duration;

use crate::bridge::AbilityData;
use crate::bridge::Alliance;
use crate::bridge::Attribute;
use crate::bridge::BuffData;
use crate::bridge::CloakState;
use crate::bridge::DisplayType;
use crate::bridge::Race;
use crate::bridge::RallyTarget;
use crate::bridge::Target;
use crate::bridge::Unit;
use crate::bridge::UnitOrder;
use crate::bridge::UnitTypeData;
use crate::bridge::Weapon;
use crate::core::Cost;
use crate::core::Float;
use crate::core::Percent;
use crate::core::PlayerTag;
use crate::core::Stat;
use crate::core::UnitTag;
use crate::game::GameState;
use crate::game::PassengerRef;
use crate::game::UnitIter;
use crate::geom::Point3DF;
use crate::geom::PointF;
use crate::geom::RectF;
use crate::stable::AbilityId;
use crate::stable::BuffId;
use crate::stable::UnitTypeId;

// -----------------------------------------------------------------------------
// Temp Ref
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct TempRef<'a> {
  data: &'a UnitTypeData,
  unit: &'a Unit,
}

impl<'a> TempRef<'a> {
  #[inline]
  pub fn new(root: &'a GameState, unit: &'a Unit) -> Self {
    Self {
      data: root.unit(unit.kind()),
      unit,
    }
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
  // Properties - Identifiers
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn kind(&self) -> UnitTypeId {
    self.unit.kind()
  }

  #[inline]
  pub fn owner(&self) -> PlayerTag {
    self.unit.owner()
  }

  // ---------------------------------------------------------------------------
  // Properties - Common (Position)
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn point(&self) -> Point3DF {
    self.unit.point()
  }

  #[inline]
  pub fn point_2d(&self) -> PointF {
    self.point().xy()
  }

  #[inline]
  pub fn in_range(&self, point: PointF, range: f32) -> bool {
    self.point_2d().distance(point) <= range
  }

  #[inline]
  pub fn facing(&self) -> f32 {
    self.unit.facing()
  }

  #[inline]
  pub fn radius(&self) -> f32 {
    self.unit.radius()
  }

  #[inline]
  pub fn rect(&self) -> RectF {
    RectF::from_center(self.point_2d(), Float::from_f32(self.radius()))
  }
}

// -----------------------------------------------------------------------------
// Unit Ref
// -----------------------------------------------------------------------------

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
  pub fn train(&self, unit: UnitTypeId, queue: bool) -> bool {
    self.build(unit, Target::None, queue)
  }

  #[inline]
  pub fn build<T>(&self, unit: UnitTypeId, target: T, queue: bool) -> bool
  where
    T: Into<Target>,
  {
    let Some(udata) = self.root.try_unit(unit) else {
      return false;
    };

    self.action(udata.ability_id(), target.into(), queue)
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
  // Properties - Core
  // ---------------------------------------------------------------------------

  #[inline]
  pub const fn root(&self) -> &'a GameState {
    self.root
  }

  #[inline]
  pub const fn data(&self) -> &'a UnitTypeData {
    self.data
  }

  #[inline]
  pub const fn unit(&self) -> &'a Unit {
    self.unit
  }

  #[inline]
  pub fn cost(&self) -> Cost {
    self.data.cost()
  }

  // ---------------------------------------------------------------------------
  // Properties - Data
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn id(&self) -> UnitTypeId {
    self.data.id()
  }

  #[inline]
  pub fn name(&self) -> &str {
    self.data.name()
  }

  #[inline]
  pub fn available(&self) -> bool {
    self.data.available()
  }

  #[inline]
  pub fn cargo_size(&self) -> u32 {
    self.data.cargo_size()
  }

  #[inline]
  pub fn mineral_cost(&self) -> u32 {
    self.data.mineral_cost()
  }

  #[inline]
  pub fn vespene_cost(&self) -> u32 {
    self.data.vespene_cost()
  }

  #[inline]
  pub fn food_required(&self) -> f32 {
    self.data.food_required()
  }

  #[inline]
  pub fn food_provided(&self) -> f32 {
    self.data.food_provided()
  }

  #[inline]
  pub fn ability_id(&self) -> Option<AbilityId> {
    self.data.ability_id().into_option()
  }

  #[inline]
  pub fn ability(&self) -> Option<&'a AbilityData> {
    self.ability_id().and_then(|id| self.root.try_ability(id))
  }

  #[inline]
  pub fn build_time(&self) -> Duration {
    Duration::from_secs_f32(self.data.build_time())
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
  pub fn sight_range(&self) -> f32 {
    self.data.sight_range()
  }

  #[inline]
  pub fn tech_alias_ids(&self) -> &[UnitTypeId] {
    self.data.tech_alias()
  }

  #[inline]
  pub fn tech_alias(&self) -> impl Iterator<Item = &'a UnitTypeData> {
    self
      .tech_alias_ids()
      .iter()
      .filter_map(|id| self.root.try_unit(*id))
  }

  #[inline]
  pub fn unit_alias_id(&self) -> Option<UnitTypeId> {
    self.data.unit_alias().into_option()
  }

  #[inline]
  pub fn unit_alias(&self) -> Option<&'a UnitTypeData> {
    self.unit_alias_id().and_then(|id| self.root.try_unit(id))
  }

  #[inline]
  pub fn tech_requirement_id(&self) -> Option<UnitTypeId> {
    self.data.tech_requirement().into_option()
  }

  #[inline]
  pub fn tech_requirement(&self) -> Option<&'a UnitTypeData> {
    self
      .tech_requirement_id()
      .and_then(|id| self.root.try_unit(id))
  }

  #[inline]
  pub fn require_attached(&self) -> bool {
    self.data.require_attached()
  }

  #[inline]
  pub fn speed(&self) -> f32 {
    self.data.movement_speed()
  }

  #[inline]
  pub fn armor(&self) -> f32 {
    self.data.armor()
  }

  #[inline]
  pub fn weapons(&self) -> &[Weapon] {
    self.data.weapons()
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
  // Properties - Identifiers
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn tag(&self) -> UnitTag {
    self.unit.tag()
  }

  #[inline]
  pub fn kind(&self) -> UnitTypeId {
    self.unit.kind()
  }

  #[inline]
  pub fn owner(&self) -> PlayerTag {
    self.unit.owner()
  }

  // ---------------------------------------------------------------------------
  // Properties - Common (Position)
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn point(&self) -> Point3DF {
    self.unit.point()
  }

  #[inline]
  pub fn point_2d(&self) -> PointF {
    self.point().xy()
  }

  #[inline]
  pub fn in_range(&self, point: PointF, range: f32) -> bool {
    self.point_2d().distance(point) <= range
  }

  #[inline]
  pub fn facing(&self) -> f32 {
    self.unit.facing()
  }

  #[inline]
  pub fn radius(&self) -> f32 {
    self.unit.radius()
  }

  #[inline]
  pub fn rect(&self) -> RectF {
    RectF::from_center(self.point_2d(), Float::from_f32(self.radius()))
  }

  // ---------------------------------------------------------------------------
  // Properties - Common (Build Progress)
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn build_progress(&self) -> Percent {
    Percent::from_f32(self.unit.progress())
  }

  #[inline]
  pub fn build_complete(&self) -> bool {
    self.build_progress().epsilon_eq(1.0)
  }

  #[inline]
  pub fn build_pending(&self) -> bool {
    !self.build_complete()
  }

  #[inline]
  pub fn build_duration(&self) -> Duration {
    self
      .build_time()
      .mul_f32(1.0 - self.build_progress().into_f32())
  }

  // ---------------------------------------------------------------------------
  // Properties - Common (Buffs)
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn buff_ids(&self) -> &[BuffId] {
    self.unit.buff_ids()
  }

  #[inline]
  pub fn buffs(&self) -> impl Iterator<Item = &'a BuffData> {
    self
      .buff_ids()
      .iter()
      .filter_map(|id| self.root.try_buff(*id))
  }

  // ---------------------------------------------------------------------------
  // Properties - Common (Range)
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn detect_range(&self) -> f32 {
    self.unit.detect_range()
  }

  #[inline]
  pub fn radar_range(&self) -> f32 {
    self.unit.radar_range()
  }

  // ---------------------------------------------------------------------------
  // Properties - Common (Misc.)
  // ---------------------------------------------------------------------------

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
  // Properties - Non-Snapshot
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn health(&self) -> Stat<f32> {
    Stat::new(self.unit.health(), self.unit.health_max())
  }

  #[inline]
  pub fn shield(&self) -> Stat<f32> {
    Stat::new(self.unit.shield(), self.unit.shield_max())
  }

  #[inline]
  pub fn energy(&self) -> Stat<f32> {
    Stat::new(self.unit.energy(), self.unit.energy_max())
  }

  #[inline]
  pub fn mineral_contents(&self) -> u32 {
    self.unit.mineral_contents() as u32
  }

  #[inline]
  pub fn vespene_contents(&self) -> u32 {
    self.unit.vespene_contents() as u32
  }

  #[inline]
  pub fn is_flying(&self) -> bool {
    self.unit.is_flying()
  }

  #[inline]
  pub fn is_burrowed(&self) -> bool {
    self.unit.is_burrowed()
  }

  #[inline]
  pub fn is_hallucination(&self) -> bool {
    self.unit.is_hallucination()
  }

  // ---------------------------------------------------------------------------
  // Properties - Non-Enemy (Orders)
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn orders(&self) -> &[UnitOrder] {
    self.unit.orders()
  }

  #[inline]
  pub fn is_idle(&self) -> bool {
    match self.addon_reactor() {
      Some(_) => self.orders().len() < 2,
      None => self.orders().is_empty(),
    }
  }

  #[inline]
  pub fn is_busy(&self) -> bool {
    !self.is_idle()
  }

  // ---------------------------------------------------------------------------
  // Properties - Non-Enemy (Addon)
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn addon_tag(&self) -> Option<UnitTag> {
    self.unit.add_on_tag().into_option()
  }

  #[inline]
  pub fn addon(&self) -> Option<Self> {
    self
      .addon_tag()
      .and_then(|tag| self.root.units().find_by_tag(tag))
  }

  #[inline]
  pub fn addon_techlab(&self) -> Option<Self> {
    self.addon().filter(|unit| unit.kind().is_techlab())
  }

  #[inline]
  pub fn addon_reactor(&self) -> Option<Self> {
    self.addon().filter(|unit| unit.kind().is_reactor())
  }

  // ---------------------------------------------------------------------------
  // Properties - Non-Enemy
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn passengers(&self) -> impl Iterator<Item = PassengerRef<'_>> {
    self.unit.passengers().iter().map(PassengerRef::new)
  }

  #[inline]
  pub fn cargo_space(&self) -> Stat<i32> {
    Stat::new(self.unit.cargo_space_taken(), self.unit.cargo_space_max())
  }

  #[inline]
  pub fn assigned_harvesters(&self) -> u32 {
    self.unit.assigned_harvesters() as u32
  }

  #[inline]
  pub fn expected_harvesters(&self) -> u32 {
    self.unit.ideal_harvesters() as u32
  }

  #[inline]
  pub fn weapon_cooldown(&self) -> f32 {
    self.unit.weapon_cooldown()
  }

  #[inline]
  pub fn engaged_target_tag(&self) -> Option<UnitTag> {
    self.unit.engaged_target_tag().into_option()
  }

  #[inline]
  pub fn buff_duration_remain(&self) -> u32 {
    self.unit.buff_duration_remain() as u32
  }

  #[inline]
  pub fn buff_duration_max(&self) -> u32 {
    self.unit.buff_duration_max() as u32
  }

  #[inline]
  pub fn rally_targets(&self) -> &[RallyTarget] {
    self.unit.rally_targets()
  }
}

impl Debug for UnitRef<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("UnitRef")
      .field("cost", &self.cost())
      .field("tag", &self.tag())
      .field("kind", &self.kind())
      .field("owner", &self.owner())
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
