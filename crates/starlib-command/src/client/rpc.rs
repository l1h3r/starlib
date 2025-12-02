use std::fmt::Debug;
use std::fmt::DebugStruct;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::net::IpAddr;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::TryLockError;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use starlib_network::NetworkError;
use starlib_network::WebSocket;
use starlib_proto::Action;
use starlib_proto::ActionResult;
use starlib_proto::CommandError as ProtoCommandError;
use starlib_proto::CommandRecv;
use starlib_proto::CommandSend;
use starlib_proto::DebugCommand;
use starlib_proto::InterfaceOptions;
use starlib_proto::Map;
use starlib_proto::ObserverAction;
use starlib_proto::Participation;
use starlib_proto::PlayerSetup;
use starlib_proto::PortSet;
use starlib_proto::Replay2;
use starlib_proto::Replay5;
use starlib_proto::Request;
use starlib_proto::RequestQueryAvailableAbilities;
use starlib_proto::RequestQueryBuildingPlacement;
use starlib_proto::RequestQueryPathing;
use starlib_proto::RequestVariant;
use starlib_proto::Response;
use starlib_proto::ResponseAvailableMaps;
use starlib_proto::ResponseData;
use starlib_proto::ResponseGameInfo;
use starlib_proto::ResponseObservation;
use starlib_proto::ResponsePing;
use starlib_proto::ResponseQuery;
use starlib_proto::ResponseReplayInfo;
use starlib_proto::ResponseVariant;
use starlib_proto::Status;
use starlib_types::Bytes;
use starlib_types::PlayerTag;

use crate::client::cmd;
use crate::client::cmd::Command as _;
use crate::error::CommandError;
use crate::error::CommandErrorKind;

static ERR_MISSING_VARIANT: &str = "invalid response (missing variant)";
static ERR_INVALID_VARIANT: &str = "invalid response (invalid variant)";

static ERR_NULL_STATUS: &str = "invalid response status (NULL)";

// -----------------------------------------------------------------------------
// RPC Interface
// -----------------------------------------------------------------------------

pub struct Sc2Rpc {
  req_id: AtomicU32,
  socket: Mutex<WebSocket>,
  status: AtomicI32,
}

impl Sc2Rpc {
  #[inline]
  pub fn connect<T>(host: T, port: u16) -> Result<Self, NetworkError>
  where
    T: Into<IpAddr>,
  {
    WebSocket::builder()
      .host(host)
      .port(port)
      .connect()
      .map(Self::new)
  }

  #[inline]
  pub fn new(socket: WebSocket) -> Self {
    Self {
      req_id: AtomicU32::new(0),
      socket: Mutex::new(socket),
      status: AtomicI32::new(Status::Launched as i32),
    }
  }

  #[inline]
  pub fn status(&self) -> Status {
    Status::try_from(self.status.load(Ordering::Relaxed)).expect("status")
  }

  pub fn create_game<I>(
    &self,
    map: Map,
    player_setup: I,
    disable_fog: bool,
    random_seed: u32,
    realtime: bool,
  ) -> Result<(), CommandError>
  where
    I: IntoIterator<Item = PlayerSetup>,
  {
    cmd::CreateGame::run(
      self,
      (
        map,
        Vec::from_iter(player_setup),
        disable_fog,
        random_seed,
        realtime,
      ),
    )
  }

  pub fn join_game<I, T, U>(
    &self,
    participation: Participation,
    options: InterfaceOptions,
    server_ports: Option<PortSet>,
    client_ports: I,
    player_name: T,
    host_ip: U,
  ) -> Result<PlayerTag, CommandError>
  where
    I: IntoIterator<Item = PortSet>,
    T: Into<String>,
    U: Into<String>,
  {
    cmd::JoinGame::run(
      self,
      (
        participation,
        options,
        server_ports,
        Vec::from_iter(client_ports),
        player_name.into(),
        host_ip.into(),
      ),
    )
  }

  pub fn restart_game(&self) -> Result<bool, CommandError> {
    cmd::RestartGame::run(self, ())
  }

