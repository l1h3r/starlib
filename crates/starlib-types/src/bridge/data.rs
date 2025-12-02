use crate::bridge::Race;
use crate::core::Cost;
use crate::core::Float;
use crate::stable::AbilityId;
use crate::stable::BuffId;
use crate::stable::EffectId;
use crate::stable::UnitTypeId;
use crate::stable::UpgradeId;

define_bridge_enum! {
  #[bridge(AbilityDataTarget, NULL = None)]
  pub enum AbilityTarget {
    None,
    Point,
    Unit,
    PointOrUnit,
    PointOrNone,
  }
}

define_bridge_message! {
  #[bridge(AbilityData)]
  pub struct AbilityData {
    id_raw: AbilityId = proto::ability_id,
    link_name: String = proto::link_name,
    link_index: u32 = proto::link_index,
    button_name: String = proto::button_name,
    friendly_name: String = proto::friendly_name,
    hotkey: String = proto::hotkey,
    remaps_to_ability_id: AbilityId = proto::remaps_to_ability_id,
    available: bool = proto::available,
    target: AbilityTarget = proto::target,
    allow_minimap: bool = proto::allow_minimap,
    allow_autocast: bool = proto::allow_autocast,
    is_building: bool = proto::is_building,
    footprint_radius: Float = proto::footprint_radius,
    is_instant_placement: bool = proto::is_instant_placement,
    cast_range: Float = proto::cast_range,
  }
}

impl AbilityData {
  #[inline]
  pub fn id(&self) -> AbilityId {
    self
      .remaps_to_ability_id
      .into_option()
      .unwrap_or(self.id_raw)
  }
}

define_bridge_enum! {
  #[bridge(Attribute, NULL = Light)]
  pub enum Attribute {
    Light,
    Armored,
    Biological,
    Mechanical,
    Robotic,
    Psionic,
    Massive,
    Structure,
    Hover,
    Heroic,
    Summoned,
  }
}

define_bridge_message! {
  #[bridge(DamageBonus)]
  pub struct DamageBonus {
    attribute: Attribute = proto::attribute,
    bonus: Float = proto::bonus,
  }
}

define_bridge_enum! {
  #[bridge(TargetType, NULL = Any)]
  pub enum TargetType {
    Ground,
    Air,
    Any,
  }
}

define_bridge_message! {
  #[bridge(Weapon)]
  pub struct Weapon {
    target: TargetType = proto::target,
    damage: Float = proto::damage,
    damage_bonus: DamageBonus ref = proto::damage_bonus,
    attacks: u32 = proto::attacks,
    range: Float = proto::range,
    speed: Float = proto::speed,
  }
}

define_bridge_message! {
  #[bridge(UnitTypeData)]
  pub struct UnitTypeData {
    id: UnitTypeId = proto::unit_id,
    name: String = proto::name,
    available: bool = proto::available,
    cargo_size: u32 = proto::cargo_size,
    mineral_cost: u32 = proto::mineral_cost,
    vespene_cost: u32 = proto::vespene_cost,
    food_required: Float = proto::food_required,
    food_provided: Float = proto::food_provided,
    ability_id: AbilityId = proto::ability_id,
    race: Race = proto::race,
    build_time: Float = proto::build_time,
    has_vespene: bool = proto::has_vespene,
    has_minerals: bool = proto::has_minerals,
    sight_range: Float = proto::sight_range,

    tech_alias_ids: UnitTypeId vec = proto::tech_alias,
    unit_alias_id: UnitTypeId = proto::unit_alias,

    tech_requirement_id: UnitTypeId = proto::tech_requirement,
    require_attached: bool = proto::require_attached,

    attributes: Attribute vec = proto::attributes,
    movement_speed: Float = proto::movement_speed,
    armor: Float = proto::armor,
    weapons: Weapon vec = proto::weapons,
  }
}

impl UnitTypeData {
  #[inline]
  pub const fn cost(&self) -> Cost {
    Cost::new(self.mineral_cost, self.vespene_cost, self.food_required)
  }

  #[inline]
  pub fn is_light(&self) -> bool {
    self.attributes.contains(&Attribute::Light)
  }

  #[inline]
  pub fn is_armored(&self) -> bool {
    self.attributes.contains(&Attribute::Armored)
  }

  #[inline]
  pub fn is_biological(&self) -> bool {
    self.attributes.contains(&Attribute::Biological)
  }

  #[inline]
  pub fn is_mechanical(&self) -> bool {
    self.attributes.contains(&Attribute::Mechanical)
  }

  #[inline]
  pub fn is_robotic(&self) -> bool {
    self.attributes.contains(&Attribute::Robotic)
  }

  #[inline]
  pub fn is_psionic(&self) -> bool {
    self.attributes.contains(&Attribute::Psionic)
  }

  #[inline]
  pub fn is_massive(&self) -> bool {
    self.attributes.contains(&Attribute::Massive)
  }

  #[inline]
  pub fn is_structure(&self) -> bool {
    self.attributes.contains(&Attribute::Structure)
  }

  #[inline]
  pub fn is_hover(&self) -> bool {
    self.attributes.contains(&Attribute::Hover)
  }

  #[inline]
  pub fn is_heroic(&self) -> bool {
    self.attributes.contains(&Attribute::Heroic)
  }

  #[inline]
  pub fn is_summoned(&self) -> bool {
    self.attributes.contains(&Attribute::Summoned)
  }
}

define_bridge_message! {
  #[bridge(UpgradeData)]
  pub struct UpgradeData {
    id: UpgradeId = proto::upgrade_id,
    name: String = proto::name,
    mineral_cost: u32 = proto::mineral_cost,
    vespene_cost: u32 = proto::vespene_cost,
    research_time: Float = proto::research_time,
    ability_id: AbilityId = proto::ability_id,
  }
}

impl UpgradeData {
  #[inline]
  pub const fn cost(&self) -> Cost {
    Cost::new(self.mineral_cost, self.vespene_cost, Float::from_f32(0.))
  }
}

define_bridge_message! {
  #[bridge(BuffData)]
  pub struct BuffData {
    id: BuffId = proto::buff_id,
    name: String = proto::name,
  }
}

define_bridge_message! {
  #[bridge(EffectData)]
  pub struct EffectData {
    id: EffectId = proto::effect_id,
    name: String = proto::name,
    friendly_name: String = proto::friendly_name,
    radius: Float = proto::radius,
  }
}
