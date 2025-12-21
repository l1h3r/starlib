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
    none: f32 = proto::none,
    army: f32 = proto::army,
    economy: f32 = proto::economy,
    technology: f32 = proto::technology,
    upgrade: f32 = proto::upgrade,
  }
}

define_bridge_message! {
  #[bridge(VitalScoreDetails)]
  pub struct VitalScore {
    life: f32 = proto::life,
    shields: f32 = proto::shields,
    energy: f32 = proto::energy,
  }
}

define_bridge_message! {
  #[bridge(ScoreDetails)]
  pub struct ScoreInfo {
    idle_production_time: f32 = proto::idle_production_time,
    idle_worker_time: f32 = proto::idle_worker_time,

    total_value_units: f32 = proto::total_value_units,
    total_value_structures: f32 = proto::total_value_structures,

    killed_value_units: f32 = proto::killed_value_units,
    killed_value_structures: f32 = proto::killed_value_structures,

    collected_minerals: f32 = proto::collected_minerals,
    collected_vespene: f32 = proto::collected_vespene,

    collection_rate_minerals: f32 = proto::collection_rate_minerals,
    collection_rate_vespene: f32 = proto::collection_rate_vespene,

    spent_minerals: f32 = proto::spent_minerals,
    spent_vespene: f32 = proto::spent_vespene,

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

    current_apm: f32 = proto::current_apm,
    current_effective_apm: f32 = proto::current_effective_apm,
  }
}