  pub fn start_replay<T>(
    &self,
    replay: Replay5,
    map_data: T,
    observed_player_id: PlayerTag,
    options: InterfaceOptions,
    disable_fog: bool,
    realtime: bool,
    record_replay: bool,
  ) -> Result<(), CommandError>
  where
    T: Into<Bytes>,
  {
    cmd::StartReplay::run(
      self,
      (
        replay,
        map_data.into(),
        observed_player_id,
        options,
        disable_fog,
        realtime,
        record_replay,
      ),
    )
  }

  pub fn leave_game(&self) -> Result<(), CommandError> {
    cmd::LeaveGame::run(self, ())
  }

  pub fn quick_save(&self) -> Result<(), CommandError> {
    cmd::QuickSave::run(self, ())
  }

  pub fn quick_load(&self) -> Result<(), CommandError> {
    cmd::QuickLoad::run(self, ())
  }

  pub fn quit(&self) -> Result<(), CommandError> {
    cmd::Quit::run(self, ())
  }

  pub fn game_info(&self) -> Result<ResponseGameInfo, CommandError> {
    cmd::GameInfo::run(self, ())
  }

  pub fn observation(
    &self,
    disable_fog: bool,
    game_loop: u32,
  ) -> Result<ResponseObservation, CommandError> {
    cmd::Observation::run(self, (disable_fog, game_loop))
  }

  pub fn step(&self, count: u32) -> Result<u32, CommandError> {
    cmd::Step::run(self, count)
  }

  pub fn action<I>(&self, actions: I) -> Result<Vec<ActionResult>, CommandError>
  where
    I: IntoIterator<Item = Action>,
  {
    cmd::Action::run(self, Vec::from_iter(actions))
  }

  pub fn observer_action<I>(&self, actions: I) -> Result<(), CommandError>
  where
    I: IntoIterator<Item = ObserverAction>,
  {
    cmd::ObserverAction::run(self, Vec::from_iter(actions))
  }

  pub fn data(
    &self,
    abilities: bool,
    units: bool,
    upgrades: bool,
    buffs: bool,
    effects: bool,
  ) -> Result<ResponseData, CommandError> {
    cmd::Data::run(self, (abilities, units, upgrades, buffs, effects))
  }

  pub fn query<I, J, K>(
    &self,
    pathing: I,
    abilities: J,
    placements: K,
    ignore_requirements: bool,
  ) -> Result<ResponseQuery, CommandError>
  where
    I: IntoIterator<Item = RequestQueryPathing>,
    J: IntoIterator<Item = RequestQueryAvailableAbilities>,
    K: IntoIterator<Item = RequestQueryBuildingPlacement>,
  {
    cmd::Query::run(
      self,
      (
        Vec::from_iter(pathing),
        Vec::from_iter(abilities),
        Vec::from_iter(placements),
        ignore_requirements,
      ),
    )
  }

  pub fn save_replay(&self) -> Result<Bytes, CommandError> {
    cmd::SaveReplay::run(self, ())
  }

  pub fn trigger<T>(&self, command: T) -> Result<(), CommandError>
  where
    T: Into<String>,
  {
    cmd::MapCommand::run(self, command.into())
  }

  pub fn replay_info(
    &self,
    replay: Replay2,
    download_data: bool,
  ) -> Result<ResponseReplayInfo, CommandError> {
    cmd::ReplayInfo::run(self, (replay, download_data))
  }

  pub fn maps(&self) -> Result<ResponseAvailableMaps, CommandError> {
    cmd::AvailableMaps::run(self, ())
  }

  pub fn save_map<T, U>(&self, path: T, data: U) -> Result<(), CommandError>
  where
    T: Into<String>,
    U: Into<Bytes>,
  {
    cmd::SaveMap::run(self, (path.into(), data.into()))
  }

  pub fn ping(&self) -> Result<ResponsePing, CommandError> {
    cmd::Ping::run(self, ())
  }

  pub fn debug<I>(&self, debug: I) -> Result<(), CommandError>
  where
    I: IntoIterator<Item = DebugCommand>,
  {
    cmd::Debug::run(self, Vec::from_iter(debug))
  }

