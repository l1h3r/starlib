use crate::bridge::Ability;
use crate::bridge::ActionRaw;
use crate::bridge::ActionResult;
use crate::bridge::ActionSpatial;
use crate::bridge::ActionUI;
use crate::bridge::ObservationRaw;
use crate::bridge::ObservationRender;
use crate::bridge::ObservationSpatial;
use crate::bridge::ObservationUI;
use crate::bridge::Race;
use crate::bridge::Score;
use crate::core::Bytes;
use crate::core::Float;
use crate::core::PlayerTag;
use crate::core::PointF;
use crate::core::SizeI;
use crate::core::UnitTag;

// -----------------------------------------------------------------------------
// Assorted
// -----------------------------------------------------------------------------

define_bridge_oneof! {
  #[bridge(Map)]
  pub enum Map {
    None,
    Internal(LocalMap) = proto::LocalMap,
    External(String) = proto::BattleNetMapName,
  }
}

define_bridge_message! {
  #[bridge(LocalMap)]
  pub struct LocalMap {
    path: String = proto::map_path,
    data: Bytes = proto::map_data,
  }
}

define_bridge_oneof! {
  #[bridge(Participation)]
  pub enum Participation {
    None,
    Play(Race) = proto::Race,
    View(PlayerTag) = proto::ObservedPlayerId,
  }
}

define_bridge_message! {
  #[bridge(PortSet)]
  pub struct Ports {
    game: i32 = proto::game_port,
    base: i32 = proto::base_port,
  }
}

define_bridge_message! {
  #[bridge(ChatReceived)]
  pub struct ChatReceived {
    player_id: PlayerTag = proto::player_id,
    message: String = proto::message,
  }
}

define_bridge_message! {
  #[bridge(PlayerInfoExtra)]
  pub struct PlayerInfoExtra {
    player_info: PlayerInfo ref = proto::player_info,
    player_result: PlayerResult ref = proto::player_result,
    player_mmr: i32 = proto::player_mmr,
    player_apm: i32 = proto::player_apm,
  }
}

// -----------------------------------------------------------------------------
// Game Setup
// -----------------------------------------------------------------------------

define_bridge_enum! {
  #[bridge(Difficulty, NULL = Easy)]
  pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    MediumHard,
    Hard,
    Harder,
    VeryHard,
    CheatVision,
    CheatMoney,
    CheatInsane,
  }
}

define_bridge_enum! {
  #[bridge(PlayerType, NULL = Observer)]
  pub enum PlayerType {
    Participant,
    Computer,
    Observer,
  }
}

define_bridge_enum! {
  #[bridge(AIBuild, NULL = RandomBuild)]
  pub enum AiBuild {
    RandomBuild,
    Rush,
    Timing,
    Power,
    Macro,
    Air,
  }
}

define_bridge_message! {
  #[bridge(PlayerSetup)]
  pub struct PlayerSetup {
    kind: PlayerType = proto::kind,
    race: Race = proto::race,
    difficulty: Difficulty = proto::difficulty,
    player_name: String = proto::player_name,
    ai_build: AiBuild = proto::ai_build,
  }
}

define_bridge_message! {
  #[bridge(SpatialCameraSetup)]
  pub struct SpatialCamera {
    resolution: SizeI = proto::resolution,
    minimap_resolution: SizeI = proto::minimap_resolution,
    width: Float = proto::width,
    crop_to_playable_area: bool = proto::crop_to_playable_area,
    allow_cheating_layers: bool = proto::allow_cheating_layers,
  }
}

define_bridge_message! {
  #[bridge(InterfaceOptions)]
  pub struct InterfaceOptions {
    raw: bool = proto::raw,
    score: bool = proto::score,
    feature_layer: SpatialCamera ref = proto::feature_layer,
    render: SpatialCamera ref = proto::render,
    show_cloaked: bool = proto::show_cloaked,
    show_burrowed_shadows: bool = proto::show_burrowed_shadows,
    show_placeholders: bool = proto::show_placeholders,
    raw_affects_selection: bool = proto::raw_affects_selection,
    raw_crop_to_playable_area: bool = proto::raw_crop_to_playable_area,
  }
}

define_bridge_message! {
  #[bridge(PlayerInfo)]
  pub struct PlayerInfo {
    id: PlayerTag = proto::player_id,
    kind: PlayerType = proto::kind,
    name: String = proto::player_name,
    race_expected: Race = proto::race_requested,
    race_received: Race = proto::race_actual,
    ai_difficulty: Difficulty = proto::difficulty,
    ai_build_type: AiBuild = proto::ai_build,
  }
}

