use crate::bridge::Image;
use crate::core::Float;
use crate::core::Percent;
use crate::core::PlayerTag;
use crate::core::Point3DF;
use crate::core::PointF;
use crate::core::RectI;
use crate::core::SizeI;
use crate::core::UnitTag;
use crate::stable::AbilityId;
use crate::stable::BuffId;
use crate::stable::EffectId;
use crate::stable::UnitTypeId;
use crate::stable::UpgradeId;

// -----------------------------------------------------------------------------
// Start
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(StartRaw)]
  pub struct StartRaw {
    map_size: SizeI = proto::map_size,
    pathing_grid: Image ref = proto::pathing_grid,
    terrain_height: Image ref = proto::terrain_height,
    placement_grid: Image ref = proto::placement_grid,
    playable_area: RectI = proto::playable_area,
    start_locations: PointF vec = proto::start_locations,
  }
}

// -----------------------------------------------------------------------------
// Observation
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(ObservationRaw)]
  pub struct ObservationRaw {
    player: PlayerRaw ref = proto::player,
    units: Unit vec = proto::units,
    map_state: MapState ref = proto::map_state,
    event: Event ref = proto::event,
    effects: Effect vec = proto::effects,
    radar: RadarRing vec = proto::radar,
  }
}

define_bridge_message! {
  #[bridge(RadarRing)]
  pub struct RadarRing {
    point: Point3DF = proto::pos,
    radius: Float = proto::radius,
  }
}

define_bridge_message! {
  #[bridge(PowerSource)]
  pub struct PowerSource {
    point: Point3DF = proto::pos,
    radius: Float = proto::radius,
    tag: UnitTag = proto::tag,
  }
}

define_bridge_message! {
  #[bridge(PlayerRaw)]
  pub struct PlayerRaw {
    power_sources: PowerSource vec = proto::power_sources,
    camera: Point3DF = proto::camera,
    upgrade_ids: UpgradeId vec = proto::upgrade_ids,
  }
}

define_bridge_oneof! {
  #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
  #[bridge(Target)]
  pub enum Target {
    None,
    Unit(UnitTag) = proto::TargetUnitTag,
    Point(PointF) = proto::TargetWorldSpacePos,
  }
}

define_bridge_message! {
  #[bridge(UnitOrder)]
  pub struct UnitOrder {
    ability_id: AbilityId = proto::ability_id,
    target: Target ref = proto::target,
    progress: Percent = proto::progress,
  }
}

define_bridge_enum! {
  #[bridge(DisplayType, NULL = Hidden)]
  pub enum DisplayType {
    Visible,
    Snapshot,
    Hidden,
    Placeholder,
  }
}

define_bridge_enum! {
  #[bridge(Alliance, NULL = Neutral)]
  pub enum Alliance {
    User,
    Ally,
    Neutral,
    Enemy,
  }
}

define_bridge_enum! {
  #[bridge(CloakState)]
  pub enum CloakState {
    CloakedUnknown,
    Cloaked,
    CloakedDetected,
    NotCloaked,
    CloakedAllied,
  }
}

define_bridge_message! {
  #[bridge(PassengerUnit)]
  pub struct Passenger {
    kind: UnitTypeId = proto::unit_type,
    tag: UnitTag = proto::tag,
    health: Float = proto::health,
    health_max: Float = proto::health_max,
    shield: Float = proto::shield,
    shield_max: Float = proto::shield_max,
    energy: Float = proto::energy,
    energy_max: Float = proto::energy_max,
  }
}

define_bridge_message! {
  #[bridge(RallyTarget)]
  pub struct RallyTarget {
    point: Point3DF = proto::point,
    tag: UnitTag = proto::tag,
  }
}

