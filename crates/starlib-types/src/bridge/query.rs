use crate::bridge::Ability;
use crate::bridge::ActionResult;
use crate::core::Float;
use crate::core::PointF;
use crate::core::UnitTag;
use crate::stable::AbilityId;
use crate::stable::UnitTypeId;

define_bridge_oneof! {
  #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
  #[bridge(RequestQueryPathingStart)]
  pub enum RequestQueryPathingStart {
    None,
    ByPos(PointF) = proto::StartPos,
    ByTag(UnitTag) = proto::UnitTag,
  }
}

define_bridge_message! {
  #[bridge(RequestQueryPathing)]
  pub struct RequestQueryPathing {
    start: RequestQueryPathingStart = proto::start,
    end: PointF = proto::end_pos,
  }
}

define_bridge_message! {
  #[bridge(ResponseQueryPathing)]
  pub struct ResponseQueryPathing {
    distance: Float = proto::distance,
  }
}

define_bridge_message! {
  #[bridge(RequestQueryAvailableAbilities)]
  pub struct RequestQueryAvailableAbilities {
    unit_tag: UnitTag = proto::unit_tag,
  }
}

define_bridge_message! {
  #[bridge(ResponseQueryAvailableAbilities)]
  pub struct ResponseQueryAvailableAbilities {
    abilities: Ability vec = proto::abilities,
    unit_kind: UnitTypeId = proto::unit_type_id,
    unit_tag: UnitTag = proto::unit_tag,
  }
}

define_bridge_message! {
  #[bridge(RequestQueryBuildingPlacement)]
  pub struct RequestQueryBuildingPlacement {
    ability_id: AbilityId = proto::ability_id,
    target: PointF = proto::target_pos,
    unit_tag: UnitTag = proto::placing_unit_tag,
  }
}

define_bridge_message! {
  #[bridge(ResponseQueryBuildingPlacement)]
  pub struct ResponseQueryBuildingPlacement {
    result: ActionResult = proto::result,
  }
}
