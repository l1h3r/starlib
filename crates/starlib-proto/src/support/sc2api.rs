use prost::DecodeError;
use prost::Enumeration;
use prost::Message;
use prost::Oneof;
use prost::bytes::Buf;
use prost::bytes::BufMut;
use prost::bytes::Bytes;
use prost::encoding;

use crate::support::ActionRaw;
use crate::support::ActionResult;
use crate::support::ActionSpatial;
use crate::support::ActionUI;
use crate::support::AvailableAbility;
use crate::support::ObservationFeatureLayer;
use crate::support::ObservationRaw;
use crate::support::ObservationRender;
use crate::support::ObservationUI;
use crate::support::Point2D;
use crate::support::Race;
use crate::support::Score;
use crate::support::Size2DI;

#[derive(Clone, Oneof)]
pub enum Map {
  #[prost(message, tag = "1")]
  LocalMap(LocalMap),
  #[prost(string, tag = "2")]
  BattleNetMapName(String),
}

#[derive(Clone, Message)]
pub struct LocalMap {
  #[prost(string, tag = "1")]
  pub map_path: String,
  #[prost(bytes, tag = "7")]
  pub map_data: Bytes,
}

#[derive(Clone, Copy, Oneof)]
pub enum Participation {
  #[prost(enumeration = "Race", tag = "1")]
  Race(i32),
  #[prost(uint32, tag = "2")]
  ObservedPlayerId(u32),
}

#[derive(Clone, Copy, Message)]
pub struct PortSet {
  #[prost(int32, tag = "1")]
  pub game_port: i32,
  #[prost(int32, tag = "2")]
  pub base_port: i32,
}

#[derive(Clone, Message)]
pub struct ChatReceived {
  #[prost(uint32, tag = "1")]
  pub player_id: u32,
  #[prost(string, tag = "2")]
  pub message: String,
}

#[derive(Clone, Message)]
pub struct PlayerInfoExtra {
  #[prost(message, tag = "1")]
  pub player_info: Option<PlayerInfo>,
  #[prost(message, tag = "2")]
  pub player_result: Option<PlayerResult>,
  #[prost(int32, tag = "3")]
  pub player_mmr: i32,
  #[prost(int32, tag = "4")]
  pub player_apm: i32,
}

#[derive(Clone, Debug)]
pub enum Replay<const TAG: u32> {
  Path(String),
  Data(Bytes),
}

impl<const TAG: u32> Replay<TAG> {
  #[inline]
  pub(crate) fn encode(&self, buffer: &mut impl BufMut) {
    match self {
      Self::Path(value) => encoding::string::encode(1, value, buffer),
      Self::Data(value) => encoding::bytes::encode(TAG, value, buffer),
    }
  }

  pub(crate) fn merge(
    field: &mut Option<Self>,
    tag: u32,
    wire_type: encoding::WireType,
    buffer: &mut impl Buf,
    context: encoding::DecodeContext,
  ) -> Result<(), DecodeError> {
    if tag == 1 {
      if let Some(Self::Path(value)) = field {
        return encoding::string::merge(wire_type, value, buffer, context);
      } else {
        todo!("Replay::merge(1)")
      }
    }

    if tag == TAG {
      if let Some(Self::Data(value)) = field {
        return encoding::bytes::merge(wire_type, value, buffer, context);
      } else {
        todo!("Replay::merge({TAG})")
      }
    }

    unreachable!("invalid Replay tag: {tag}")
  }

  #[inline]
  pub(crate) fn encoded_len(&self) -> usize {
    match self {
      Self::Path(value) => encoding::string::encoded_len(1, value),
      Self::Data(value) => encoding::bytes::encoded_len(TAG, value),
    }
  }
}

