use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use starlib_proto::Action;
use starlib_proto::ActionError;
use starlib_proto::ActionResult as ProtoResult;
use starlib_proto::ChatReceived;
use starlib_proto::Message;
use starlib_proto::Observation;
use starlib_proto::PlayerResult;
use starlib_proto::RequestObservation;
use starlib_proto::Response;
use starlib_proto::ResponseObservation;
use starlib_proto::ResponsePing;
use starlib_proto::ResponseVariant;
use starlib_types::ActionResult;
use starlib_types::Float;
use starlib_types::GameData;
use starlib_types::GameInfo;
use starlib_types::GameResult;
use starlib_types::GameState;
use starlib_types::IntoSc2;
use starlib_types::PlayerTag;
use starlib_types::Race;
use starlib_types::UnitIter;
use starlib_types::UnitRef;
use starlib_types::UnitTag;
use starlib_types::UnitTypeId;
use starlib_types::UpgradeId;

use crate::agent::Agent;
use crate::agent::Event;
use crate::client::Sc2Rpc;
use crate::config::Config;
use crate::error::CommandError;

// -----------------------------------------------------------------------------
// Context Data - Unit Frame
// -----------------------------------------------------------------------------

struct UnitFrame {
  kind: UnitTypeId,
  health: Float,
  shield: Float,
  progress: Float,
}

impl UnitFrame {
  #[inline]
  fn new(unit: UnitRef<'_>) -> Self {
    Self {
      kind: unit.kind(),
      health: unit.health().current(),
      shield: unit.shield().current(),
      progress: *unit.build_progress(),
    }
  }
}

// -----------------------------------------------------------------------------
// Context Data - Private Data
// -----------------------------------------------------------------------------

struct PrivateData {
  stale: Response,
  fresh: Response,
  queue: Vec<Action>,
}

impl PrivateData {
  #[inline]
  fn update(&mut self, rpc: &Sc2Rpc, request: RequestObservation) -> Result<(), CommandError> {
    self.stale.clear();

    mem::swap(&mut self.stale, &mut self.fresh);

    rpc.request(request, &mut self.fresh)?;

    Ok(())
  }

  #[inline]
  fn snapshot(&self) -> &ResponseObservation {
    match self.fresh.response.as_ref() {
      Some(ResponseVariant::Observation(inner)) => inner,
      Some(_) | None => unreachable!(),
    }
  }

  #[inline]
  fn actions(&self) -> &[Action] {
    self.snapshot().actions.as_slice()
  }

  #[inline]
  fn errors(&self) -> &[ActionError] {
    self.snapshot().action_errors.as_slice()
  }

  #[inline]
  fn observation(&self) -> &Observation {
    match self.snapshot().observation.as_ref() {
      Some(inner) => inner,
      None => unreachable!(),
    }
  }

  #[inline]
  fn results(&self) -> &[PlayerResult] {
    self.snapshot().player_result.as_slice()
  }

  #[inline]
  fn chats(&self) -> &[ChatReceived] {
    self.snapshot().chat.as_slice()
  }
}

// -----------------------------------------------------------------------------
// Agent State
// -----------------------------------------------------------------------------

pub struct AgentState<A: ?Sized> {
  __phantom: PhantomData<A>,
  __private: PrivateData,

  game_config: Arc<Config>,
  game_result: HashMap<PlayerTag, GameResult>,
  game_state: Arc<GameState>,
  cached_loop: u32,
  cached_time: Duration,

  known_upgrades: HashSet<UpgradeId>,
  previous_units: HashSet<UnitTag>,
  previous_owned: HashMap<UnitTag, UnitFrame>,
  previous_enemy: HashSet<UnitTag>,
}

