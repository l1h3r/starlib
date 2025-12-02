use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use starlib_proto::LocalMap;
use starlib_proto::Map as ProtoMap;
use starlib_proto::PlayerSetup;
use starlib_proto::PlayerType;
use starlib_types::AiBuild;
use starlib_types::Bytes;
use starlib_types::Difficulty;
use starlib_types::FromSc2;
use starlib_types::IntoProto;
use starlib_types::PathBuf;
use starlib_types::PlayerTag;
use starlib_types::Race;

// -----------------------------------------------------------------------------
// Player - StarCraft II AI
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Computer {
  pub(crate) race: Race,
  pub(crate) name: String,
  pub(crate) ai_build_type: AiBuild,
  pub(crate) ai_difficulty: Difficulty,
}

impl Computer {
  #[inline]
  pub fn new<T>(name: T, race: Race) -> Self
  where
    T: Into<String>,
  {
    Self {
      race,
      name: name.into(),
      ai_build_type: AiBuild::RandomBuild,
      ai_difficulty: Difficulty::Easy,
    }
  }

  #[inline]
  pub const fn ai_build_type(mut self, value: AiBuild) -> Self {
    self.ai_build_type = value;
    self
  }

  #[inline]
  pub const fn ai_difficulty(mut self, value: Difficulty) -> Self {
    self.ai_difficulty = value;
    self
  }
}

impl FromSc2<Computer> for PlayerSetup {
  #[inline]
  fn from_sc2(other: Computer) -> Self {
    PlayerSetup {
      kind: PlayerType::Computer as i32,
      race: other.race as i32,
      difficulty: other.ai_difficulty as i32,
      player_name: other.name,
      ai_build: other.ai_build_type as i32,
    }
  }
}

impl<'a> FromSc2<&'a Computer> for PlayerSetup {
  #[inline]
  fn from_sc2(other: &'a Computer) -> Self {
    PlayerSetup {
      kind: PlayerType::Computer as i32,
      race: other.race as i32,
      difficulty: other.ai_difficulty as i32,
      player_name: other.name.to_owned(),
      ai_build: other.ai_build_type as i32,
    }
  }
}

// -----------------------------------------------------------------------------
// Player - Observer
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Observer {
  pub(crate) player: PlayerTag,
}

impl Observer {
  #[inline]
  pub const fn new(player: PlayerTag) -> Self {
    Self { player }
  }
}

impl FromSc2<Observer> for PlayerSetup {
  #[inline]
  fn from_sc2(_other: Observer) -> Self {
    PlayerSetup {
      kind: PlayerType::Observer as i32,
      ..Default::default()
    }
  }
}

impl<'a> FromSc2<&'a Observer> for PlayerSetup {
  #[inline]
  fn from_sc2(_other: &'a Observer) -> Self {
    PlayerSetup {
      kind: PlayerType::Observer as i32,
      ..Default::default()
    }
  }
}

// -----------------------------------------------------------------------------
// Player - User AI
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PlayerAi {
  pub(crate) race: Race,
  pub(crate) name: String,
}

impl PlayerAi {
  #[inline]
  pub fn new<T>(name: T, race: Race) -> Self
  where
    T: Into<String>,
  {
    Self {
      race,
      name: name.into(),
    }
  }
}

impl FromSc2<PlayerAi> for PlayerSetup {
  #[inline]
  fn from_sc2(_other: PlayerAi) -> Self {
    PlayerSetup {
      kind: PlayerType::Participant as i32,
      ..Default::default()
    }
  }
}

impl<'a> FromSc2<&'a PlayerAi> for PlayerSetup {
  #[inline]
  fn from_sc2(_other: &'a PlayerAi) -> Self {
    PlayerSetup {
      kind: PlayerType::Participant as i32,
      ..Default::default()
    }
  }
}

// -----------------------------------------------------------------------------
// Player
// -----------------------------------------------------------------------------

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Player {
  Computer(Computer),
  Observer(Observer),
  PlayerAi(PlayerAi),
}

impl Debug for Player {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Computer(inner) => Debug::fmt(inner, f),
      Self::Observer(inner) => Debug::fmt(inner, f),
      Self::PlayerAi(inner) => Debug::fmt(inner, f),
    }
  }
}

impl From<Computer> for Player {
  #[inline]
  fn from(other: Computer) -> Self {
    Self::Computer(other)
  }
}