define_bridge_message! {
  #[bridge(Unit)]
  pub struct Unit {
    display: DisplayType = proto::display_type,
    alliance: Alliance = proto::alliance,

    tag: UnitTag = proto::tag,
    kind: UnitTypeId = proto::unit_type,
    owner: PlayerTag = proto::owner,

    point: Point3DF = proto::pos,
    facing: Float = proto::facing,
    radius: Float = proto::radius,
    progress: Percent = proto::build_progress,
    cloak: CloakState = proto::cloak,
    buff_ids: BuffId vec = proto::buff_ids,

    detect_range: Float = proto::detect_range,
    radar_range: Float = proto::radar_range,

    is_selected: bool = proto::is_selected,
    is_on_screen: bool = proto::is_on_screen,
    is_blip: bool = proto::is_blip,
    is_powered: bool = proto::is_powered,
    is_active: bool = proto::is_active,
    attack_upgrade_level: i32 = proto::attack_upgrade_level,
    armor_upgrade_level: i32 = proto::armor_upgrade_level,
    shield_upgrade_level: i32 = proto::shield_upgrade_level,

    health: Float = proto::health,
    health_max: Float = proto::health_max,
    shield: Float = proto::shield,
    shield_max: Float = proto::shield_max,
    energy: Float = proto::energy,
    energy_max: Float = proto::energy_max,
    mineral_contents: i32 = proto::mineral_contents,
    vespene_contents: i32 = proto::vespene_contents,
    is_flying: bool = proto::is_flying,
    is_burrowed: bool = proto::is_burrowed,
    is_hallucination: bool = proto::is_hallucination,

    orders: UnitOrder vec = proto::orders,
    add_on_tag: UnitTag = proto::add_on_tag,
    passengers: Passenger vec = proto::passengers,
    cargo_space_taken: i32 = proto::cargo_space_taken,
    cargo_space_max: i32 = proto::cargo_space_max,
    assigned_harvesters: i32 = proto::assigned_harvesters,
    ideal_harvesters: i32 = proto::ideal_harvesters,
    weapon_cooldown: Float = proto::weapon_cooldown,

    engaged_target_tag: UnitTag = proto::engaged_target_tag,
    buff_duration_remain: i32 = proto::buff_duration_remain,
    buff_duration_max: i32 = proto::buff_duration_max,
    rally_targets: RallyTarget vec = proto::rally_targets,
  }
}

define_bridge_message! {
  #[bridge(MapState)]
  pub struct MapState {
    visibility: Image ref = proto::visibility,
    creep: Image ref = proto::creep,
  }
}

define_bridge_message! {
  #[bridge(Event)]
  pub struct Event {
    dead_units: UnitTag vec = proto::dead_units,
  }
}

define_bridge_message! {
  #[bridge(Effect)]
  pub struct Effect {
    id: EffectId = proto::effect_id,
    point: PointF vec = proto::pos,
    alliance: Alliance = proto::alliance,
    owner: PlayerTag = proto::owner,
    radius: Float = proto::radius,
  }
}

// -----------------------------------------------------------------------------
// Action
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(ActionRaw)]
  pub struct ActionRaw {
    action: ActionRawVariant ref = proto::action,
  }
}

define_bridge_oneof! {
  #[bridge(ActionRawVariant)]
  pub enum ActionRawVariant {
    None,
    UnitCommand(ActionRawUnitCommand) = proto::UnitCommand,
    CameraMove(ActionRawCameraMove) = proto::CameraMove,
    ToggleAutocast(ActionRawToggleAutocast) = proto::ToggleAutocast,
  }
}

define_bridge_message! {
  #[bridge(ActionRawUnitCommand)]
  pub struct ActionRawUnitCommand {
    ability_id: AbilityId = proto::ability_id,
    target: Target ref = proto::target,
    unit_tags: UnitTag vec = proto::unit_tags,
    queue_command: bool = proto::queue_command,
  }
}

define_bridge_message! {
  #[bridge(ActionRawCameraMove)]
  pub struct ActionRawCameraMove {
    center_world_space: Point3DF = proto::center_world_space,
  }
}

define_bridge_message! {
  #[bridge(ActionRawToggleAutocast)]
  pub struct ActionRawToggleAutocast {
    ability_id: AbilityId = proto::ability_id,
    unit_tags: UnitTag vec = proto::unit_tags,
  }
}