impl<A> AgentState<A>
where
  A: ?Sized,
{
  const CAP_QUEUE: usize = 64;

  #[inline]
  pub(crate) fn new(
    player_tag: PlayerTag,
    player_race: Race,
    player_name: String,
    game_config: Arc<Config>,
    ping_info: ResponsePing,
    game_info: GameInfo,
    game_data: GameData,
  ) -> Self {
    Self {
      __phantom: PhantomData,
      __private: PrivateData {
        stale: Default::default(),
        fresh: Default::default(),
        queue: Vec::with_capacity(Self::CAP_QUEUE),
      },
      game_config,
      game_result: HashMap::new(),
      game_state: Arc::new(GameState::new(
        player_tag,
        player_race,
        player_name,
        ping_info,
        game_info,
        game_data,
      )),
      cached_loop: 0,
      cached_time: Duration::ZERO,
      known_upgrades: HashSet::new(),
      previous_units: HashSet::new(),
      previous_owned: HashMap::new(),
      previous_enemy: HashSet::new(),
    }
  }

  pub(crate) fn update(&mut self, rpc: &Sc2Rpc, block_until: u32) -> Result<(), CommandError>
  where
    A: Agent,
  {
    self.update_snapshot(rpc, block_until)?;
    self.update_results();
    self.update_cached();

    Ok(())
  }

  #[inline]
  fn update_snapshot(&mut self, rpc: &Sc2Rpc, block_until: u32) -> Result<(), CommandError> {
    let request: RequestObservation = RequestObservation {
      disable_fog: self.game_config.disable_fog,
      game_loop: block_until,
    };

    self.__private.update(rpc, request)?;

    Ok(())
  }

  #[inline]
  fn update_results(&mut self) {
    let snapshot: &ResponseObservation = self.__private.snapshot();

    if snapshot.player_result.is_empty() {
      return;
    }

    self.game_result.clear();

    for result in snapshot.player_result.iter() {
      self
        .game_result
        .insert(result.player_id.into_sc2(), result.result.into_sc2());
    }
  }

  #[inline]
  fn update_cached(&mut self)
  where
    A: Agent,
  {
    self.cached_loop = self.__private.observation().game_loop;
    self.cached_time = Duration::from_secs_f64(self.cached_loop as f64 / A::SPEED.fps());
  }

  pub(crate) fn harvest<const INIT: bool>(&mut self) -> Result<(), CommandError> {
    let observation: &Observation = self.__private.observation();

    assert_eq!(Arc::strong_count(&self.game_state), 1);
    assert_eq!(Arc::weak_count(&self.game_state), 0);

    let Some(state) = Arc::get_mut(&mut self.game_state) else {
      unreachable!()
    };

    self.previous_units.clear();
    self.previous_owned.clear();
    self.previous_enemy.clear();

    for unit in state.units() {
      self.previous_units.insert(unit.tag());

      if unit.is_owned() {
        self.previous_owned.insert(unit.tag(), UnitFrame::new(unit));
      } else if unit.is_enemy() {
        self.previous_enemy.insert(unit.tag());
      }
    }

    state.update::<INIT>(observation);

    Ok(())
  }

  pub(crate) fn trigger(&mut self, agent: &mut A) -> Result<(), A::Error>
  where
    A: Agent,
  {
    // -------------------------------------------------------------------------
    // Handle Events - Upgrades
    // -------------------------------------------------------------------------

    for upgrade in self.game_state.upgrades().difference(&self.known_upgrades) {
      agent.on_event(Event::UpgradeComplete(*upgrade))?;
    }

    // -------------------------------------------------------------------------
    // Handle Events - Unit Killed
    // -------------------------------------------------------------------------

    for tag in self
      .game_state
      .dead_now()
      .intersection(&self.previous_units)
    {
      agent.on_event(Event::UnitKilled(*tag))?;
    }

    // -------------------------------------------------------------------------
    // Handle Events - Owned Units
    // -------------------------------------------------------------------------

    for unit in self.game_state.units().owned() {
      if let Some(frame) = self.previous_owned.get(&unit.tag()) {
        if unit.is_structure() {
          if unit.build_complete() && !frame.progress.epsilon_eq(1.0) {
            agent.on_event(Event::BuildingComplete(unit))?;
          }
        }

        if unit.health().current() < frame.health || unit.shield().current() < frame.shield {
          let dmg_health: Float = frame.health - unit.health().current();
          let dmg_shield: Float = frame.shield - unit.shield().current();

          agent.on_event(Event::UnitDamaged(unit, dmg_health, dmg_shield))?;
        }

        if unit.kind() != frame.kind {
          agent.on_event(Event::UnitMorphed(unit, frame.kind))?;
        }
      } else if unit.is_structure() {
        if unit.build_complete() {
          agent.on_event(Event::BuildingComplete(unit))?;
        } else {
          agent.on_event(Event::BuildingPending(unit))?;
        }
      } else {
        agent.on_event(Event::UnitCreated(unit))?;
      }
    }

    // -------------------------------------------------------------------------
    // Handle Events - Enemy Units
    // -------------------------------------------------------------------------

    for unit in self.game_state.units().enemies() {
      if !self.previous_enemy.contains(&unit.tag()) {
        agent.on_event(Event::EnterVision(unit))?;
      }
    }

    for tag in self.previous_enemy.difference(self.game_state.tags_set()) {
      agent.on_event(Event::LeaveVision(*tag))?;
    }

    // -------------------------------------------------------------------------
    // Update State
    // -------------------------------------------------------------------------

    self.known_upgrades.extend(self.game_state.upgrades());

    Ok(())
  }

  pub(crate) fn dispatch(&mut self, rpc: &Sc2Rpc) -> Result<Vec<ActionResult>, CommandError> {
    self
      .game_state
      .commands()
      .drain(|action| self.__private.queue.push(action));

    let errors: Vec<ActionResult> = rpc
      .action(self.__private.queue.drain(..))?
      .into_iter()
      .filter(|event| !matches!(event, ProtoResult::Success))
      .map(IntoSc2::into_sc2)
      .collect();

    Ok(errors)
  }

  // ---------------------------------------------------------------------------
  // Public API
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn game_config(&self) -> &Arc<Config> {
    &self.game_config
  }

  #[inline]
  pub fn game_result(&self, player: PlayerTag) -> Option<GameResult> {
    self.game_result.get(&player).copied()
  }

  #[inline]
  pub fn game_state(&self) -> &Arc<GameState> {
    &self.game_state
  }

  #[inline]
  pub fn game_loop(&self) -> u32 {
    self.cached_loop
  }

  #[inline]
  pub fn game_time(&self) -> Duration {
    self.cached_time
  }

  #[inline]
  pub fn proto_snapshot(&self) -> &ResponseObservation {
    self.__private.snapshot()
  }

  #[inline]
  pub fn proto_actions(&self) -> &[Action] {
    self.__private.actions()
  }

  #[inline]
  pub fn proto_errors(&self) -> &[ActionError] {
    self.__private.errors()
  }

  #[inline]
  pub fn proto_observation(&self) -> &Observation {
    self.__private.observation()
  }

  #[inline]
  pub fn proto_results(&self) -> &[PlayerResult] {
    self.__private.results()
  }

  #[inline]
  pub fn proto_chats(&self) -> &[ChatReceived] {
    self.__private.chats()
  }
}

impl<A> Debug for AgentState<A>
where
  A: ?Sized,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("AgentState")
      .field("game_config", &self.game_config)
      .field("game_result", &self.game_result)
      .field("game_state", &self.game_state)
      .field("game_loop", &self.cached_loop)
      .field("game_time", &self.cached_time)
      .finish_non_exhaustive()
  }
}

impl<A> Deref for AgentState<A>
where
  A: ?Sized,
{
  type Target = Arc<GameState>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.game_state
  }
}
