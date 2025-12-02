use prost::Enumeration;
use prost::Message;
use prost::Oneof;

use crate::support::Point;
use crate::support::Point2D;

#[derive(Clone, Message)]
pub struct DebugCommand {
  #[prost(oneof = "DebugCommandVariant", tags = "1,2,3,4,5,6,7,8")]
  pub command: Option<DebugCommandVariant>,
}

#[derive(Clone, Oneof)]
pub enum DebugCommandVariant {
  #[prost(message, tag = "1")]
  Draw(DebugDraw),
  #[prost(enumeration = "DebugGameState", tag = "2")]
  GameState(i32),
  #[prost(message, tag = "3")]
  CreateUnit(DebugCreateUnit),
  #[prost(message, tag = "4")]
  KillUnit(DebugKillUnit),
  #[prost(message, tag = "5")]
  TestProcess(DebugTestProcess),
  #[prost(message, tag = "6")]
  Score(DebugSetScore),
  #[prost(message, tag = "7")]
  EndGame(DebugEndGame),
  #[prost(message, tag = "8")]
  UnitValue(DebugSetUnitValue),
}

#[derive(Clone, Message)]
pub struct DebugDraw {
  #[prost(repeated, message, tag = "1")]
  pub text: Vec<DebugText>,
  #[prost(repeated, message, tag = "2")]
  pub lines: Vec<DebugLine>,
  #[prost(repeated, message, tag = "3")]
  pub boxes: Vec<DebugBox>,
  #[prost(repeated, message, tag = "4")]
  pub spheres: Vec<DebugSphere>,
}

#[derive(Clone, Copy, Message)]
pub struct Line {
  #[prost(message, tag = "1")]
  pub p0: Option<Point>,
  #[prost(message, tag = "2")]
  pub p1: Option<Point>,
}

#[derive(Clone, Copy, Message)]
pub struct Color {
  #[prost(uint32, tag = "1")]
  pub r: u32,
  #[prost(uint32, tag = "2")]
  pub g: u32,
  #[prost(uint32, tag = "3")]
  pub b: u32,
}

#[derive(Clone, Message)]
pub struct DebugText {
  #[prost(message, tag = "1")]
  pub color: Option<Color>,
  #[prost(string, tag = "2")]
  pub text: String,
  #[prost(message, tag = "3")]
  pub virtual_pos: Option<Point>,
  #[prost(message, tag = "4")]
  pub world_pos: Option<Point>,
  #[prost(uint32, tag = "5")]
  pub size: u32,
}

#[derive(Clone, Copy, Message)]
pub struct DebugLine {
  #[prost(message, tag = "1")]
  pub color: Option<Color>,
  #[prost(message, tag = "2")]
  pub line: Option<Line>,
}

#[derive(Clone, Copy, Message)]
pub struct DebugBox {
  #[prost(message, tag = "1")]
  pub color: Option<Color>,
  #[prost(message, tag = "2")]
  pub min: Option<Point>,
  #[prost(message, tag = "3")]
  pub max: Option<Point>,
}

#[derive(Clone, Copy, Message)]
pub struct DebugSphere {
  #[prost(message, tag = "1")]
  pub color: Option<Color>,
  #[prost(message, tag = "2")]
  pub p: Option<Point>,
  #[prost(float, tag = "3")]
  pub r: f32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum DebugGameState {
  NULL = 0,
  ShowMap = 1,
  ControlEnemy = 2,
  Food = 3,
  Free = 4,
  AllResources = 5,
  God = 6,
  Minerals = 7,
  Gas = 8,
  Cooldown = 9,
  TechTree = 10,
  Upgrade = 11,
  FastBuild = 12,
}

#[derive(Clone, Copy, Message)]
pub struct DebugCreateUnit {
  #[prost(uint32, tag = "1")]
  pub unit_type: u32,
  #[prost(int32, tag = "2")]
  pub owner: i32,
  #[prost(message, tag = "3")]
  pub pos: Option<Point2D>,
  #[prost(uint32, tag = "4")]
  pub quantity: u32,
}

#[derive(Clone, Message)]
pub struct DebugKillUnit {
  #[prost(repeated, uint64, tag = "1")]
  pub tag: Vec<u64>,
}

#[derive(Clone, Copy, Message)]
pub struct DebugTestProcess {
  #[prost(enumeration = "DebugTestProcessTest", tag = "1")]
  pub test: i32,
  #[prost(int32, tag = "2")]
  pub delay_ms: i32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum DebugTestProcessTest {
  NULL = 0,
  Hang = 1,
  Crash = 2,
  Exit = 3,
}

#[derive(Clone, Copy, Message)]
pub struct DebugSetScore {
  #[prost(float, tag = "1")]
  pub score: f32,
}

#[derive(Clone, Copy, Message)]
pub struct DebugEndGame {
  #[prost(enumeration = "EndResult", tag = "1")]
  pub end_result: i32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum EndResult {
  NULL = 0,
  Surrender = 1,
  DeclareVictory = 2,
}

#[derive(Clone, Copy, Message)]
pub struct DebugSetUnitValue {
  #[prost(enumeration = "UnitValue", tag = "1")]
  pub unit_value: i32,
  #[prost(float, tag = "2")]
  pub value: f32,
  #[prost(uint64, tag = "3")]
  pub unit_tag: u64,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum UnitValue {
  NULL = 0,
  Energy = 1,
  Life = 2,
  Shields = 3,
}
