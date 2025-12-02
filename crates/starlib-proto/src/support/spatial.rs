use prost::Enumeration;
use prost::Message;
use prost::Oneof;

use crate::support::ImageData;
use crate::support::PointI;
use crate::support::RectangleI;

// -----------------------------------------------------------------------------
// Observation - Feature Layer
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct ObservationFeatureLayer {
  #[prost(message, tag = "1")]
  pub renders: Option<FeatureLayers>,
  #[prost(message, tag = "2")]
  pub minimap_renders: Option<FeatureLayersMinimap>,
}

#[derive(Clone, Message)]
pub struct FeatureLayers {
  #[prost(message, tag = "1")]
  pub height_map: Option<ImageData>,
  #[prost(message, tag = "2")]
  pub visibility_map: Option<ImageData>,
  #[prost(message, tag = "3")]
  pub creep: Option<ImageData>,
  #[prost(message, tag = "4")]
  pub power: Option<ImageData>,
  #[prost(message, tag = "5")]
  pub player_id: Option<ImageData>,
  #[prost(message, tag = "6")]
  pub unit_type: Option<ImageData>,
  #[prost(message, tag = "7")]
  pub selected: Option<ImageData>,
  #[prost(message, tag = "8")]
  pub unit_hit_points: Option<ImageData>,
  #[prost(message, tag = "17")]
  pub unit_hit_points_ratio: Option<ImageData>,
  #[prost(message, tag = "9")]
  pub unit_energy: Option<ImageData>,
  #[prost(message, tag = "18")]
  pub unit_energy_ratio: Option<ImageData>,
  #[prost(message, tag = "10")]
  pub unit_shields: Option<ImageData>,
  #[prost(message, tag = "19")]
  pub unit_shields_ratio: Option<ImageData>,
  #[prost(message, tag = "11")]
  pub player_relative: Option<ImageData>,
  #[prost(message, tag = "14")]
  pub unit_density_aa: Option<ImageData>,
  #[prost(message, tag = "15")]
  pub unit_density: Option<ImageData>,
  #[prost(message, tag = "20")]
  pub effects: Option<ImageData>,
  #[prost(message, tag = "21")]
  pub hallucinations: Option<ImageData>,
  #[prost(message, tag = "22")]
  pub cloaked: Option<ImageData>,
  #[prost(message, tag = "23")]
  pub blip: Option<ImageData>,
  #[prost(message, tag = "24")]
  pub buffs: Option<ImageData>,
  #[prost(message, tag = "26")]
  pub buff_duration: Option<ImageData>,
  #[prost(message, tag = "25")]
  pub active: Option<ImageData>,
  #[prost(message, tag = "27")]
  pub build_progress: Option<ImageData>,
  #[prost(message, tag = "28")]
  pub buildable: Option<ImageData>,
  #[prost(message, tag = "29")]
  pub pathable: Option<ImageData>,
  #[prost(message, tag = "30")]
  pub placeholder: Option<ImageData>,
}

#[derive(Clone, Message)]
pub struct FeatureLayersMinimap {
  #[prost(message, tag = "1")]
  pub height_map: Option<ImageData>,
  #[prost(message, tag = "2")]
  pub visibility_map: Option<ImageData>,
  #[prost(message, tag = "3")]
  pub creep: Option<ImageData>,
  #[prost(message, tag = "4")]
  pub camera: Option<ImageData>,
  #[prost(message, tag = "5")]
  pub player_id: Option<ImageData>,
  #[prost(message, tag = "6")]
  pub player_relative: Option<ImageData>,
  #[prost(message, tag = "7")]
  pub selected: Option<ImageData>,
  #[prost(message, tag = "9")]
  pub alerts: Option<ImageData>,
  #[prost(message, tag = "10")]
  pub buildable: Option<ImageData>,
  #[prost(message, tag = "11")]
  pub pathable: Option<ImageData>,

  #[prost(message, tag = "8")]
  pub unit_type: Option<ImageData>,
}

// -----------------------------------------------------------------------------
// Observation - Rendered
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct ObservationRender {
  #[prost(message, tag = "1")]
  pub map: Option<ImageData>,
  #[prost(message, tag = "2")]
  pub minimap: Option<ImageData>,
}

// -----------------------------------------------------------------------------
// Action
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct ActionSpatial {
  #[prost(oneof = "ActionSpatialVariant", tags = "1,2,3,4")]
  pub action: Option<ActionSpatialVariant>,
}

#[derive(Clone, Oneof)]
pub enum ActionSpatialVariant {
  #[prost(message, tag = "1")]
  UnitCommand(ActionSpatialUnitCommand),
  #[prost(message, tag = "2")]
  CameraMove(ActionSpatialCameraMove),
  #[prost(message, tag = "3")]
  UnitSelectionPoint(ActionSpatialUnitSelectionPoint),
  #[prost(message, tag = "4")]
  UnitSelectionRect(ActionSpatialUnitSelectionRect),
}

#[derive(Clone, Copy, Oneof)]
pub enum ActionSpatialUnitCommandTarget {
  #[prost(message, tag = "2")]
  TargetScreenCoord(PointI),
  #[prost(message, tag = "3")]
  TargetMinimapCoord(PointI),
}

#[derive(Clone, Copy, Message)]
pub struct ActionSpatialUnitCommand {
  #[prost(int32, tag = "1")]
  pub ability_id: i32,
  #[prost(oneof = "ActionSpatialUnitCommandTarget", tags = "2,3")]
  pub target: Option<ActionSpatialUnitCommandTarget>,
  #[prost(bool, tag = "4")]
  pub queue_command: bool,
}

#[derive(Clone, Copy, Message)]
pub struct ActionSpatialCameraMove {
  #[prost(message, tag = "1")]
  pub center_minimap: Option<PointI>,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ActionSpatialUnitSelectionPointType {
  NULL = 0,
  Select = 1,
  Toggle = 2,
  AllType = 3,
  AddAllType = 4,
}

#[derive(Clone, Copy, Message)]
pub struct ActionSpatialUnitSelectionPoint {
  #[prost(message, tag = "1")]
  pub selection_screen_coord: Option<PointI>,
  #[prost(enumeration = "ActionSpatialUnitSelectionPointType", tag = "2")]
  pub kind: i32,
}

#[derive(Clone, Message)]
pub struct ActionSpatialUnitSelectionRect {
  #[prost(repeated, message, tag = "1")]
  pub selection_screen_coord: Vec<RectangleI>,
  #[prost(bool, tag = "2")]
  pub selection_add: bool,
}
