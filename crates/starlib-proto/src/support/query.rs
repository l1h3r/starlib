use prost::Message;
use prost::Oneof;

use crate::support::ActionResult;
use crate::support::AvailableAbility;
use crate::support::Point2D;

#[derive(Clone, Copy, Oneof)]
pub enum RequestQueryPathingStart {
  #[prost(message, tag = "1")]
  StartPos(Point2D),
  #[prost(uint64, tag = "2")]
  UnitTag(u64),
}

#[derive(Clone, Copy, Message)]
pub struct RequestQueryPathing {
  #[prost(oneof = "RequestQueryPathingStart", tags = "1,2")]
  pub start: Option<RequestQueryPathingStart>,
  #[prost(message, tag = "3")]
  pub end_pos: Option<Point2D>,
}

#[derive(Clone, Copy, Message)]
pub struct ResponseQueryPathing {
  #[prost(float, tag = "1")]
  pub distance: f32,
}

#[derive(Clone, Copy, Message)]
pub struct RequestQueryAvailableAbilities {
  #[prost(uint64, tag = "1")]
  pub unit_tag: u64,
}

#[derive(Clone, Message)]
pub struct ResponseQueryAvailableAbilities {
  #[prost(repeated, message, tag = "1")]
  pub abilities: Vec<AvailableAbility>,
  #[prost(uint64, tag = "2")]
  pub unit_tag: u64,
  #[prost(uint32, tag = "3")]
  pub unit_type_id: u32,
}

#[derive(Clone, Copy, Message)]
pub struct RequestQueryBuildingPlacement {
  #[prost(int32, tag = "1")]
  pub ability_id: i32,
  #[prost(message, tag = "2")]
  pub target_pos: Option<Point2D>,
  #[prost(uint64, tag = "3")]
  pub placing_unit_tag: u64,
}

#[derive(Clone, Copy, Message)]
pub struct ResponseQueryBuildingPlacement {
  #[prost(enumeration = "ActionResult", tag = "1")]
  pub result: i32,
}
