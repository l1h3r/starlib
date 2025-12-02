use std::net::IpAddr;
use std::sync::Arc;

use starlib_network::NetworkError;
use starlib_network::WebSocket;
use starlib_proto::InterfaceOptions;
use starlib_proto::Participation;
use starlib_proto::PlayerSetup;
use starlib_proto::ResponsePing;
use starlib_types::Bytes;
use starlib_types::GameData;
use starlib_types::GameInfo;
use starlib_types::IntoProto;
use starlib_types::IntoSc2;
use starlib_types::PlayerTag;
use starlib_types::PlayerType;
use starlib_types::Race;

use crate::client::Sc2Rpc;
use crate::config::Config;
use crate::error::CommandError;
use crate::setup::Computer;
use crate::setup::Connection;
use crate::setup::Map;

// -----------------------------------------------------------------------------
// API Interface
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Sc2Api {
  cfg: Arc<Config>,
  rpc: Sc2Rpc,
}

impl Sc2Api {
  #[inline]
  pub fn connect<T>(host: T, port: u16) -> Result<Self, NetworkError>
  where
    T: Into<IpAddr>,
  {
    Sc2Rpc::connect(host, port).map(Into::into)
  }

  #[inline]
  pub fn new(rpc: Sc2Rpc) -> Self {
    Self::with_config(rpc, Default::default())
  }

  #[inline]
  pub fn with_config(rpc: Sc2Rpc, cfg: Config) -> Self {
    Self {
      rpc,
      cfg: Arc::new(cfg),
    }
  }

  #[inline]
  pub const fn cfg(&self) -> &Arc<Config> {
    &self.cfg
  }

  #[inline]
  pub const fn rpc(&self) -> &Sc2Rpc {
    &self.rpc
  }

  pub fn start_singleplayer<T>(
    &self,
    name: T,
    race: Race,
    map: Map,
    opponents: &[Computer],
  ) -> Result<PlayerTag, CommandError>
  where
    T: Into<String>,
  {
    fn to_players(opponents: &[Computer]) -> Vec<PlayerSetup> {
      let player: PlayerSetup = PlayerSetup {
        kind: PlayerType::Participant as i32,
        ..Default::default()
      };

      let mut data: Vec<PlayerSetup> = Vec::with_capacity(1 + opponents.len());
      data.push(player);
      data.extend(opponents.iter().map(IntoProto::into_proto));
      data
    }

    let _unused: () = self.rpc.create_game(
      map.into_proto(),
      to_players(opponents),
      self.cfg.disable_fog,
      self.cfg.random_seed,
      self.cfg.realtime,
    )?;

    self.join_singleplayer(name, race)
  }

  pub fn join_singleplayer<T>(&self, name: T, race: Race) -> Result<PlayerTag, CommandError>
  where
    T: Into<String>,
  {
    self.join_game(race.into(), name.into(), None)
  }

  pub fn view_singleplayer(&self, player: PlayerTag) -> Result<PlayerTag, CommandError> {
    self.join_game(player.into(), String::new(), None)
  }

  pub fn join_multiplayer<T>(
    &self,
    name: T,
    race: Race,
    connection: Connection,
  ) -> Result<PlayerTag, CommandError>
  where
    T: Into<String>,
  {
    self.join_game(race.into(), name.into(), Some(connection))
  }

  pub fn view_multiplayer(
    &self,
    player: PlayerTag,
    connection: Connection,
  ) -> Result<PlayerTag, CommandError> {
    self.join_game(player.into(), String::new(), Some(connection))
  }

  fn join_game(
    &self,
    participation: Participation,
    player_name: String,
    connection: Option<Connection>,
  ) -> Result<PlayerTag, CommandError> {
    let options: InterfaceOptions = InterfaceOptions {
      raw: true,
      score: true,
      feature_layer: None,
      render: None,
      show_cloaked: true,
      show_burrowed_shadows: true,
      show_placeholders: true,
      raw_affects_selection: self.cfg.raw_affects_selection,
      raw_crop_to_playable_area: false,
    };

    if let Some(connection) = connection {
      self.rpc.join_game(
        participation,
        options,
        Some(connection.server().into_proto()),
        [connection.client().into_proto()],
        player_name,
        connection.host().to_string(),
      )
    } else {
      self.rpc.join_game(
        participation,
        options,
        None,
        Vec::new(),
        player_name,
        String::new(),
      )
    }
  }

  pub fn restart_game(&self) -> Result<bool, CommandError> {
    self.rpc.restart_game()
  }

  pub fn leave_game(&self) -> Result<(), CommandError> {
    self.rpc.leave_game()
  }

  pub fn quick_save(&self) -> Result<(), CommandError> {
    self.rpc.quick_save()
  }

  pub fn quick_load(&self) -> Result<(), CommandError> {
    self.rpc.quick_load()
  }

  pub fn quit(&self) -> Result<(), CommandError> {
    self.rpc.quit()
  }

  pub fn game_info(&self) -> Result<GameInfo, CommandError> {
    self.rpc.game_info().map(IntoSc2::into_sc2)
  }

  pub fn game_data(&self) -> Result<GameData, CommandError> {
    self
      .rpc
      .data(true, true, true, true, true)
      .map(IntoSc2::into_sc2)
  }

  pub fn step(&self, count: u32) -> Result<u32, CommandError> {
    self.rpc.step(count)
  }

  pub fn save_replay(&self) -> Result<Bytes, CommandError> {
    self.rpc.save_replay()
  }

  pub fn ping(&self) -> Result<ResponsePing, CommandError> {
    self.rpc.ping()
  }
}

impl From<WebSocket> for Sc2Api {
  #[inline]
  fn from(other: WebSocket) -> Self {
    Self::new(Sc2Rpc::new(other))
  }
}

impl From<Sc2Rpc> for Sc2Api {
  #[inline]
  fn from(other: Sc2Rpc) -> Self {
    Self::new(other)
  }
}
