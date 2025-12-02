use crate::command::RequestVariant;
use crate::command::ResponseSaveMap;
use crate::command::ResponseSaveMapError;
use crate::command::ResponseVariant;

// -----------------------------------------------------------------------------
// Seal
// -----------------------------------------------------------------------------

mod private {
  pub trait Sealed {}
}

// -----------------------------------------------------------------------------
// Command Send
// -----------------------------------------------------------------------------

pub trait CommandSend: private::Sealed + Sized {
  type Recv: CommandRecv<Send = Self>;

  fn into_variant(self) -> RequestVariant;
  fn from_variant(this: RequestVariant) -> Option<Self>;
}

// -----------------------------------------------------------------------------
// Command Recv
// -----------------------------------------------------------------------------

pub trait CommandRecv: private::Sealed + Sized {
  type Send: CommandSend<Recv = Self>;

  fn into_variant(self) -> ResponseVariant;
  fn from_variant(this: ResponseVariant) -> Option<Self>;
}

// -----------------------------------------------------------------------------
// Command Error
// -----------------------------------------------------------------------------

pub trait CommandError: private::Sealed {
  type Error;
  fn to_error(&self) -> Option<String>;
}

// -----------------------------------------------------------------------------
// Send/Recv Implementations
// -----------------------------------------------------------------------------

macro_rules! command {
  ($variant:ident, $request:ident => $response:ident) => {
    impl $crate::traits::private::Sealed for $crate::command::$request {}
    impl $crate::traits::private::Sealed for $crate::command::$response {}

    impl $crate::traits::CommandSend for $crate::command::$request {
      type Recv = $crate::command::$response;

      #[inline]
      fn into_variant(self) -> $crate::command::RequestVariant {
        $crate::command::RequestVariant::$variant(self)
      }

      #[inline]
      fn from_variant(this: $crate::command::RequestVariant) -> Option<Self> {
        match this {
          $crate::command::RequestVariant::$variant(this) => Some(this),
          _ => None,
        }
      }
    }

    impl $crate::traits::CommandRecv for $crate::command::$response {
      type Send = $crate::command::$request;

      #[inline]
      fn into_variant(self) -> $crate::command::ResponseVariant {
        $crate::command::ResponseVariant::$variant(self)
      }

      #[inline]
      fn from_variant(this: $crate::command::ResponseVariant) -> Option<Self> {
        match this {
          $crate::command::ResponseVariant::$variant(this) => Some(this),
          _ => None,
        }
      }
    }
  };
}

command!(CreateGame, RequestCreateGame => ResponseCreateGame);
command!(JoinGame, RequestJoinGame => ResponseJoinGame);
command!(RestartGame, RequestRestartGame => ResponseRestartGame);
command!(StartReplay, RequestStartReplay => ResponseStartReplay);
command!(LeaveGame, RequestLeaveGame => ResponseLeaveGame);

command!(QuickSave, RequestQuickSave => ResponseQuickSave);
command!(QuickLoad, RequestQuickLoad => ResponseQuickLoad);

command!(Quit, RequestQuit => ResponseQuit);

command!(GameInfo, RequestGameInfo => ResponseGameInfo);
command!(Observation, RequestObservation => ResponseObservation);
command!(Action, RequestAction => ResponseAction);
command!(ObserverAction, RequestObserverAction => ResponseObserverAction);
command!(Step, RequestStep => ResponseStep);
command!(Data, RequestData => ResponseData);
command!(Query, RequestQuery => ResponseQuery);
command!(SaveReplay, RequestSaveReplay => ResponseSaveReplay);
command!(MapCommand, RequestMapCommand => ResponseMapCommand);

command!(ReplayInfo, RequestReplayInfo => ResponseReplayInfo);
command!(AvailableMaps, RequestAvailableMaps => ResponseAvailableMaps);
command!(SaveMap, RequestSaveMap => ResponseSaveMap);

command!(Ping, RequestPing => ResponsePing);
command!(Debug, RequestDebug => ResponseDebug);

// -----------------------------------------------------------------------------
// Error Implementations
// -----------------------------------------------------------------------------

macro_rules! error {
  ($error:ident in $response:ident) => {
    impl $crate::traits::CommandError for $crate::command::$response {
      type Error = $crate::command::$error;

      fn to_error(&self) -> Option<String> {
        let error_type: Self::Error = self.error();
        let error_info: &str = self.error_details.as_str();

        if matches!(error_type, Self::Error::NULL) {
          return None;
        }

        if error_info.is_empty() {
          Some(format!("error ({error_type:?})"))
        } else {
          Some(format!("error ({error_type:?}, {error_info})"))
        }
      }
    }
  };
}

error!(ResponseCreateGameError in ResponseCreateGame);
error!(ResponseJoinGameError in ResponseJoinGame);
error!(ResponseRestartGameError in ResponseRestartGame);
error!(ResponseStartReplayError in ResponseStartReplay);
error!(ResponseMapCommandError in ResponseMapCommand);
error!(ResponseReplayInfoError in ResponseReplayInfo);

impl CommandError for ResponseSaveMap {
  type Error = ResponseSaveMapError;

  fn to_error(&self) -> Option<String> {
    let error_type: Self::Error = self.error();

    if matches!(error_type, Self::Error::NULL) {
      return None;
    }

    Some(format!("error ({error_type:?})"))
  }
}