// -----------------------------------------------------------------------------
// Game Setup
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum Difficulty {
  NULL = 0,
  VeryEasy = 1,
  Easy = 2,
  Medium = 3,
  MediumHard = 4,
  Hard = 5,
  Harder = 6,
  VeryHard = 7,
  CheatVision = 8,
  CheatMoney = 9,
  CheatInsane = 10,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum PlayerType {
  NULL = 0,
  Participant = 1,
  Computer = 2,
  Observer = 3,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum AIBuild {
  NULL = 0,
  RandomBuild = 1,
  Rush = 2,
  Timing = 3,
  Power = 4,
  Macro = 5,
  Air = 6,
}

#[derive(Clone, Message)]
pub struct PlayerSetup {
  #[prost(enumeration = "PlayerType", tag = "1")]
  pub kind: i32,
  #[prost(enumeration = "Race", tag = "2")]
  pub race: i32,
  #[prost(enumeration = "Difficulty", tag = "3")]
  pub difficulty: i32,
  #[prost(string, tag = "4")]
  pub player_name: String,
  #[prost(enumeration = "AIBuild", tag = "5")]
  pub ai_build: i32,
}

#[derive(Clone, Copy, Message)]
pub struct SpatialCameraSetup {
  #[prost(message, tag = "2")]
  pub resolution: Option<Size2DI>,
  #[prost(message, tag = "3")]
  pub minimap_resolution: Option<Size2DI>,
  #[prost(float, tag = "1")]
  pub width: f32,
  #[prost(bool, tag = "4")]
  pub crop_to_playable_area: bool,
  #[prost(bool, tag = "5")]
  pub allow_cheating_layers: bool,
}

#[derive(Clone, Copy, Message)]
pub struct InterfaceOptions {
  #[prost(bool, tag = "1")]
  pub raw: bool,
  #[prost(bool, tag = "2")]
  pub score: bool,
  #[prost(message, tag = "3")]
  pub feature_layer: Option<SpatialCameraSetup>,
  #[prost(message, tag = "4")]
  pub render: Option<SpatialCameraSetup>,
  #[prost(bool, tag = "5")]
  pub show_cloaked: bool,
  #[prost(bool, tag = "9")]
  pub show_burrowed_shadows: bool,
  #[prost(bool, tag = "8")]
  pub show_placeholders: bool,
  #[prost(bool, tag = "6")]
  pub raw_affects_selection: bool,
  #[prost(bool, tag = "7")]
  pub raw_crop_to_playable_area: bool,
}

#[derive(Clone, Message)]
pub struct PlayerInfo {
  #[prost(uint32, tag = "1")]
  pub player_id: u32,
  #[prost(enumeration = "PlayerType", tag = "2")]
  pub kind: i32,
  #[prost(enumeration = "Race", tag = "3")]
  pub race_requested: i32,
  #[prost(enumeration = "Race", tag = "4")]
  pub race_actual: i32,
  #[prost(enumeration = "Difficulty", tag = "5")]
  pub difficulty: i32,
  #[prost(enumeration = "AIBuild", tag = "7")]
  pub ai_build: i32,
  #[prost(string, tag = "6")]
  pub player_name: String,
}

// -----------------------------------------------------------------------------
// During Game
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct PlayerCommon {
  #[prost(uint32, tag = 1)]
  pub player_id: u32,
  #[prost(uint32, tag = 2)]
  pub minerals: u32,
  #[prost(uint32, tag = 3)]
  pub vespene: u32,
  #[prost(uint32, tag = 4)]
  pub food_cap: u32,
  #[prost(uint32, tag = 5)]
  pub food_used: u32,
  #[prost(uint32, tag = 6)]
  pub food_army: u32,
  #[prost(uint32, tag = 7)]
  pub food_workers: u32,
  #[prost(uint32, tag = 8)]
  pub idle_worker_count: u32,
  #[prost(uint32, tag = 9)]
  pub army_count: u32,
  #[prost(uint32, tag = 10)]
  pub warp_gate_count: u32,
  #[prost(uint32, tag = 11)]
  pub larva_count: u32,
}

#[derive(Clone, Message)]
pub struct Observation {
  #[prost(uint32, tag = "9")]
  pub game_loop: u32,
  #[prost(message, tag = "1")]
  pub player_common: Option<PlayerCommon>,
  #[prost(repeated, enumeration = "Alert", tag = "10")]
  pub alerts: Vec<i32>,
  #[prost(repeated, message, tag = "3")]
  pub abilities: Vec<AvailableAbility>,
  #[prost(message, tag = "4")]
  pub score: Option<Score>,

  #[prost(message, tag = "5")]
  pub raw_data: Option<ObservationRaw>,
  #[prost(message, tag = "6")]
  pub feature_layer_data: Option<ObservationFeatureLayer>,
  #[prost(message, tag = "7")]
  pub render_data: Option<ObservationRender>,
  #[prost(message, tag = "8")]
  pub ui_data: Option<ObservationUI>,
}

#[derive(Clone, Message)]
pub struct Action {
  #[prost(message, tag = "1")]
  pub action_raw: Option<ActionRaw>,
  #[prost(message, tag = "2")]
  pub action_feature_layer: Option<ActionSpatial>,
  #[prost(message, tag = "3")]
  pub action_render: Option<ActionSpatial>,
  #[prost(message, tag = "4")]
  pub action_ui: Option<ActionUI>,
  #[prost(message, tag = "6")]
  pub action_chat: Option<ActionChat>,
  #[prost(uint32, tag = "7")]
  pub game_loop: u32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ActionChatChannel {
  NULL = 0,
  Broadcast = 1,
  Team = 2,
}

#[derive(Clone, Message)]
pub struct ActionChat {
  #[prost(enumeration = "ActionChatChannel", tag = "1")]
  pub channel: i32,
  #[prost(string, tag = "2")]
  pub message: String,
}

#[derive(Clone, Copy, Message)]
pub struct ActionError {
  #[prost(uint64, tag = "1")]
  pub unit_tag: u64,
  #[prost(uint64, tag = "2")]
  pub ability_id: u64,
  #[prost(enumeration = "ActionResult", tag = "3")]
  pub result: i32,
}

#[derive(Clone, Message)]
pub struct ObserverAction {
  #[prost(oneof = "ObserverActionVariant", tags = "1,2,3,4")]
  pub action: Option<ObserverActionVariant>,
}

#[derive(Clone, Oneof)]
pub enum ObserverActionVariant {
  #[prost(message, tag = "1")]
  PlayerPerspective(ActionObserverPlayerPerspective),
  #[prost(message, tag = "2")]
  CameraMove(ActionObserverCameraMove),
  #[prost(message, tag = "3")]
  CameraFollowPlayer(ActionObserverCameraFollowPlayer),
  #[prost(message, tag = "4")]
  CameraFollowUnits(ActionObserverCameraFollowUnits),
}

#[derive(Clone, Copy, Message)]
pub struct ActionObserverPlayerPerspective {
  #[prost(uint32, tag = "1")]
  pub player_id: u32,
}

#[derive(Clone, Copy, Message)]
pub struct ActionObserverCameraMove {
  #[prost(message, tag = "1")]
  pub world_pos: Option<Point2D>,
  #[prost(float, tag = "2")]
  pub distance: f32,
}

#[derive(Clone, Copy, Message)]
pub struct ActionObserverCameraFollowPlayer {
  #[prost(uint32, tag = "1")]
  pub player_id: u32,
}

#[derive(Clone, Message)]
pub struct ActionObserverCameraFollowUnits {
  #[prost(repeated, uint64, tag = "1")]
  pub unit_tags: Vec<u64>,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum Alert {
  NULL = 0,
  AlertError = 3,
  AddOnComplete = 4,
  BuildingComplete = 5,
  BuildingUnderAttack = 6,
  LarvaHatched = 7,
  MergeComplete = 8,
  MineralsExhausted = 9,
  MorphComplete = 10,
  MothershipComplete = 11,
  MULEExpired = 12,
  NuclearLaunchDetected = 1,
  NukeComplete = 13,
  NydusWormDetected = 2,
  ResearchComplete = 14,
  TrainError = 15,
  TrainUnitComplete = 16,
  TrainWorkerComplete = 17,
  TransformationComplete = 18,
  UnitUnderAttack = 19,
  UpgradeComplete = 20,
  VespeneExhausted = 21,
  WarpInComplete = 22,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum GameResult {
  NULL = 0,
  Victory = 1,
  Defeat = 2,
  Tie = 3,
  Undecided = 4,
}

#[derive(Clone, Copy, Message)]
pub struct PlayerResult {
  #[prost(uint32, tag = "1")]
  pub player_id: u32,
  #[prost(enumeration = "GameResult", tag = "2")]
  pub result: i32,
}
