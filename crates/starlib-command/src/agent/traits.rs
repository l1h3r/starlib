use std::error::Error;
use std::fs;

use starlib_types::Bytes;
use starlib_types::GameResult;
use starlib_types::Path;
use starlib_types::PathBuf;
use starlib_types::Race;

use crate::agent::AgentState;
use crate::agent::Event;
use crate::agent::Speed;
use crate::client::Sc2Api;
use crate::error::CommandError;
use crate::error::CommandErrorKind;
use crate::runner::singleplayer;
use crate::setup::Computer;
use crate::setup::Map;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AgentConfig {
  pub name: &'static str,
  pub race: Race,
  pub save_replay: Option<&'static Path>,
  pub auto_construction: bool,
  pub auto_distribution: bool,
}

impl AgentConfig {
  #[inline]
  pub const fn new(race: Race) -> Self {
    Self {
      name: "Agent",
      race,
      save_replay: None,
      auto_construction: false,
      auto_distribution: false,
    }
  }

  #[inline]
  pub const fn name(mut self, value: &'static str) -> Self {
    self.name = value;
    self
  }

  #[inline]
  pub const fn race(mut self, value: Race) -> Self {
    self.race = value;
    self
  }

  #[inline]
  pub fn save_replay<P>(mut self, value: &'static P) -> Self
  where
    P: AsRef<Path> + ?Sized,
  {
    self.save_replay = Some(value.as_ref());
    self
  }

  #[inline]
  pub const fn auto_construction(mut self, value: bool) -> Self {
    self.auto_construction = value;
    self
  }

  #[inline]
  pub const fn auto_distribution(mut self, value: bool) -> Self {
    self.auto_distribution = value;
    self
  }
}

pub trait Agent {
  type Error: Error + 'static;

  #[doc(hidden)]
  const SPEED: Speed = Speed::Faster;

  fn config(&self) -> AgentConfig;

  #[inline]
  fn on_create(&mut self) -> Result<(), Self::Error> {
    Ok(()) // do nothing
  }

  #[inline]
  fn on_start(&mut self, _state: &AgentState<Self>) -> Result<(), Self::Error> {
    Ok(()) // do nothing
  }

  #[inline]
  fn on_step(&mut self, _state: &AgentState<Self>, _index: u32) -> Result<(), Self::Error> {
    Ok(()) // do nothing
  }

  #[inline]
  fn on_event(&mut self, _event: Event<'_>) -> Result<(), Self::Error> {
    Ok(()) // do nothing
  }

  #[inline]
  fn on_end(&mut self, _result: GameResult) -> Result<(), Self::Error> {
    Ok(()) // do nothing
  }

  #[inline]
  fn run_singleplayer(
    &mut self,
    api: &Sc2Api,
    map: Map,
    opponents: &[Computer],
  ) -> Result<GameResult, CommandError> {
    let config: AgentConfig = self.config();

    let result: Result<GameResult, CommandError> =
      singleplayer::run(self, &config, api, map, opponents);

    if let Some(path) = config.save_replay {
      save_replay(path, api)?;
    }

    result
  }
}

fn save_replay(path: &Path, api: &Sc2Api) -> Result<(), CommandError> {
  let data: Bytes = api.save_replay()?;
  let path: PathBuf = path.with_extension("SC2Replay");

  fs::write(path, data).map_err(|error| CommandError::new(error, CommandErrorKind::Replay))
}
