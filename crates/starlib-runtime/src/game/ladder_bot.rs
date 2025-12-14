use std::sync::Arc;

use starlib_command::Agent;
use starlib_command::AgentConfig;
use starlib_command::AgentState;
use starlib_command::CommandError;
use starlib_command::Computer;
use starlib_command::Event;
use starlib_command::Map;
use starlib_command::Sc2Api;
use starlib_types::GameData;
use starlib_types::GameResult;
use starlib_types::Race;

use crate::core::ControlFlow;
use crate::core::DynOperation;
use crate::core::Par;
use crate::exec::Argument;
use crate::exec::Executor;
use crate::exec::System;
use crate::exec::WorldRef;
use crate::game::CostTracker;
use crate::game::UnitTracker;

// -----------------------------------------------------------------------------
// Ladder Bot Configuration
// -----------------------------------------------------------------------------

pub trait BotConfig {
  fn race(&self) -> Race;
  fn init(&self) -> Par;
}

// -----------------------------------------------------------------------------
// Build Order System
// -----------------------------------------------------------------------------

struct BotBuild(Par);

impl System for BotBuild {
  type Argv = WorldRef<'static>;

  #[inline]
  fn setup(&mut self, _world: &mut World) -> Result<(), String> {
    self.0.setup()
  }

  #[inline]
  fn run(&mut self, input: <Self::Argv as Argument>::Item<'_>) {
    let _unused: ControlFlow = self.0.run(input);
  }
}

// -----------------------------------------------------------------------------
// Ladder Bot
// -----------------------------------------------------------------------------

pub struct LadderBot<T> {
  behavior: T,
  executor: Executor,
}

impl<T> LadderBot<T> {
  #[inline]
  pub fn new(behavior: T) -> Self {
    Self {
      behavior,
      executor: Executor::new(),
    }
  }

  #[inline]
  pub fn run_1v1(
    mut self,
    api: &Sc2Api,
    map: Map,
    opponent: Race,
  ) -> Result<GameResult, CommandError>
  where
    T: BotConfig,
  {
    self.run_singleplayer(api, map, &[Computer::new("Computer AI", opponent)])
  }
}

impl<T> Agent for LadderBot<T>
where
  T: BotConfig,
{
  type Error = CommandError;

  #[inline]
  fn config(&self) -> AgentConfig {
    AgentConfig::new(self.behavior.race())
      .name("Ladder Bot")
      .auto_construction(true)
      .auto_distribution(true)
  }

  #[inline]
  fn on_create(&mut self) -> Result<(), Self::Error> {
    println!("[LadderBot]: OnCreate");

    Ok(())
  }

  #[inline]
  fn on_start(&mut self, state: &AgentState<Self>) -> Result<(), Self::Error> {
    println!("[LadderBot]: OnStart");

    let data: Arc<GameData> = Arc::clone(state.game_data());

    self.executor.insert_resource(CostTracker::new());
    self.executor.insert_resource(UnitTracker::new(data));

    self.executor.insert_system(BotBuild(self.behavior.init()));

    if let Err(error) = self.executor.setup() {
      return Err(CommandError::msg(error));
    }

    Ok(())
  }

  #[inline]
  fn on_step(&mut self, state: &AgentState<Self>, _index: u32) -> Result<(), Self::Error> {
    self.executor.run(Arc::clone(state.game_state()));

    Ok(())
  }

  #[inline]
  fn on_event(&mut self, event: Event<'_>) -> Result<(), Self::Error> {
    println!("[LadderBot]: OnEvent {event}");

    Ok(())
  }

  #[inline]
  fn on_end(&mut self, result: GameResult) -> Result<(), Self::Error> {
    println!("[LadderBot]: OnEnd ({result})");

    Ok(())
  }
}
