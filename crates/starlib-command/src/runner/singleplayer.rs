use std::error::Error;
use std::sync::Arc;

use starlib_proto::ResponsePing;
use starlib_types::ActionResult;
use starlib_types::GameData;
use starlib_types::GameInfo;
use starlib_types::GameResult;
use starlib_types::PlayerTag;

use crate::agent::Agent;
use crate::agent::AgentConfig;
use crate::agent::AgentState;
use crate::agent::Event;
use crate::client::Sc2Api;
use crate::error::CommandError;
use crate::error::CommandErrorKind;
use crate::setup::Computer;
use crate::setup::Map;

// General Flow
//
// - Create and Join Game
// - Agent Callback (on_create)
// - Fetch Static Data
// - Initialize Agent State
// - Fetch/Validate Observation (initial)
// - Update Agent State
// - Agent Callback (on_start)
// - Dispatch Actions
// - Core Loop
//   - Fetch/Validate Observation (step)
//   - Validate Match Timeout
//   - Update Agent State
//   - Dispatch Events
//   - Agent Callback (on_step)
//   - Dispatch Actions
//   - Trigger Step
pub fn run<A>(
  agent: &mut A,
  config: &AgentConfig,
  api: &Sc2Api,
  map: Map,
  opponents: &[Computer],
) -> Result<GameResult, CommandError>
where
  A: Agent + ?Sized,
{
  // ---------------------------------------------------------------------------
  // Create and Join Game
  // ---------------------------------------------------------------------------

  debug!("> Running StarCraft II Match");
  debug!("");
  debug!("        Map = {}", map);
  debug!("    Players = {}", 1 + opponents.len());
  debug!("  Player AI = {} ({})", config.race, config.name);
  debug!("Disable Fog = {}", api.cfg().disable_fog);
  debug!("Random Seed = {}", api.cfg().random_seed);
  debug!("   Realtime = {}", api.cfg().realtime);
  debug!(" Raw Select = {}", api.cfg().raw_affects_selection);
  debug!("    Timeout = {:?}", api.cfg().match_timeout);
  debug!("");

  let player_tag: PlayerTag = api.start_singleplayer(config.name, config.race, map, opponents)?;

  // ---------------------------------------------------------------------------
  // Agent Callback (on_create)
  // ---------------------------------------------------------------------------

  if let Err(error) = agent.on_create() {
    return exit_err(agent, error);
  }

  // ---------------------------------------------------------------------------
  // Fetch Static Data
  // ---------------------------------------------------------------------------

  let ping_info: ResponsePing = api.ping()?;
  let game_info: GameInfo = api.game_info()?;
  let game_data: GameData = api.game_data()?;

  // ---------------------------------------------------------------------------
  // Initialize Agent State
  // ---------------------------------------------------------------------------

  let mut state: AgentState<A> = AgentState::new(
    player_tag,
    config.race,
    config.name.to_owned(),
    Arc::clone(api.cfg()),
    ping_info,
    game_info,
    game_data,
  );

  // ---------------------------------------------------------------------------
  // Fetch/Validate Observation (initial)
  // ---------------------------------------------------------------------------

  state.update(api.rpc(), 0)?;

  if let Some(result) = state.game_result(player_tag) {
    return exit_ok(agent, result);
  }

  // ---------------------------------------------------------------------------
  // Update Agent State
  // ---------------------------------------------------------------------------

  state.harvest::<true>()?;

  // ---------------------------------------------------------------------------
  // Agent Callback (on_start)
  // ---------------------------------------------------------------------------

  if config.auto_construction {
    super::auto::do_auto_construction(&state)?;
  }

  if config.auto_distribution {
    super::auto::do_auto_distribution(&state)?;
  }

  if let Err(error) = agent.on_start(&state) {
    return exit_err(agent, error);
  }

  // ---------------------------------------------------------------------------
  // Dispatch Actions
  // ---------------------------------------------------------------------------

  let results: Vec<ActionResult> = state.dispatch(api.rpc())?;

  for result in results {
    if let Err(error) = agent.on_event(Event::ActionError(result)) {
      return exit_err(agent, error);
    }
  }

  // ---------------------------------------------------------------------------
  // Core Loop
  // ---------------------------------------------------------------------------

  let mut index: u32 = 0;

  loop {
    // -------------------------------------------------------------------------
    // Fetch/Validate Observation (step)
    // -------------------------------------------------------------------------

    if state.game_config().realtime && index != 0 {
      todo!()
    } else {
      state.update(api.rpc(), 0)?;
    }

    if let Some(result) = state.game_result(player_tag) {
      return exit_ok(agent, result);
    }

    // -------------------------------------------------------------------------
    // Validate Match Timeout
    // -------------------------------------------------------------------------

    if let Some(timeout) = state.game_config().match_timeout {
      if state.game_time() > timeout {
        return exit_ok(agent, GameResult::Tie);
      }
    }

    // -------------------------------------------------------------------------
    // Update Agent State
    // -------------------------------------------------------------------------

    state.harvest::<false>()?;

    // -------------------------------------------------------------------------
    // Dispatch Events
    // -------------------------------------------------------------------------

    if let Err(error) = state.trigger(agent) {
      return exit_err(agent, error);
    }

    // -------------------------------------------------------------------------
    // Agent Callback (on_step)
    // -------------------------------------------------------------------------

    if let Err(error) = agent.on_step(&state, index) {
      return exit_err(agent, error);
    }

    // -------------------------------------------------------------------------
    // Dispatch Actions
    // -------------------------------------------------------------------------

    let results: Vec<ActionResult> = state.dispatch(api.rpc())?;

    for result in results {
      if let Err(error) = agent.on_event(Event::ActionError(result)) {
        return exit_err(agent, error);
      }
    }

    // -------------------------------------------------------------------------
    // Trigger Step
    // -------------------------------------------------------------------------

    if !state.game_config().realtime {
      let _loop: u32 = api.step(state.game_config().step_count)?;
    }

    index = index.wrapping_add(1);
  }
}

fn exit_ok<A>(agent: &mut A, result: GameResult) -> Result<GameResult, CommandError>
where
  A: Agent + ?Sized,
{
  if let Err(error) = agent.on_end(result) {
    error!("> Agent Error = {error}");
  }

  Ok(result)
}

fn exit_err<A, E>(agent: &mut A, error: E) -> Result<GameResult, CommandError>
where
  A: Agent + ?Sized,
  E: Error + 'static,
{
  error!("> Agent Error = {error}");
  error!("> Resigning...");

  if let Err(error) = agent.on_end(GameResult::Defeat) {
    error!("> Agent Error = {error}");
  }

  Err(CommandError::new(error, CommandErrorKind::Agent))
}
