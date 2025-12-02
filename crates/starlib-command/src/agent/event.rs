use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use starlib_types::ActionResult;
use starlib_types::Float;
use starlib_types::UnitRef;
use starlib_types::UnitTag;
use starlib_types::UnitTypeId;
use starlib_types::UpgradeId;

#[derive(Clone, Copy, Debug)]
pub enum Event<'a> {
  // Action Error
  ActionError(ActionResult),
  // Upgrade Info
  UpgradeComplete(UpgradeId),
  // Unit Status
  UnitKilled(UnitTag),
  // Unit Info
  UnitCreated(UnitRef<'a>),
  UnitMorphed(UnitRef<'a>, UnitTypeId),
  UnitDamaged(UnitRef<'a>, Float, Float),
  // Enemy Vision
  EnterVision(UnitRef<'a>),
  LeaveVision(UnitTag),
  // Building Construction
  BuildingPending(UnitRef<'a>),
  BuildingComplete(UnitRef<'a>),
}

impl Display for Event<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::ActionError(error) => {
        write!(f, "ActionError({error})")
      }
      Self::UpgradeComplete(id) => {
        write!(f, "UpgradeComplete({id})")
      }
      Self::UnitKilled(tag) => {
        write!(f, "UnitKilled({tag})")
      }
      Self::UnitCreated(unit) => {
        write!(f, "UnitCreated({} - {})", unit.tag(), unit.kind())
      }
      Self::UnitMorphed(unit, from) => {
        write!(f, "UnitMorphed({} - {from} => {})", unit.tag(), unit.kind())
      }
      Self::UnitDamaged(unit, health, shield) if shield.epsilon_eq(0.0) => {
        write!(f, "UnitDamaged({} - {health})", unit.tag())
      }
      Self::UnitDamaged(unit, health, shield) => {
        write!(f, "UnitDamaged({} - {health} | {shield})", unit.tag())
      }
      Self::EnterVision(unit) => {
        write!(f, "EnterVision({} - {})", unit.tag(), unit.kind())
      }
      Self::LeaveVision(unit) => {
        write!(f, "LeaveVision({unit})")
      }
      Self::BuildingPending(unit) => {
        write!(f, "BuildingPending({} - {})", unit.tag(), unit.kind())
      }
      Self::BuildingComplete(unit) => {
        write!(f, "BuildingComplete({} - {})", unit.tag(), unit.kind())
      }
    }
  }
}
