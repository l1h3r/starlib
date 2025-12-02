use prost::Enumeration;
use prost::Message;
use prost::Oneof;
use prost::bytes::Bytes;

use crate::support::AbilityData;
use crate::support::Action;
use crate::support::ActionError;
use crate::support::ActionResult;
use crate::support::BuffData;
use crate::support::ChatReceived;
use crate::support::DebugCommand;
use crate::support::EffectData;
use crate::support::InterfaceOptions;
use crate::support::Map;
use crate::support::Observation;
use crate::support::ObserverAction;
use crate::support::Participation;
use crate::support::PlayerInfo;
use crate::support::PlayerInfoExtra;
use crate::support::PlayerResult;
use crate::support::PlayerSetup;
use crate::support::PortSet;
use crate::support::Replay2;
use crate::support::Replay5;
use crate::support::RequestQueryAvailableAbilities;
use crate::support::RequestQueryBuildingPlacement;
use crate::support::RequestQueryPathing;
use crate::support::ResponseQueryAvailableAbilities;
use crate::support::ResponseQueryBuildingPlacement;
use crate::support::ResponseQueryPathing;
use crate::support::StartRaw;
use crate::support::UnitTypeData;
use crate::support::UpgradeData;

// -----------------------------------------------------------------------------
// Request
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct Request {
  #[prost(
    oneof = "RequestVariant",
    tags = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22"
  )]
  pub request: Option<RequestVariant>,
  #[prost(uint32, tag = "97")]
  pub id: u32,
}

#[derive(Clone, Oneof)]
pub enum RequestVariant {
  #[prost(message, tag = "1")]
  CreateGame(RequestCreateGame),
  #[prost(message, tag = "2")]
  JoinGame(RequestJoinGame),
  #[prost(message, tag = "3")]
  RestartGame(RequestRestartGame),
  #[prost(message, tag = "4")]
  StartReplay(RequestStartReplay),
  #[prost(message, tag = "5")]
  LeaveGame(RequestLeaveGame),

  #[prost(message, tag = "6")]
  QuickSave(RequestQuickSave),
  #[prost(message, tag = "7")]
  QuickLoad(RequestQuickLoad),

  #[prost(message, tag = "8")]
  Quit(RequestQuit),

  #[prost(message, tag = "9")]
  GameInfo(RequestGameInfo),
  #[prost(message, tag = "10")]
  Observation(RequestObservation),
  #[prost(message, tag = "11")]
  Action(RequestAction),
  #[prost(message, tag = "21")]
  ObserverAction(RequestObserverAction),
  #[prost(message, tag = "12")]
  Step(RequestStep),
  #[prost(message, tag = "13")]
  Data(RequestData),
  #[prost(message, tag = "14")]
  Query(RequestQuery),
  #[prost(message, tag = "15")]
  SaveReplay(RequestSaveReplay),
  #[prost(message, tag = "22")]
  MapCommand(RequestMapCommand),

  #[prost(message, tag = "16")]
  ReplayInfo(RequestReplayInfo),
  #[prost(message, tag = "17")]
  AvailableMaps(RequestAvailableMaps),
  #[prost(message, tag = "18")]
  SaveMap(RequestSaveMap),

  #[prost(message, tag = "19")]
  Ping(RequestPing),
  #[prost(message, tag = "20")]
  Debug(RequestDebug),
}

// -----------------------------------------------------------------------------
// Response
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct Response {
  #[prost(
    oneof = "ResponseVariant",
    tags = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22"
  )]
  pub response: Option<ResponseVariant>,
  #[prost(uint32, tag = "97")]
  pub id: u32,
  #[prost(repeated, string, tag = "98")]
  pub error: Vec<String>,
  #[prost(enumeration = "Status", tag = "99")]
  pub status: i32,
}

#[derive(Clone, Oneof)]
pub enum ResponseVariant {
  #[prost(message, tag = "1")]
  CreateGame(ResponseCreateGame),
  #[prost(message, tag = "2")]
  JoinGame(ResponseJoinGame),
  #[prost(message, tag = "3")]
  RestartGame(ResponseRestartGame),
  #[prost(message, tag = "4")]
  StartReplay(ResponseStartReplay),
  #[prost(message, tag = "5")]
  LeaveGame(ResponseLeaveGame),

  #[prost(message, tag = "6")]
  QuickSave(ResponseQuickSave),
  #[prost(message, tag = "7")]
  QuickLoad(ResponseQuickLoad),

  #[prost(message, tag = "8")]
  Quit(ResponseQuit),

