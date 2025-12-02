use prost::Enumeration;
use prost::Message;

use crate::support::Race;

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum AbilityDataTarget {
  NULL = 0,
  None = 1,
  Point = 2,
  Unit = 3,
  PointOrUnit = 4,
  PointOrNone = 5,
}

#[derive(Clone, Message)]
pub struct AbilityData {
  #[prost(uint32, tag = "1")]
  pub ability_id: u32,
  #[prost(string, tag = "2")]
  pub link_name: String,
  #[prost(uint32, tag = "3")]
  pub link_index: u32,
  #[prost(string, tag = "4")]
  pub button_name: String,
  #[prost(string, tag = "5")]
  pub friendly_name: String,
  #[prost(string, tag = "6")]
  pub hotkey: String,
  #[prost(uint32, tag = "7")]
  pub remaps_to_ability_id: u32,
  #[prost(bool, tag = "8")]
  pub available: bool,
  #[prost(enumeration = "AbilityDataTarget", tag = "9")]
  pub target: i32,
  #[prost(bool, tag = "10")]
  pub allow_minimap: bool,
  #[prost(bool, tag = "11")]
  pub allow_autocast: bool,
  #[prost(bool, tag = "12")]
  pub is_building: bool,
  #[prost(float, tag = "13")]
  pub footprint_radius: f32,
  #[prost(bool, tag = "14")]
  pub is_instant_placement: bool,
  #[prost(float, tag = "15")]
  pub cast_range: f32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum Attribute {
  NULL = 0,
  Light = 1,
  Armored = 2,
  Biological = 3,
  Mechanical = 4,
  Robotic = 5,
  Psionic = 6,
  Massive = 7,
  Structure = 8,
  Hover = 9,
  Heroic = 10,
  Summoned = 11,
}

#[derive(Clone, Copy, Message)]
pub struct DamageBonus {
  #[prost(enumeration = "Attribute", tag = "1")]
  pub attribute: i32,
  #[prost(float, tag = "2")]
  pub bonus: f32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum TargetType {
  NULL = 0,
  Ground = 1,
  Air = 2,
  Any = 3,
}

#[derive(Clone, Copy, Message)]
pub struct Weapon {
  #[prost(enumeration = "TargetType", tag = "1")]
  pub target: i32,
  #[prost(float, tag = "2")]
  pub damage: f32,
  #[prost(message, tag = "3")]
  pub damage_bonus: Option<DamageBonus>,
  #[prost(uint32, tag = "4")]
  pub attacks: u32,
  #[prost(float, tag = "5")]
  pub range: f32,
  #[prost(float, tag = "6")]
  pub speed: f32,
}

#[derive(Clone, Message)]
pub struct UnitTypeData {
  #[prost(uint32, tag = "1")]
  pub unit_id: u32,
  #[prost(string, tag = "2")]
  pub name: String,
  #[prost(bool, tag = "3")]
  pub available: bool,
  #[prost(uint32, tag = "4")]
  pub cargo_size: u32,
  #[prost(uint32, tag = "12")]
  pub mineral_cost: u32,
  #[prost(uint32, tag = "13")]
  pub vespene_cost: u32,
  #[prost(float, tag = "14")]
  pub food_required: f32,
  #[prost(float, tag = "18")]
  pub food_provided: f32,
  #[prost(uint32, tag = "15")]
  pub ability_id: u32,
  #[prost(enumeration = "Race", tag = "16")]
  pub race: i32,
  #[prost(float, tag = "17")]
  pub build_time: f32,
  #[prost(bool, tag = "19")]
  pub has_vespene: bool,
  #[prost(bool, tag = "20")]
  pub has_minerals: bool,
  #[prost(float, tag = "25")]
  pub sight_range: f32,
  #[prost(repeated, uint32, tag = "21")]
  pub tech_alias: Vec<u32>,
  #[prost(uint32, tag = "22")]
  pub unit_alias: u32,
  #[prost(uint32, tag = "23")]
  pub tech_requirement: u32,
  #[prost(bool, tag = "24")]
  pub require_attached: bool,
  #[prost(repeated, enumeration = "Attribute", tag = "8")]
  pub attributes: Vec<i32>,
  #[prost(float, tag = "9")]
  pub movement_speed: f32,
  #[prost(float, tag = "10")]
  pub armor: f32,
  #[prost(repeated, message, tag = "11")]
  pub weapons: Vec<Weapon>,
}

#[derive(Clone, Message)]
pub struct UpgradeData {
  #[prost(uint32, tag = "1")]
  pub upgrade_id: u32,
  #[prost(string, tag = "2")]
  pub name: String,
  #[prost(uint32, tag = "3")]
  pub mineral_cost: u32,
  #[prost(uint32, tag = "4")]
  pub vespene_cost: u32,
  #[prost(float, tag = "5")]
  pub research_time: f32,
  #[prost(uint32, tag = "6")]
  pub ability_id: u32,
}

#[derive(Clone, Message)]
pub struct BuffData {
  #[prost(uint32, tag = "1")]
  pub buff_id: u32,
  #[prost(string, tag = "2")]
  pub name: String,
}

#[derive(Clone, Message)]
pub struct EffectData {
  #[prost(uint32, tag = "1")]
  pub effect_id: u32,
  #[prost(string, tag = "2")]
  pub name: String,
  #[prost(string, tag = "3")]
  pub friendly_name: String,
  #[prost(float, tag = "4")]
  pub radius: f32,
}
