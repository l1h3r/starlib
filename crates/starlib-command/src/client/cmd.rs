use std::any::TypeId;
use std::borrow::Cow;
use std::marker::PhantomData;

use starlib_proto::Action as ProtoAction;
use starlib_proto::ActionResult;
use starlib_proto::DebugCommand;
use starlib_proto::InterfaceOptions;
use starlib_proto::Map;
use starlib_proto::ObserverAction as ProtoObserverAction;
use starlib_proto::Participation;
use starlib_proto::PlayerSetup;
use starlib_proto::PortSet;
use starlib_proto::Replay2;
use starlib_proto::Replay5;
use starlib_proto::RequestAction;
use starlib_proto::RequestAvailableMaps;
use starlib_proto::RequestCreateGame;
use starlib_proto::RequestData;
use starlib_proto::RequestDebug;
use starlib_proto::RequestGameInfo;
use starlib_proto::RequestJoinGame;
use starlib_proto::RequestLeaveGame;
use starlib_proto::RequestMapCommand;
use starlib_proto::RequestObservation;
use starlib_proto::RequestObserverAction;
use starlib_proto::RequestPing;
use starlib_proto::RequestQuery;
use starlib_proto::RequestQueryAvailableAbilities;
use starlib_proto::RequestQueryBuildingPlacement;
use starlib_proto::RequestQueryPathing;
use starlib_proto::RequestQuickLoad;
use starlib_proto::RequestQuickSave;
use starlib_proto::RequestQuit;
use starlib_proto::RequestReplayInfo;
use starlib_proto::RequestRestartGame;
use starlib_proto::RequestSaveMap;
use starlib_proto::RequestSaveReplay;
use starlib_proto::RequestStartReplay;
use starlib_proto::RequestStep;
use starlib_proto::ResponseAvailableMaps;
use starlib_proto::ResponseData;
use starlib_proto::ResponseGameInfo;
use starlib_proto::ResponseObservation;
use starlib_proto::ResponsePing;
use starlib_proto::ResponseQuery;
use starlib_proto::ResponseReplayInfo;
use starlib_proto::Status;
use starlib_types::Bytes;
use starlib_types::IntoProto;
use starlib_types::IntoSc2;
use starlib_types::PlayerTag;

use crate::client::Sc2Rpc;
use crate::error::CommandError;

// -----------------------------------------------------------------------------
// State
// -----------------------------------------------------------------------------

pub trait State: 'static {
  fn is_valid(status: Status) -> bool;

  fn expect() -> Cow<'static, str>;
}

// -----------------------------------------------------------------------------
// Available States
// -----------------------------------------------------------------------------

macro_rules! state {
  ($($name:ident)+) => {
    $(
      #[derive(Clone, Copy, Debug)]
      pub struct $name;

      impl State for $name {
        #[inline]
        fn is_valid(status: Status) -> bool {
          matches!(status, Status::$name)
        }

        #[inline]
        fn expect() -> ::std::borrow::Cow<'static, str> {
          ::std::borrow::Cow::Borrowed(concat!("`", stringify!($name), "`"))
        }
      }
    )+
  };
}

state!(Launched InitGame InGame InReplay Ended);

// -----------------------------------------------------------------------------
// Helper State - One of A, B, or C
// -----------------------------------------------------------------------------

pub struct OneOf<A, B, C = NoState> {
  marker_a: PhantomData<A>,
  marker_b: PhantomData<B>,
  marker_c: PhantomData<C>,
}

impl<A, B, C> State for OneOf<A, B, C>
where
  A: State,
  B: State,
  C: State,
{
  #[inline]
  fn is_valid(status: Status) -> bool {
    A::is_valid(status) || B::is_valid(status) || C::is_valid(status)
  }

  #[inline]
  fn expect() -> Cow<'static, str> {
    if TypeId::of::<C>() == TypeId::of::<NoState>() {
      Cow::Owned(format!("{} or {}", A::expect(), B::expect()))
    } else {
      Cow::Owned(format!(
        "one of {}, {}, or {}",
        A::expect(),
        B::expect(),
        C::expect()
      ))
    }
  }
}

// -----------------------------------------------------------------------------
// Helper State - No State is Valid
// -----------------------------------------------------------------------------

pub struct NoState;

impl State for NoState {
  #[inline]
  fn is_valid(_status: Status) -> bool {
    false
  }

  #[inline]
  fn expect() -> Cow<'static, str> {
    Cow::Borrowed("no state")
  }
}

