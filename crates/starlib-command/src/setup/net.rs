use std::net::IpAddr;

use starlib_proto::PortSet;
use starlib_types::FromSc2;

// -----------------------------------------------------------------------------
// Connection
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Connection {
  host: IpAddr,
  client: Ports,
  server: Ports,
}

impl Connection {
  #[inline]
  pub fn new<T>(host: T, client: Ports, server: Ports) -> Self
  where
    T: Into<IpAddr>,
  {
    Self {
      host: host.into(),
      client,
      server,
    }
  }

  #[inline]
  pub const fn host(&self) -> IpAddr {
    self.host
  }

  #[inline]
  pub const fn client(&self) -> Ports {
    self.client
  }

  #[inline]
  pub const fn server(&self) -> Ports {
    self.server
  }
}

// -----------------------------------------------------------------------------
// Ports
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ports {
  game: u16,
  base: u16,
}

impl Ports {
  #[inline]
  pub const fn new(game: u16, base: u16) -> Self {
    Self { game, base }
  }

  #[inline]
  pub const fn game(&self) -> u16 {
    self.game
  }

  #[inline]
  pub const fn base(&self) -> u16 {
    self.base
  }
}

impl FromSc2<Ports> for PortSet {
  #[inline]
  fn from_sc2(other: Ports) -> Self {
    Self {
      game_port: other.game as i32,
      base_port: other.base as i32,
    }
  }
}
