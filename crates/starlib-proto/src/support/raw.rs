use prost::Enumeration;
use prost::Message;
use prost::Oneof;

use crate::support::ImageData;
use crate::support::Point;
use crate::support::Point2D;
use crate::support::RectangleI;
use crate::support::Size2DI;

// -----------------------------------------------------------------------------
// Start
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct StartRaw {
  #[prost(message, tag = "1")]
  pub map_size: Option<Size2DI>,
  #[prost(message, tag = "2")]
  pub pathing_grid: Option<ImageData>,
  #[prost(message, tag = "3")]
  pub terrain_height: Option<ImageData>,
  #[prost(message, tag = "4")]
  pub placement_grid: Option<ImageData>,
  #[prost(message, tag = "5")]
  pub playable_area: Option<RectangleI>,
  #[prost(repeated, message, tag = "6")]
  pub start_locations: Vec<Point2D>,
}

// -----------------------------------------------------------------------------
// Observation
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct ObservationRaw {
  #[prost(message, tag = "1")]
  pub player: Option<PlayerRaw>,
  #[prost(repeated, message, tag = "2")]
  pub units: Vec<Unit>,
  #[prost(message, tag = "3")]
  pub map_state: Option<MapState>,
  #[prost(message, tag = "4")]
  pub event: Option<Event>,
  #[prost(repeated, message, tag = "5")]
  pub effects: Vec<Effect>,
  #[prost(repeated, message, tag = "6")]
  pub radar: Vec<RadarRing>,
}

#[derive(Clone, Copy, Message)]
pub struct RadarRing {
  #[prost(message, tag = "1")]
  pub pos: Option<Point>,
  #[prost(float, tag = "2")]
  pub radius: f32,
}

#[derive(Clone, Copy, Message)]
pub struct PowerSource {
  #[prost(message, tag = "1")]
  pub pos: Option<Point>,
  #[prost(float, tag = "2")]
  pub radius: f32,
  #[prost(uint64, tag = "3")]
  pub tag: u64,
}

#[derive(Clone, Message)]
pub struct PlayerRaw {
  #[prost(repeated, message, tag = "1")]
  pub power_sources: Vec<PowerSource>,
  #[prost(message, tag = "2")]
  pub camera: Option<Point>,
  #[prost(repeated, uint32, tag = "3")]
  pub upgrade_ids: Vec<u32>,
}

#[derive(Clone, Copy, Oneof)]
pub enum Target {
  #[prost(message, tag = "2")]
  TargetWorldSpacePos(Point2D),
  #[prost(uint64, tag = "3")]
  TargetUnitTag(u64),
}

