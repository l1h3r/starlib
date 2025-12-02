//! StarLib API Interface

#[macro_use]
extern crate starlib_utils;

mod agent;
mod client;
mod config;
mod error;
mod runner;
mod setup;

pub use self::agent::Agent;
pub use self::agent::AgentConfig;
pub use self::agent::AgentState;
pub use self::agent::Event;
pub use self::agent::Speed;

pub use self::client::Sc2Api;
pub use self::client::Sc2Rpc;

pub use self::config::Config;

pub use self::error::CommandError;
pub use self::error::CommandErrorKind;

pub use self::setup::Computer;
pub use self::setup::Connection;
pub use self::setup::InternalMap;
pub use self::setup::Map;
pub use self::setup::Observer;
pub use self::setup::Player;
pub use self::setup::PlayerAi;
pub use self::setup::Ports;