// -----------------------------------------------------------------------------
// In-Game
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(PlayerCommon)]
  pub struct PlayerCommon {
    id: PlayerTag = proto::player_id,
    minerals: u32 = proto::minerals,
    vespene: u32 = proto::vespene,
    food_cap: u32 = proto::food_cap,
    food_used: u32 = proto::food_used,
    food_army: u32 = proto::food_army,
    food_workers: u32 = proto::food_workers,
    idle_worker_count: u32 = proto::idle_worker_count,
    army_count: u32 = proto::army_count,
    warp_gate_count: u32 = proto::warp_gate_count,
    larva_count: u32 = proto::larva_count,
  }
}

define_bridge_message! {
  #[bridge(Observation)]
  pub struct Observation {
    game_loop: u32 = proto::game_loop,
    player: PlayerCommon ref = proto::player_common,
    alerts: Alert vec = proto::alerts,
    abilities: Ability vec = proto::abilities,
    score: Score ref = proto::score,

    raw_data: ObservationRaw ref = proto::raw_data,
    feature_layer_data: ObservationSpatial ref = proto::feature_layer_data,
    render_data: ObservationRender ref = proto::render_data,
    ui_data: ObservationUI ref = proto::ui_data,
  }
}

define_bridge_message! {
  #[bridge(Action)]
  pub struct Action {
    action_raw: ActionRaw ref = proto::action_raw,
    action_feature_layer: ActionSpatial ref = proto::action_feature_layer,
    action_render: ActionSpatial ref = proto::action_render,
    action_ui: ActionUI ref = proto::action_ui,
    action_chat: ActionChat ref = proto::action_chat,
    game_loop: u32 = proto::game_loop,
  }
}

define_bridge_enum! {
  #[bridge(ActionChatChannel, NULL = Broadcast)]
  pub enum ActionChatChannel {
    Broadcast,
    Team,
  }
}

define_bridge_message! {
  #[bridge(ActionChat)]
  pub struct ActionChat {
    channel: ActionChatChannel = proto::channel,
    message: String = proto::message,
  }
}

define_bridge_message! {
  #[bridge(ActionError)]
  pub struct ActionError {
    unit_tag: UnitTag = proto::unit_tag,
    ability_id: u64 = proto::ability_id,
    result: ActionResult = proto::result,
  }
}

define_bridge_message! {
  #[bridge(ObserverAction)]
  pub struct ObserverAction {
    action: ObserverActionVariant ref = proto::action,
  }
}

define_bridge_oneof! {
  #[bridge(ObserverActionVariant)]
  pub enum ObserverActionVariant {
    None,
    PlayerPerspective(ActionObserverPlayerPerspective) = proto::PlayerPerspective,
    CameraMove(ActionObserverCameraMove) = proto::CameraMove,
    CameraFollowPlayer(ActionObserverCameraFollowPlayer) = proto::CameraFollowPlayer,
    CameraFollowUnits(ActionObserverCameraFollowUnits) = proto::CameraFollowUnits,
  }
}

define_bridge_message! {
  #[bridge(ActionObserverPlayerPerspective)]
  pub struct ActionObserverPlayerPerspective {
    player_id: PlayerTag = proto::player_id,
  }
}

define_bridge_message! {
  #[bridge(ActionObserverCameraMove)]
  pub struct ActionObserverCameraMove {
    world_pos: PointF = proto::world_pos,
    distance: Float = proto::distance,
  }
}

define_bridge_message! {
  #[bridge(ActionObserverCameraFollowPlayer)]
  pub struct ActionObserverCameraFollowPlayer {
    player_id: PlayerTag = proto::player_id,
  }
}

define_bridge_message! {
  #[bridge(ActionObserverCameraFollowUnits)]
  pub struct ActionObserverCameraFollowUnits {
    unit_tags: UnitTag vec = proto::unit_tags,
  }
}

define_bridge_enum! {
  #[bridge(Alert, NULL = AlertError)]
  pub enum Alert {
    AlertError,
    AddOnComplete,
    BuildingComplete,
    BuildingUnderAttack,
    LarvaHatched,
    MergeComplete,
    MineralsExhausted,
    MorphComplete,
    MothershipComplete,
    MULEExpired,
    NuclearLaunchDetected,
    NukeComplete,
    NydusWormDetected,
    ResearchComplete,
    TrainError,
    TrainUnitComplete,
    TrainWorkerComplete,
    TransformationComplete,
    UnitUnderAttack,
    UpgradeComplete,
    VespeneExhausted,
    WarpInComplete,
  }
}

define_bridge_enum! {
  #[bridge(GameResult, NULL = Undecided)]
  pub enum GameResult {
    Victory,
    Defeat,
    Tie,
    Undecided,
  }
}

define_bridge_message! {
  #[bridge(PlayerResult)]
  pub struct PlayerResult {
    player: PlayerTag = proto::player_id,
    result: GameResult = proto::result,
  }
}
