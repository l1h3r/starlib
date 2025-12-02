use crate::core::Float;
use crate::core::PlayerTag;
use crate::core::Point3DF;
use crate::core::PointF;
use crate::core::UnitTag;
use crate::stable::UnitTypeId;

define_bridge_message! {
  #[bridge(DebugCommand)]
  pub struct DebugCommand {
    command: DebugCommandVariant ref = proto::command,
  }
}

define_bridge_oneof! {
  #[bridge(DebugCommandVariant)]
  pub enum DebugCommandVariant {
    None,
    Draw(DebugDraw) = proto::Draw,
    GameState(DebugGameState) = proto::GameState,
    CreateUnit(DebugCreateUnit) = proto::CreateUnit,
    KillUnit(DebugKillUnit) = proto::KillUnit,
    TestProcess(DebugTestProcess) = proto::TestProcess,
    Score(DebugSetScore) = proto::Score,
    EndGame(DebugEndGame) = proto::EndGame,
    UnitValue(DebugSetUnitValue) = proto::UnitValue,
  }
}

define_bridge_message! {
  #[bridge(DebugDraw)]
  pub struct DebugDraw {
    text: DebugText vec = proto::text,
    lines: DebugLine vec = proto::lines,
    boxes: DebugBox vec = proto::boxes,
    spheres: DebugSphere vec = proto::spheres,
  }
}

define_bridge_message! {
  #[bridge(Line)]
  pub struct Line {
    p0: Point3DF = proto::p0,
    p1: Point3DF = proto::p1,
  }
}

define_bridge_message! {
  #[bridge(Color)]
  pub struct Color {
    r: u32 = proto::r,
    g: u32 = proto::g,
    b: u32 = proto::b,
  }
}

define_bridge_message! {
  #[bridge(DebugText)]
  pub struct DebugText {
    color: Color ref = proto::color,
    text: String = proto::text,
    virtual_point: Point3DF = proto::virtual_pos,
    world_point: Point3DF = proto::world_pos,
    size: u32 = proto::size,
  }
}

define_bridge_message! {
  #[bridge(DebugLine)]
  pub struct DebugLine {
    color: Color ref = proto::color,
    line: Line ref = proto::line,
  }
}

define_bridge_message! {
  #[bridge(DebugBox)]
  pub struct DebugBox {
    color: Color ref = proto::color,
    min: Point3DF = proto::min,
    max: Point3DF = proto::max,
  }
}

define_bridge_message! {
  #[bridge(DebugSphere)]
  pub struct DebugSphere {
    color: Color ref = proto::color,
    point: Point3DF = proto::p,
    radius: Float = proto::r,
  }
}

define_bridge_enum! {
  #[bridge(DebugGameState, NULL = ShowMap)]
  pub enum DebugGameState {
    ShowMap,
    ControlEnemy,
    Food,
    Free,
    AllResources,
    God,
    Minerals,
    Gas,
    Cooldown,
    TechTree,
    Upgrade,
    FastBuild,
  }
}

define_bridge_message! {
  #[bridge(DebugCreateUnit)]
  pub struct DebugCreateUnit {
    unit_type: UnitTypeId = proto::unit_type,
    owner: PlayerTag = proto::owner,
    point: PointF = proto::pos,
    quantity: u32 = proto::quantity,
  }
}

define_bridge_message! {
  #[bridge(DebugKillUnit)]
  pub struct DebugKillUnit {
    tag: UnitTag vec = proto::tag,
  }
}

define_bridge_enum! {
  #[bridge(DebugTestProcessTest, NULL = Hang)]
  pub enum TestProcessType {
    Hang,
    Crash,
    Exit,
  }
}

define_bridge_message! {
  #[bridge(DebugTestProcess)]
  pub struct DebugTestProcess {
    test: TestProcessType = proto::test,
    delay_ms: i32 = proto::delay_ms,
  }
}

define_bridge_message! {
  #[bridge(DebugSetScore)]
  pub struct DebugSetScore {
    score: Float = proto::score,
  }
}

define_bridge_enum! {
  #[bridge(EndResult, NULL = Surrender)]
  pub enum EndResult {
    Surrender,
    DeclareVictory,
  }
}

define_bridge_message! {
  #[bridge(DebugEndGame)]
  pub struct DebugEndGame {
    end_result: EndResult = proto::end_result,
  }
}

define_bridge_enum! {
  #[bridge(UnitValue, NULL = Energy)]
  pub enum UnitValue {
    Energy,
    Life,
    Shields,
  }
}

define_bridge_message! {
  #[bridge(DebugSetUnitValue)]
  pub struct DebugSetUnitValue {
    unit_value: UnitValue = proto::unit_value,
    value: Float = proto::value,
    unit_tag: UnitTag = proto::unit_tag,
  }
}