// -----------------------------------------------------------------------------
// Helper State - Any State is Valid
// -----------------------------------------------------------------------------

pub struct AnyState;

impl State for AnyState {
  #[inline]
  fn is_valid(_status: Status) -> bool {
    true
  }

  #[inline]
  fn expect() -> Cow<'static, str> {
    Cow::Borrowed("any state")
  }
}

// -----------------------------------------------------------------------------
// RPC Command
// -----------------------------------------------------------------------------

pub trait Command {
  type State: State;

  type Argv;
  type Retv;

  fn run(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    if Self::State::is_valid(rpc.status()) {
      return Self::run_unchecked(rpc, input);
    }

    let expected: Cow<'_, str> = <Self::State as State>::expect();
    let received: Status = rpc.status();

    Err(CommandError::msg(format_args!(
      "invalid response status (expected {expected}, received `{received:?}`)",
    )))
  }

  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError>;
}

// -----------------------------------------------------------------------------
// RPC Command: CreateGame
// -----------------------------------------------------------------------------

pub struct CreateGame;

impl Command for CreateGame {
  type State = OneOf<Launched, Ended>;

  type Argv = (Map, Vec<PlayerSetup>, bool, u32, bool);
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestCreateGame = RequestCreateGame {
      map: Some(input.0),
      player_setup: input.1,
      disable_fog: input.2,
      random_seed: input.3,
      realtime: input.4,
    };

    rpc.try_command(request).map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: JoinGame
// -----------------------------------------------------------------------------

pub struct JoinGame;

impl Command for JoinGame {
  type State = OneOf<InitGame, Launched>;

  type Argv = (
    Participation,
    InterfaceOptions,
    Option<PortSet>,
    Vec<PortSet>,
    String,
    String,
  );

  type Retv = PlayerTag;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestJoinGame = RequestJoinGame {
      participation: Some(input.0),
      options: Some(input.1),
      server_ports: input.2,
      client_ports: input.3,
      #[expect(deprecated, reason = "raison")]
      shared_port: 0,
      player_name: input.4,
      host_ip: input.5,
    };

    rpc
      .try_command(request)
      .map(|response| response.player_id.into_sc2())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: RestartGame
// -----------------------------------------------------------------------------

pub struct RestartGame;

impl Command for RestartGame {
  type State = Ended;

  type Argv = ();
  type Retv = bool;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc
      .try_command(RequestRestartGame {})
      .map(|response| response.need_hard_reset)
  }
}

// -----------------------------------------------------------------------------
// RPC Command: StartReplay
// -----------------------------------------------------------------------------

pub struct StartReplay;

impl Command for StartReplay {
  type State = OneOf<Launched, Ended>;

  type Argv = (
    Replay5,
    Bytes,
    PlayerTag,
    InterfaceOptions,
    bool,
    bool,
    bool,
  );

  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestStartReplay = RequestStartReplay {
      replay: Some(input.0),
      map_data: input.1,
      observed_player_id: input.2.into_proto(),
      options: Some(input.3),
      disable_fog: input.4,
      realtime: input.5,
      record_replay: input.6,
    };

    rpc.try_command(request).map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: LeaveGame
// -----------------------------------------------------------------------------

pub struct LeaveGame;

impl Command for LeaveGame {
  type State = InGame;

  type Argv = ();
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestLeaveGame {}).map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: QuickSave
// -----------------------------------------------------------------------------

pub struct QuickSave;

impl Command for QuickSave {
  type State = InGame;

  type Argv = ();
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestQuickSave {}).map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: QuickLoad
// -----------------------------------------------------------------------------

pub struct QuickLoad;

impl Command for QuickLoad {
  type State = OneOf<InGame, Ended>;

  type Argv = ();
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestQuickLoad {}).map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Quit
// -----------------------------------------------------------------------------

pub struct Quit;

impl Command for Quit {
  type State = AnyState;

  type Argv = ();
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestQuit {}).map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: GameInfo
// -----------------------------------------------------------------------------

pub struct GameInfo;

impl Command for GameInfo {
  type State = OneOf<InGame, InReplay, Ended>;

  type Argv = ();
  type Retv = ResponseGameInfo;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestGameInfo {})
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Observation
// -----------------------------------------------------------------------------

pub struct Observation;

impl Command for Observation {
  type State = OneOf<InGame, InReplay, Ended>;

  type Argv = (bool, u32);
  type Retv = ResponseObservation;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestObservation = RequestObservation {
      disable_fog: input.0,
      game_loop: input.1,
    };