impl From<Observer> for Player {
  #[inline]
  fn from(other: Observer) -> Self {
    Self::Observer(other)
  }
}

impl From<PlayerAi> for Player {
  #[inline]
  fn from(other: PlayerAi) -> Self {
    Self::PlayerAi(other)
  }
}

impl FromSc2<Player> for PlayerSetup {
  #[inline]
  fn from_sc2(other: Player) -> Self {
    match other {
      Player::Computer(inner) => FromSc2::from_sc2(inner),
      Player::Observer(inner) => FromSc2::from_sc2(inner),
      Player::PlayerAi(inner) => FromSc2::from_sc2(inner),
    }
  }
}

impl<'a> FromSc2<&'a Player> for PlayerSetup {
  #[inline]
  fn from_sc2(other: &'a Player) -> Self {
    match other {
      Player::Computer(inner) => FromSc2::from_sc2(inner),
      Player::Observer(inner) => FromSc2::from_sc2(inner),
      Player::PlayerAi(inner) => FromSc2::from_sc2(inner),
    }
  }
}

// -----------------------------------------------------------------------------
// Map
// -----------------------------------------------------------------------------

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Map {
  Internal(InternalMap),
  External(String),
}

impl Map {
  #[inline]
  pub fn name<T>(value: T) -> Self
  where
    T: Into<String>,
  {
    Self::External(value.into())
  }

  #[inline]
  pub fn path<T>(value: T) -> Self
  where
    T: Into<PathBuf>,
  {
    Self::Internal(InternalMap::from_path(value))
  }

  #[inline]
  pub fn data<T>(value: T) -> Self
  where
    T: Into<Bytes>,
  {
    Self::Internal(InternalMap::from_data(value))
  }
}

impl Debug for Map {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Internal(inner) => Debug::fmt(inner, f),
      Self::External(inner) => Debug::fmt(inner, f),
    }
  }
}

impl Display for Map {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Internal(inner) => Display::fmt(inner, f),
      Self::External(inner) => Display::fmt(inner, f),
    }
  }
}

impl FromSc2<Map> for ProtoMap {
  #[inline]
  fn from_sc2(other: Map) -> Self {
    match other {
      Map::Internal(inner) => Self::LocalMap(inner.into_proto()),
      Map::External(inner) => Self::BattleNetMapName(inner),
    }
  }
}

// -----------------------------------------------------------------------------
// Internal Map
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum InternalMap {
  Path(PathBuf),
  Data(Bytes),
  Both(PathBuf, Bytes),
}

impl InternalMap {
  #[inline]
  pub fn from_path<T>(value: T) -> Self
  where
    T: Into<PathBuf>,
  {
    Self::Path(value.into())
  }

  #[inline]
  pub fn from_data<T>(value: T) -> Self
  where
    T: Into<Bytes>,
  {
    Self::Data(value.into())
  }

  #[inline]
  pub fn path<T>(self, value: T) -> Self
  where
    T: Into<PathBuf>,
  {
    match self {
      Self::Path(_) => Self::Path(value.into()),
      Self::Data(data) => Self::Both(value.into(), data),
      Self::Both(_, data) => Self::Both(value.into(), data),
    }
  }

  #[inline]
  pub fn data<T>(self, value: T) -> Self
  where
    T: Into<Bytes>,
  {
    match self {
      Self::Path(path) => Self::Both(path, value.into()),
      Self::Data(_) => Self::Data(value.into()),
      Self::Both(path, _) => Self::Both(path, value.into()),
    }
  }
}

impl Display for InternalMap {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Path(path) => match path.file_name() {
        Some(name) => write!(f, "Map<{name}>"),
        None => write!(f, "Map<{path}>"),
      },
      Self::Data(data) => {
        write!(f, "Map<{} bytes>", data.len())
      }
      Self::Both(path, data) => match path.file_name() {
        Some(name) => write!(f, "Map<{} bytes as {name}>", data.len()),
        None => write!(f, "Map<{} bytes as {path}>", data.len()),
      },
    }
  }
}

impl FromSc2<InternalMap> for LocalMap {
  #[inline]
  fn from_sc2(other: InternalMap) -> Self {
    let (map_path, map_data): (String, Bytes) = match other {
      InternalMap::Path(path) => (path.to_string(), Bytes::new()),
      InternalMap::Data(data) => (String::new(), data),
      InternalMap::Both(path, data) => (path.to_string(), data),
    };

    Self { map_path, map_data }
  }
}
