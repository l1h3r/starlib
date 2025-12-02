use prost::Enumeration;
use prost::Message;

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ScoreType {
  NULL = 0,
  Curriculum = 1,
  Melee = 2,
}

#[derive(Clone, Copy, Message)]
pub struct Score {
  #[prost(enumeration = "ScoreType", tag = "6")]
  pub score_type: i32,
  #[prost(int32, tag = "7")]
  pub score: i32,
  #[prost(message, tag = "8")]
  pub score_details: Option<ScoreDetails>,
}

#[derive(Clone, Copy, Message)]
pub struct CategoryScoreDetails {
  #[prost(float, tag = "1")]
  pub none: f32,
  #[prost(float, tag = "2")]
  pub army: f32,
  #[prost(float, tag = "3")]
  pub economy: f32,
  #[prost(float, tag = "4")]
  pub technology: f32,
  #[prost(float, tag = "5")]
  pub upgrade: f32,
}

#[derive(Clone, Copy, Message)]
pub struct VitalScoreDetails {
  #[prost(float, tag = "1")]
  pub life: f32,
  #[prost(float, tag = "2")]
  pub shields: f32,
  #[prost(float, tag = "3")]
  pub energy: f32,
}

#[derive(Clone, Copy, Message)]
pub struct ScoreDetails {
  #[prost(float, tag = "1")]
  pub idle_production_time: f32,
  #[prost(float, tag = "2")]
  pub idle_worker_time: f32,

  #[prost(float, tag = "3")]
  pub total_value_units: f32,
  #[prost(float, tag = "4")]
  pub total_value_structures: f32,

  #[prost(float, tag = "5")]
  pub killed_value_units: f32,
  #[prost(float, tag = "6")]
  pub killed_value_structures: f32,

  #[prost(float, tag = "7")]
  pub collected_minerals: f32,
  #[prost(float, tag = "8")]
  pub collected_vespene: f32,

  #[prost(float, tag = "9")]
  pub collection_rate_minerals: f32,
  #[prost(float, tag = "10")]
  pub collection_rate_vespene: f32,

  #[prost(float, tag = "11")]
  pub spent_minerals: f32,
  #[prost(float, tag = "12")]
  pub spent_vespene: f32,

  #[prost(message, tag = "13")]
  pub food_used: Option<CategoryScoreDetails>,

  #[prost(message, tag = "14")]
  pub killed_minerals: Option<CategoryScoreDetails>,
  #[prost(message, tag = "15")]
  pub killed_vespene: Option<CategoryScoreDetails>,

  #[prost(message, tag = "16")]
  pub lost_minerals: Option<CategoryScoreDetails>,
  #[prost(message, tag = "17")]
  pub lost_vespene: Option<CategoryScoreDetails>,

  #[prost(message, tag = "18")]
  pub friendly_fire_minerals: Option<CategoryScoreDetails>,
  #[prost(message, tag = "19")]
  pub friendly_fire_vespene: Option<CategoryScoreDetails>,

  #[prost(message, tag = "20")]
  pub used_minerals: Option<CategoryScoreDetails>,
  #[prost(message, tag = "21")]
  pub used_vespene: Option<CategoryScoreDetails>,

  #[prost(message, tag = "22")]
  pub total_used_minerals: Option<CategoryScoreDetails>,
  #[prost(message, tag = "23")]
  pub total_used_vespene: Option<CategoryScoreDetails>,

  #[prost(message, tag = "24")]
  pub total_damage_dealt: Option<VitalScoreDetails>,
  #[prost(message, tag = "25")]
  pub total_damage_taken: Option<VitalScoreDetails>,
  #[prost(message, tag = "26")]
  pub total_healed: Option<VitalScoreDetails>,

  #[prost(float, tag = "27")]
  pub current_apm: f32,
  #[prost(float, tag = "28")]
  pub current_effective_apm: f32,
}