#[derive(Clone, Copy, Message)]
pub struct UnitOrder {
  #[prost(uint32, tag = "1")]
  pub ability_id: u32,
  #[prost(oneof = "Target", tags = "2,3")]
  pub target: Option<Target>,
  #[prost(float, tag = "4")]
  pub progress: f32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum DisplayType {
  NULL = 0,
  Visible = 1,
  Snapshot = 2,
  Hidden = 3,
  Placeholder = 4,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum Alliance {
  NULL = 0,
  User = 1,
  Ally = 2,
  Neutral = 3,
  Enemy = 4,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum CloakState {
  CloakedUnknown = 0,
  Cloaked = 1,
  CloakedDetected = 2,
  NotCloaked = 3,
  CloakedAllied = 4,
}

#[derive(Clone, Copy, Message)]
pub struct PassengerUnit {
  #[prost(uint64, tag = "1")]
  pub tag: u64,
  #[prost(float, tag = "2")]
  pub health: f32,
  #[prost(float, tag = "3")]
  pub health_max: f32,
  #[prost(float, tag = "4")]
  pub shield: f32,
  #[prost(float, tag = "7")]
  pub shield_max: f32,
  #[prost(float, tag = "5")]
  pub energy: f32,
  #[prost(float, tag = "8")]
  pub energy_max: f32,
  #[prost(uint32, tag = "6")]
  pub unit_type: u32,
}

#[derive(Clone, Copy, Message)]
pub struct RallyTarget {
  #[prost(message, tag = "1")]
  pub point: Option<Point>,
  #[prost(uint64, tag = "2")]
  pub tag: u64,
}

#[derive(Clone, Message)]
pub struct Unit {
  #[prost(enumeration = "DisplayType", tag = "1")]
  pub display_type: i32,
  #[prost(enumeration = "Alliance", tag = "2")]
  pub alliance: i32,

  #[prost(uint64, tag = "3")]
  pub tag: u64,
  #[prost(uint32, tag = "4")]
  pub unit_type: u32,
  #[prost(int32, tag = "5")]
  pub owner: i32,

  #[prost(message, tag = "6")]
  pub pos: Option<Point>,
  #[prost(float, tag = "7")]
  pub facing: f32,
  #[prost(float, tag = "8")]
  pub radius: f32,
  #[prost(float, tag = "9")]
  pub build_progress: f32,
  #[prost(enumeration = "CloakState", tag = "10")]
  pub cloak: i32,
  #[prost(repeated, uint32, tag = "27")]
  pub buff_ids: Vec<u32>,

  #[prost(float, tag = "31")]
  pub detect_range: f32,
  #[prost(float, tag = "32")]
  pub radar_range: f32,

  #[prost(bool, tag = "11")]
  pub is_selected: bool,
  #[prost(bool, tag = "12")]
  pub is_on_screen: bool,
  #[prost(bool, tag = "13")]
  pub is_blip: bool,
  #[prost(bool, tag = "35")]
  pub is_powered: bool,
  #[prost(bool, tag = "39")]
  pub is_active: bool,
  #[prost(int32, tag = "40")]
  pub attack_upgrade_level: i32,
  #[prost(int32, tag = "41")]
  pub armor_upgrade_level: i32,
  #[prost(int32, tag = "42")]
  pub shield_upgrade_level: i32,

  #[prost(float, tag = "14")]
  pub health: f32,
  #[prost(float, tag = "15")]
  pub health_max: f32,
  #[prost(float, tag = "16")]
  pub shield: f32,
  #[prost(float, tag = "36")]
  pub shield_max: f32,
  #[prost(float, tag = "17")]
  pub energy: f32,
  #[prost(float, tag = "37")]
  pub energy_max: f32,
  #[prost(int32, tag = "18")]
  pub mineral_contents: i32,
  #[prost(int32, tag = "19")]
  pub vespene_contents: i32,
  #[prost(bool, tag = "20")]
  pub is_flying: bool,
  #[prost(bool, tag = "21")]
  pub is_burrowed: bool,
  #[prost(bool, tag = "38")]
  pub is_hallucination: bool,

  #[prost(repeated, message, tag = "22")]
  pub orders: Vec<UnitOrder>,
  #[prost(uint64, tag = "23")]
  pub add_on_tag: u64,
  #[prost(repeated, message, tag = "24")]
  pub passengers: Vec<PassengerUnit>,
  #[prost(int32, tag = "25")]
  pub cargo_space_taken: i32,
  #[prost(int32, tag = "26")]
  pub cargo_space_max: i32,
  #[prost(int32, tag = "28")]
  pub assigned_harvesters: i32,
  #[prost(int32, tag = "29")]
  pub ideal_harvesters: i32,
  #[prost(float, tag = "30")]
  pub weapon_cooldown: f32,
  #[prost(uint64, tag = "34")]
  pub engaged_target_tag: u64,
  #[prost(int32, tag = "43")]
  pub buff_duration_remain: i32,
  #[prost(int32, tag = "44")]
  pub buff_duration_max: i32,
  #[prost(repeated, message, tag = "45")]
  pub rally_targets: Vec<RallyTarget>,
}

#[derive(Clone, Message)]
pub struct MapState {
  #[prost(message, tag = "1")]
  pub visibility: Option<ImageData>,
  #[prost(message, tag = "2")]
  pub creep: Option<ImageData>,
}

#[derive(Clone, Message)]
pub struct Event {
  #[prost(repeated, uint64, tag = "1")]
  pub dead_units: Vec<u64>,
}

#[derive(Clone, Message)]
pub struct Effect {
  #[prost(uint32, tag = "1")]
  pub effect_id: u32,
  #[prost(repeated, message, tag = "2")]
  pub pos: Vec<Point2D>,
  #[prost(enumeration = "Alliance", tag = "3")]
  pub alliance: i32,
  #[prost(int32, tag = "4")]
  pub owner: i32,
  #[prost(float, tag = "5")]
  pub radius: f32,
}

// -----------------------------------------------------------------------------
// Action
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct ActionRaw {
  #[prost(oneof = "ActionRawVariant", tags = "1,2,3")]
  pub action: Option<ActionRawVariant>,
}

#[derive(Clone, Oneof)]
pub enum ActionRawVariant {
  #[prost(message, tag = "1")]
  UnitCommand(ActionRawUnitCommand),
  #[prost(message, tag = "2")]
  CameraMove(ActionRawCameraMove),
  #[prost(message, tag = "3")]
  ToggleAutocast(ActionRawToggleAutocast),
}

#[derive(Clone, Message)]
pub struct ActionRawUnitCommand {
  #[prost(int32, tag = "1")]
  pub ability_id: i32,
  #[prost(oneof = "Target", tags = "2,3")]
  pub target: Option<Target>,
  #[prost(repeated, uint64, tag = "4")]
  pub unit_tags: Vec<u64>,
  #[prost(bool, tag = "5")]
  pub queue_command: bool,
}

#[derive(Clone, Copy, Message)]
pub struct ActionRawCameraMove {
  #[prost(message, tag = "1")]
  pub center_world_space: Option<Point>,
}

#[derive(Clone, Message)]
pub struct ActionRawToggleAutocast {
  #[prost(int32, tag = "1")]
  pub ability_id: i32,
  #[prost(repeated, uint64, tag = "2")]
  pub unit_tags: Vec<u64>,
}
