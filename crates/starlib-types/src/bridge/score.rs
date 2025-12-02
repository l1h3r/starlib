use crate::core::Float;

define_bridge_enum! {
  #[bridge(ScoreType, NULL = Melee)]
  pub enum ScoreType {
    Curriculum,
    Melee,
  }
}

define_bridge_message! {
  #[bridge(Score)]
  pub struct Score {
    score_type: ScoreType = proto::score_type,
    score_data: i32 = proto::score,
    score_info: ScoreInfo ref = proto::score_details,
  }
}

define_bridge_message! {
  #[bridge(CategoryScoreDetails)]
  pub struct CategoryScore {
    none: Float = proto::none,
    army: Float = proto::army,
    economy: Float = proto::economy,
    technology: Float = proto::technology,
    upgrade: Float = proto::upgrade,
  }
}

define_bridge_message! {
  #[bridge(VitalScoreDetails)]
  pub struct VitalScore {
    life: Float = proto::life,
    shields: Float = proto::shields,
    energy: Float = proto::energy,
  }
}

define_bridge_message! {
  #[bridge(ScoreDetails)]
  pub struct ScoreInfo {
    idle_production_time: Float = proto::idle_production_time,
    idle_worker_time: Float = proto::idle_worker_time,

    total_value_units: Float = proto::total_value_units,
    total_value_structures: Float = proto::total_value_structures,

    killed_value_units: Float = proto::killed_value_units,
    killed_value_structures: Float = proto::killed_value_structures,

    collected_minerals: Float = proto::collected_minerals,
    collected_vespene: Float = proto::collected_vespene,

    collection_rate_minerals: Float = proto::collection_rate_minerals,
    collection_rate_vespene: Float = proto::collection_rate_vespene,

    spent_minerals: Float = proto::spent_minerals,
    spent_vespene: Float = proto::spent_vespene,

    food_used: CategoryScore ref = proto::food_used,

    killed_minerals: CategoryScore ref = proto::killed_minerals,
    killed_vespene: CategoryScore ref = proto::killed_vespene,

    lost_minerals: CategoryScore ref = proto::lost_minerals,
    lost_vespene: CategoryScore ref = proto::lost_vespene,

    friendly_fire_minerals: CategoryScore ref = proto::friendly_fire_minerals,
    friendly_fire_vespene: CategoryScore ref = proto::friendly_fire_vespene,

    used_minerals: CategoryScore ref = proto::used_minerals,
    used_vespene: CategoryScore ref = proto::used_vespene,

    total_used_minerals: CategoryScore ref = proto::total_used_minerals,
    total_used_vespene: CategoryScore ref = proto::total_used_vespene,

    total_damage_dealt: VitalScore ref = proto::total_damage_dealt,
    total_damage_taken: VitalScore ref = proto::total_damage_taken,

    total_healed: VitalScore ref = proto::total_healed,

    current_apm: Float = proto::current_apm,
    current_effective_apm: Float = proto::current_effective_apm,
  }
}
