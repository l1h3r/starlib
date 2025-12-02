use crate::bridge::Image;
use crate::core::PointI;
use crate::core::RectI;
use crate::stable::AbilityId;

// -----------------------------------------------------------------------------
// Observation - Feature Layer
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(ObservationFeatureLayer)]
  pub struct ObservationSpatial {
    minimap: ObservationSpatialMinimap ref = proto::minimap_renders,
    renders: ObservationSpatialRenders ref = proto::renders,
  }
}

define_bridge_message! {
  #[bridge(FeatureLayers)]
  pub struct ObservationSpatialRenders {
    height_map: Image ref = proto::height_map,
    visibility_map: Image ref = proto::visibility_map,
    creep: Image ref = proto::creep,
    power: Image ref = proto::power,
    player_id: Image ref = proto::player_id,
    unit_type: Image ref = proto::unit_type,
    selected: Image ref = proto::selected,
    unit_hit_points: Image ref = proto::unit_hit_points,
    unit_hit_points_ratio: Image ref = proto::unit_hit_points_ratio,
    unit_energy: Image ref = proto::unit_energy,
    unit_energy_ratio: Image ref = proto::unit_energy_ratio,
    unit_shields: Image ref = proto::unit_shields,
    unit_shields_ratio: Image ref = proto::unit_shields_ratio,
    player_relative: Image ref = proto::player_relative,
    unit_density_aa: Image ref = proto::unit_density_aa,
    unit_density: Image ref = proto::unit_density,
    effects: Image ref = proto::effects,
    hallucinations: Image ref = proto::hallucinations,
    cloaked: Image ref = proto::cloaked,
    blip: Image ref = proto::blip,
    buffs: Image ref = proto::buffs,
    buff_duration: Image ref = proto::buff_duration,
    active: Image ref = proto::active,
    build_progress: Image ref = proto::build_progress,
    buildable: Image ref = proto::buildable,
    pathable: Image ref = proto::pathable,
    placeholder: Image ref = proto::placeholder,
  }
}

define_bridge_message! {
  #[bridge(FeatureLayersMinimap)]
  pub struct ObservationSpatialMinimap {
    height_map: Image ref = proto::height_map,
    visibility_map: Image ref = proto::visibility_map,
    creep: Image ref = proto::creep,
    camera: Image ref = proto::camera,
    player_id: Image ref = proto::player_id,
    player_relative: Image ref = proto::player_relative,
    selected: Image ref = proto::selected,
    alerts: Image ref = proto::alerts,
    buildable: Image ref = proto::buildable,
    pathable: Image ref = proto::pathable,
    unit_type: Image ref = proto::unit_type,
  }
}

// -----------------------------------------------------------------------------
// Observation - Rendered
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(ObservationRender)]
  pub struct ObservationRender {
    fullmap: Image ref = proto::map,
    minimap: Image ref = proto::minimap,
  }
}

// -----------------------------------------------------------------------------
// Action
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(ActionSpatial)]
  pub struct ActionSpatial {
    action: ActionSpatialVariant ref = proto::action,
  }
}

define_bridge_oneof! {
  #[bridge(ActionSpatialVariant)]
  pub enum ActionSpatialVariant {
    None,
    UnitCommand(ActionSpatialUnitCommand) = proto::UnitCommand,
    CameraMove(ActionSpatialCameraMove) = proto::CameraMove,
    UnitSelectionPoint(ActionSpatialUnitSelectionPoint) = proto::UnitSelectionPoint,
    UnitSelectionRect(ActionSpatialUnitSelectionRect) = proto::UnitSelectionRect,
  }
}

define_bridge_oneof! {
  #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
  #[bridge(ActionSpatialUnitCommandTarget)]
  pub enum ActionSpatialUnitCommandTarget {
    None,
    Screen(PointI) = proto::TargetScreenCoord,
    Minimap(PointI) = proto::TargetMinimapCoord,
  }
}

define_bridge_message! {
  #[bridge(ActionSpatialUnitCommand)]
  pub struct ActionSpatialUnitCommand {
    ability_id: AbilityId = proto::ability_id,
    target: ActionSpatialUnitCommandTarget = proto::target,
    queue_command: bool = proto::queue_command,
  }
}

define_bridge_message! {
  #[bridge(ActionSpatialCameraMove)]
  pub struct ActionSpatialCameraMove {
    center: PointI = proto::center_minimap,
  }
}

define_bridge_enum! {
  #[bridge(ActionSpatialUnitSelectionPointType, NULL = Select)]
  pub enum ActionSpatialUnitSelectionPointType {
    Select,
    Toggle,
    AllType,
    AddAllType,
  }
}

define_bridge_message! {
  #[bridge(ActionSpatialUnitSelectionPoint)]
  pub struct ActionSpatialUnitSelectionPoint {
    point: PointI = proto::selection_screen_coord,
    point_type: ActionSpatialUnitSelectionPointType = proto::kind,
  }
}

define_bridge_message! {
  #[bridge(ActionSpatialUnitSelectionRect)]
  pub struct ActionSpatialUnitSelectionRect {
    add_units: bool = proto::selection_add,
    selection: RectI vec = proto::selection_screen_coord,
  }
}
