use starlib_proto::Participation;

use crate::bridge::Race;
use crate::core::PlayerTag;
use crate::traits::IntoProto;

impl From<Race> for Participation {
  #[inline]
  fn from(other: Race) -> Self {
    Participation::Race(other.into_proto())
  }
}

impl From<PlayerTag> for Participation {
  #[inline]
  fn from(other: PlayerTag) -> Self {
    Participation::ObservedPlayerId(other.into_proto())
  }
}