  pub fn try_command<T>(&self, request: T) -> Result<T::Recv, CommandError>
  where
    T: CommandSend,
    T::Recv: ProtoCommandError,
  {
    let response: T::Recv = self.command(request)?;

    if let Some(error) = ProtoCommandError::to_error(&response) {
      Err(CommandError::msg(error))
    } else {
      Ok(response)
    }
  }

  pub fn command<T>(&self, request: T) -> Result<T::Recv, CommandError>
  where
    T: CommandSend,
  {
    let mut response: Response = Default::default();

    let request: RequestVariant = request.into_variant();
    let _unused: &ResponseVariant = self.__request(request, &mut response)?;

    let Some(variant): Option<ResponseVariant> = response.response else {
      return Err(CommandError::msg(ERR_MISSING_VARIANT));
    };

    let Some(variant): Option<T::Recv> = CommandRecv::from_variant(variant) else {
      return Err(CommandError::msg(ERR_INVALID_VARIANT));
    };

    Ok(variant)
  }

  pub fn request<'a, T>(
    &self,
    request: T,
    response: &'a mut Response,
  ) -> Result<&'a ResponseVariant, CommandError>
  where
    T: CommandSend,
  {
    self.__request(request.into_variant(), response)
  }

  fn __request<'a>(
    &self,
    request: RequestVariant,
    response: &'a mut Response,
  ) -> Result<&'a ResponseVariant, CommandError> {
    let request_id: u32 = self.try_send(request)?;

    self.try_with(|socket| socket.read_into(response))?;

    if response.id != request_id {
      return Err(CommandError::msg(format_args!(
        "invalid response id (expected {}, received {})",
        request_id, response.id,
      )));
    }

    if matches!(response.status(), Status::NULL) {
      return Err(CommandError::msg(ERR_NULL_STATUS));
    }

    if !response.error.is_empty() {
      return Err(CommandError::msg(response.error.join("\n")));
    }

    if let Some(ref variant) = response.response {
      self.status.store(response.status, Ordering::SeqCst);
      Ok(variant)
    } else {
      Err(CommandError::msg(ERR_MISSING_VARIANT))
    }
  }

  fn try_send(&self, variant: RequestVariant) -> Result<u32, CommandError> {
    let request_id: u32 = self.req_id.fetch_add(1, Ordering::Relaxed);

    let request: Request = Request {
      request: Some(variant),
      id: request_id,
    };

    self.try_with(|socket| socket.send(request))?;

    Ok(request_id)
  }

  fn try_with<F, T, E>(&self, f: F) -> Result<T, CommandError>
  where
    F: FnOnce(&mut WebSocket) -> Result<T, E>,
    E: Into<CommandError>,
  {
    const ERR: TryLockError<()> = TryLockError::<()>::WouldBlock;

    let __do = |mut guard: MutexGuard<'_, WebSocket>| f(&mut *guard).map_err(Into::into);

    match self.socket.try_lock() {
      Ok(guard) => __do(guard),
      Err(TryLockError::Poisoned(error)) => __do(error.into_inner()),
      Err(TryLockError::WouldBlock) => Err(CommandError::new(ERR, CommandErrorKind::Blocked)),
    }
  }
}

impl Debug for Sc2Rpc {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let mut debug: DebugStruct<'_, '_> = f.debug_struct("Sc2Rpc");

    debug.field("req_id", &self.req_id);
    debug.field("status", &self.status());

    match self.socket.try_lock() {
      Ok(guard) => {
        debug.field("socket", &&*guard);
      }
      Err(TryLockError::Poisoned(error)) => {
        debug.field("socket", &&**error.get_ref());
      }
      Err(TryLockError::WouldBlock) => {
        debug.field("socket", &"<locked>");
      }
    }

    debug.finish_non_exhaustive()
  }
}

impl From<WebSocket> for Sc2Rpc {
  #[inline]
  fn from(other: WebSocket) -> Self {
    Self::new(other)
  }
}