  #[prost(message, tag = "9")]
  GameInfo(ResponseGameInfo),
  #[prost(message, tag = "10")]
  Observation(ResponseObservation),
  #[prost(message, tag = "11")]
  Action(ResponseAction),
  #[prost(message, tag = "21")]
  ObserverAction(ResponseObserverAction),
  #[prost(message, tag = "12")]
  Step(ResponseStep),
  #[prost(message, tag = "13")]
  Data(ResponseData),
  #[prost(message, tag = "14")]
  Query(ResponseQuery),
  #[prost(message, tag = "15")]
  SaveReplay(ResponseSaveReplay),
  #[prost(message, tag = "22")]
  MapCommand(ResponseMapCommand),

  #[prost(message, tag = "16")]
  ReplayInfo(ResponseReplayInfo),
  #[prost(message, tag = "17")]
  AvailableMaps(ResponseAvailableMaps),
  #[prost(message, tag = "18")]
  SaveMap(ResponseSaveMap),

  #[prost(message, tag = "19")]
  Ping(ResponsePing),
  #[prost(message, tag = "20")]
  Debug(ResponseDebug),
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum Status {
  NULL = 0,
  Launched = 1,
  InitGame = 2,
  InGame = 3,
  InReplay = 4,
  Ended = 5,
  Quit = 6,
  Unknown = 99,
}

// -----------------------------------------------------------------------------
// @command - Create Game
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestCreateGame {
  #[prost(oneof = "Map", tags = "1,2")]
  pub map: Option<Map>,
  #[prost(repeated, message, tag = "3")]
  pub player_setup: Vec<PlayerSetup>,
  #[prost(bool, tag = "4")]
  pub disable_fog: bool,
  #[prost(uint32, tag = "5")]
  pub random_seed: u32,
  #[prost(bool, tag = "6")]
  pub realtime: bool,
}

#[derive(Clone, Message)]
pub struct ResponseCreateGame {
  #[prost(enumeration = "ResponseCreateGameError", tag = "1")]
  pub error: i32,
  #[prost(string, tag = "2")]
  pub error_details: String,
}

#[derive(Clone, Debug, Enumeration)]
#[repr(i32)]
pub enum ResponseCreateGameError {
  NULL = 0,
  MissingMap = 1,
  InvalidMapPath = 2,
  InvalidMapData = 3,
  InvalidMapName = 4,
  InvalidMapHandle = 5,
  MissingPlayerSetup = 6,
  InvalidPlayerSetup = 7,
  MultiplayerUnsupported = 8,
}

// -----------------------------------------------------------------------------
// @command - Join Game
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestJoinGame {
  #[prost(oneof = "Participation", tags = "1,2")]
  pub participation: Option<Participation>,
  #[prost(message, tag = "3")]
  pub options: Option<InterfaceOptions>,
  #[prost(message, tag = "4")]
  pub server_ports: Option<PortSet>,
  #[prost(repeated, message, tag = "5")]
  pub client_ports: Vec<PortSet>,
  #[deprecated(since = "TODO")]
  #[prost(int32, tag = "6")]
  pub shared_port: i32,
  #[prost(string, tag = "7")]
  pub player_name: String,
  #[prost(string, tag = "8")]
  pub host_ip: String,
}

#[derive(Clone, Message)]
pub struct ResponseJoinGame {
  #[prost(uint32, tag = "1")]
  pub player_id: u32,
  #[prost(enumeration = "ResponseJoinGameError", tag = "2")]
  pub error: i32,
  #[prost(string, tag = "3")]
  pub error_details: String,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ResponseJoinGameError {
  NULL = 0,
  MissingParticipation = 1,
  InvalidObservedPlayerId = 2,
  MissingOptions = 3,
  MissingPorts = 4,
  GameFull = 5,
  LaunchError = 6,
  FeatureUnsupported = 7,
  NoSpaceForUser = 8,
  MapDoesNotExist = 9,
  CannotOpenMap = 10,
  ChecksumError = 11,
  NetworkError = 12,
  OtherError = 13,
}

// -----------------------------------------------------------------------------
// @command - Restart Game
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestRestartGame {}

#[derive(Clone, Message)]
pub struct ResponseRestartGame {
  #[prost(enumeration = "ResponseRestartGameError", tag = "1")]
  pub error: i32,
  #[prost(string, tag = "2")]
  pub error_details: String,
  #[prost(bool, tag = "3")]
  pub need_hard_reset: bool,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ResponseRestartGameError {
  NULL = 0,
  LaunchError = 1,
}

// -----------------------------------------------------------------------------
// @command - Start Replay
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestStartReplay {
  #[prost(oneof = "Replay5", tags = "1,5")]
  pub replay: Option<Replay5>,
  #[prost(bytes, tag = "6")]
  pub map_data: Bytes,
  #[prost(int32, tag = "2")]
  pub observed_player_id: i32,
  #[prost(message, tag = "3")]
  pub options: Option<InterfaceOptions>,
  #[prost(bool, tag = "4")]
  pub disable_fog: bool,
  #[prost(bool, tag = "7")]
  pub realtime: bool,
  #[prost(bool, tag = "8")]
  pub record_replay: bool,
}

#[derive(Clone, Message)]
pub struct ResponseStartReplay {
  #[prost(enumeration = "ResponseStartReplayError", tag = "1")]
  pub error: i32,
  #[prost(string, tag = "2")]
  pub error_details: String,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ResponseStartReplayError {
  NULL = 0,
  MissingReplay = 1,
  InvalidReplayPath = 2,
  InvalidReplayData = 3,
  InvalidMapData = 4,
  InvalidObservedPlayerId = 5,
  MissingOptions = 6,
  LaunchError = 7,
}

// -----------------------------------------------------------------------------
// @command - Leave Game
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestLeaveGame {}

#[derive(Clone, Copy, Message)]
pub struct ResponseLeaveGame {}

// -----------------------------------------------------------------------------
// @command - Quick Save
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestQuickSave {}

#[derive(Clone, Copy, Message)]
pub struct ResponseQuickSave {}

// -----------------------------------------------------------------------------
// @command - Quick Load
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestQuickLoad {}

#[derive(Clone, Copy, Message)]
pub struct ResponseQuickLoad {}

// -----------------------------------------------------------------------------
// @command - Quit
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestQuit {}

#[derive(Clone, Copy, Message)]
pub struct ResponseQuit {}

// -----------------------------------------------------------------------------
// @command - Game Info
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestGameInfo {}

#[derive(Clone, Message)]
pub struct ResponseGameInfo {
  #[prost(string, tag = "1")]
  pub map_name: String,
  #[prost(repeated, string, tag = "6")]
  pub mod_names: Vec<String>,
  #[prost(string, tag = "2")]
  pub local_map_path: String,
  #[prost(repeated, message, tag = "3")]
  pub player_info: Vec<PlayerInfo>,
  #[prost(message, tag = "4")]
  pub start_raw: Option<StartRaw>,
  #[prost(message, tag = "5")]
  pub options: Option<InterfaceOptions>,
}

// -----------------------------------------------------------------------------
// @command - Observation
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestObservation {
  #[prost(bool, tag = "1")]
  pub disable_fog: bool,
  #[prost(uint32, tag = "2")]
  pub game_loop: u32,
}

#[derive(Clone, Message)]
pub struct ResponseObservation {
  #[prost(repeated, message, tag = "1")]
  pub actions: Vec<Action>,
  #[prost(repeated, message, tag = "2")]
  pub action_errors: Vec<ActionError>,
  #[prost(message, tag = "3")]
  pub observation: Option<Observation>,
  #[prost(repeated, message, tag = "4")]
  pub player_result: Vec<PlayerResult>,
  #[prost(repeated, message, tag = "5")]
  pub chat: Vec<ChatReceived>,
}

// -----------------------------------------------------------------------------
// @command - Action
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestAction {
  #[prost(repeated, message, tag = "1")]
  pub actions: Vec<Action>,
}

#[derive(Clone, Message)]
pub struct ResponseAction {
  #[prost(repeated, enumeration = "ActionResult", tag = "1")]
  pub results: Vec<i32>,
}

// -----------------------------------------------------------------------------
// @command - Observer Action
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestObserverAction {
  #[prost(repeated, message, tag = "1")]
  pub actions: Vec<ObserverAction>,
}

#[derive(Clone, Copy, Message)]
pub struct ResponseObserverAction {}

// -----------------------------------------------------------------------------
// @command - Step
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestStep {
  #[prost(uint32, tag = "1")]
  pub count: u32,
}

#[derive(Clone, Copy, Message)]
pub struct ResponseStep {
  #[prost(uint32, tag = "1")]
  pub simulation_loop: u32,
}

// -----------------------------------------------------------------------------
// @command - Data
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestData {
  #[prost(bool, tag = "1")]
  pub ability_id: bool,
  #[prost(bool, tag = "2")]
  pub unit_type_id: bool,
  #[prost(bool, tag = "3")]
  pub upgrade_id: bool,
  #[prost(bool, tag = "4")]
  pub buff_id: bool,
  #[prost(bool, tag = "5")]
  pub effect_id: bool,
}

#[derive(Clone, Message)]
pub struct ResponseData {
  #[prost(repeated, message, tag = "1")]
  pub abilities: Vec<AbilityData>,
  #[prost(repeated, message, tag = "2")]
  pub units: Vec<UnitTypeData>,
  #[prost(repeated, message, tag = "3")]
  pub upgrades: Vec<UpgradeData>,
  #[prost(repeated, message, tag = "4")]
  pub buffs: Vec<BuffData>,
  #[prost(repeated, message, tag = "5")]
  pub effects: Vec<EffectData>,
}

// -----------------------------------------------------------------------------
// @command - Query
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestQuery {
  #[prost(repeated, message, tag = "1")]
  pub pathing: Vec<RequestQueryPathing>,
  #[prost(repeated, message, tag = "2")]
  pub abilities: Vec<RequestQueryAvailableAbilities>,
  #[prost(repeated, message, tag = "3")]
  pub placements: Vec<RequestQueryBuildingPlacement>,
  #[prost(bool, tag = "4")]
  pub ignore_resource_requirements: bool,
}

#[derive(Clone, Message)]
pub struct ResponseQuery {
  #[prost(repeated, message, tag = "1")]
  pub pathing: Vec<ResponseQueryPathing>,
  #[prost(repeated, message, tag = "2")]
  pub abilities: Vec<ResponseQueryAvailableAbilities>,
  #[prost(repeated, message, tag = "3")]
  pub placements: Vec<ResponseQueryBuildingPlacement>,
}

// -----------------------------------------------------------------------------
// @command - Save Replay
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestSaveReplay {}

#[derive(Clone, Message)]
pub struct ResponseSaveReplay {
  #[prost(bytes, tag = "1")]
  pub data: Bytes,
}

// -----------------------------------------------------------------------------
// @command - Map Command
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestMapCommand {
  #[prost(string, tag = "1")]
  pub command: String,
}

#[derive(Clone, Message)]
pub struct ResponseMapCommand {
  #[prost(enumeration = "ResponseMapCommandError", tag = "1")]
  pub error: i32,
  #[prost(string, tag = "2")]
  pub error_details: String,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ResponseMapCommandError {
  NULL = 0,
  NoTriggerError = 1,
}

// -----------------------------------------------------------------------------
// @command - Replay Info
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestReplayInfo {
  #[prost(oneof = "Replay2", tags = "1,2")]
  pub replay: Option<Replay2>,
  #[prost(bool, tag = "3")]
  pub download_data: bool,
}

#[derive(Clone, Message)]
pub struct ResponseReplayInfo {
  #[prost(string, tag = "1")]
  pub map_name: String,
  #[prost(string, tag = "2")]
  pub local_map_path: String,
  #[prost(repeated, message, tag = "3")]
  pub player_info: Vec<PlayerInfoExtra>,
  #[prost(uint32, tag = "4")]
  pub game_duration_loops: u32,
  #[prost(float, tag = "5")]
  pub game_duration_seconds: f32,
  #[prost(string, tag = "6")]
  pub game_version: String,
  #[prost(string, tag = "11")]
  pub data_version: String,
  #[prost(uint32, tag = "7")]
  pub data_build: u32,
  #[prost(uint32, tag = "8")]
  pub base_build: u32,
  #[prost(enumeration = "ResponseReplayInfoError", tag = "9")]
  pub error: i32,
  #[prost(string, tag = "10")]
  pub error_details: String,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ResponseReplayInfoError {
  NULL = 0,
  MissingReplay = 1,
  InvalidReplayPath = 2,
  InvalidReplayData = 3,
  ParsingError = 4,
  DownloadError = 5,
}

// -----------------------------------------------------------------------------
// @command - Available Maps
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestAvailableMaps {}

#[derive(Clone, Message)]
pub struct ResponseAvailableMaps {
  #[prost(repeated, string, tag = "1")]
  pub local_map_paths: Vec<String>,
  #[prost(repeated, string, tag = "2")]
  pub battlenet_map_names: Vec<String>,
}

// -----------------------------------------------------------------------------
// @command - Save Map
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestSaveMap {
  #[prost(string, tag = "1")]
  pub map_path: String,
  #[prost(bytes, tag = "2")]
  pub map_data: Bytes,
}

#[derive(Clone, Copy, Message)]
pub struct ResponseSaveMap {
  #[prost(enumeration = "ResponseSaveMapError", tag = "1")]
  pub error: i32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ResponseSaveMapError {
  NULL = 0,
  InvalidMapData = 1,
}

// -----------------------------------------------------------------------------
// @command - Ping
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct RequestPing {}

#[derive(Clone, Message)]
pub struct ResponsePing {
  #[prost(string, tag = "1")]
  pub game_version: String,
  #[prost(string, tag = "2")]
  pub data_version: String,
  #[prost(uint32, tag = "3")]
  pub data_build: u32,
  #[prost(uint32, tag = "4")]
  pub base_build: u32,
}

// -----------------------------------------------------------------------------
// @command - Debug
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct RequestDebug {
  #[prost(repeated, message, tag = "1")]
  pub debug: Vec<DebugCommand>,
}

#[derive(Clone, Copy, Message)]
pub struct ResponseDebug {}
