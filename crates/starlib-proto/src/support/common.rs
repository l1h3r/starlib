use prost::Enumeration;
use prost::Message;
use prost::bytes::Bytes;

#[derive(Clone, Copy, Message)]
pub struct AvailableAbility {
  #[prost(int32, tag = "1")]
  pub ability_id: i32,
  #[prost(bool, tag = "2")]
  pub requires_point: bool,
}

#[derive(Clone, Message)]
pub struct ImageData {
  #[prost(int32, tag = "1")]
  pub bits_per_pixel: i32,
  #[prost(message, tag = "2")]
  pub size: Option<Size2DI>,
  #[prost(bytes, tag = "3")]
  pub data: Bytes,
}

#[derive(Clone, Copy, Message)]
pub struct PointI {
  #[prost(int32, tag = "1")]
  pub x: i32,
  #[prost(int32, tag = "2")]
  pub y: i32,
}

#[derive(Clone, Copy, Message)]
pub struct RectangleI {
  #[prost(message, tag = "1")]
  pub p0: Option<PointI>,
  #[prost(message, tag = "2")]
  pub p1: Option<PointI>,
}

#[derive(Clone, Copy, Message)]
pub struct Point2D {
  #[prost(float, tag = "1")]
  pub x: f32,
  #[prost(float, tag = "2")]
  pub y: f32,
}

#[derive(Clone, Copy, Message)]
pub struct Point {
  #[prost(float, tag = "1")]
  pub x: f32,
  #[prost(float, tag = "2")]
  pub y: f32,
  #[prost(float, tag = "3")]
  pub z: f32,
}

#[derive(Clone, Copy, Message)]
pub struct Size2DI {
  #[prost(int32, tag = "1")]
  pub x: i32,
  #[prost(int32, tag = "2")]
  pub y: i32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum Race {
  NoRace = 0,
  Terran = 1,
  Zerg = 2,
  Protoss = 3,
  Random = 4,
}