    rpc.command(request)
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Step
// -----------------------------------------------------------------------------

pub struct Step;

impl Command for Step {
  type State = OneOf<InGame, InReplay>;

  type Argv = u32;
  type Retv = u32;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc
      .command(RequestStep { count: input })
      .map(|response| response.simulation_loop)
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Action
// -----------------------------------------------------------------------------

pub struct Action;

impl Command for Action {
  type State = InGame;

  type Argv = Vec<ProtoAction>;
  type Retv = Vec<ActionResult>;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc
      .command(RequestAction { actions: input })
      .map(|response| Vec::from_iter(response.results()))
  }
}

// -----------------------------------------------------------------------------
// RPC Command: ObserverAction
// -----------------------------------------------------------------------------

pub struct ObserverAction;

impl Command for ObserverAction {
  type State = OneOf<InGame, InReplay>;

  type Argv = Vec<ProtoObserverAction>;
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc
      .command(RequestObserverAction { actions: input })
      .map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Data
// -----------------------------------------------------------------------------

pub struct Data;

impl Command for Data {
  type State = OneOf<InGame, InReplay, Ended>;

  type Argv = (bool, bool, bool, bool, bool);
  type Retv = ResponseData;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestData = RequestData {
      ability_id: input.0,
      unit_type_id: input.1,
      upgrade_id: input.2,
      buff_id: input.3,
      effect_id: input.4,
    };

    rpc.command(request)
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Query
// -----------------------------------------------------------------------------

pub struct Query;

impl Command for Query {
  type State = OneOf<InGame, InReplay, Ended>;

  type Argv = (
    Vec<RequestQueryPathing>,
    Vec<RequestQueryAvailableAbilities>,
    Vec<RequestQueryBuildingPlacement>,
    bool,
  );

  type Retv = ResponseQuery;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestQuery = RequestQuery {
      pathing: input.0,
      abilities: input.1,
      placements: input.2,
      ignore_resource_requirements: input.3,
    };

    rpc.command(request)
  }
}

// -----------------------------------------------------------------------------
// RPC Command: SaveReplay
// -----------------------------------------------------------------------------

pub struct SaveReplay;

impl Command for SaveReplay {
  type State = OneOf<InGame, Ended>;

  type Argv = ();
  type Retv = Bytes;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc
      .command(RequestSaveReplay {})
      .map(|response| response.data)
  }
}

// -----------------------------------------------------------------------------
// RPC Command: MapCommand
// -----------------------------------------------------------------------------

pub struct MapCommand;

impl Command for MapCommand {
  type State = InGame;

  type Argv = String;
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc
      .try_command(RequestMapCommand { command: input })
      .map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: ReplayInfo
// -----------------------------------------------------------------------------

pub struct ReplayInfo;

impl Command for ReplayInfo {
  type State = AnyState;

  type Argv = (Replay2, bool);
  type Retv = ResponseReplayInfo;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestReplayInfo = RequestReplayInfo {
      replay: Some(input.0),
      download_data: input.1,
    };

    rpc.try_command(request)
  }
}

// -----------------------------------------------------------------------------
// RPC Command: AvailableMaps
// -----------------------------------------------------------------------------

pub struct AvailableMaps;

impl Command for AvailableMaps {
  type State = AnyState;

  type Argv = ();
  type Retv = ResponseAvailableMaps;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestAvailableMaps {})
  }
}

// -----------------------------------------------------------------------------
// RPC Command: SaveMap
// -----------------------------------------------------------------------------

pub struct SaveMap;

impl Command for SaveMap {
  type State = AnyState;

  type Argv = (String, Bytes);
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    let request: RequestSaveMap = RequestSaveMap {
      map_path: input.0,
      map_data: input.1,
    };

    rpc.try_command(request).map(|_| ())
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Ping
// -----------------------------------------------------------------------------

pub struct Ping;

impl Command for Ping {
  type State = AnyState;

  type Argv = ();
  type Retv = ResponsePing;

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, _input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestPing {})
  }
}

// -----------------------------------------------------------------------------
// RPC Command: Debug
// -----------------------------------------------------------------------------

pub struct Debug;

impl Command for Debug {
  type State = InGame;

  type Argv = Vec<DebugCommand>;
  type Retv = ();

  #[inline]
  fn run_unchecked(rpc: &Sc2Rpc, input: Self::Argv) -> Result<Self::Retv, CommandError> {
    rpc.command(RequestDebug { debug: input }).map(|_| ())
  }
}
