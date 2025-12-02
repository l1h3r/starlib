use prost::Enumeration;
use prost::Message;
use prost::Oneof;

// -----------------------------------------------------------------------------
// Observation
// -----------------------------------------------------------------------------

#[derive(Clone, Message)]
pub struct ObservationUI {
  #[prost(repeated, message, tag = "1")]
  pub groups: Vec<ControlGroup>,
  #[prost(oneof = "ObservationUIPanel", tags = "2,3,4,5")]
  pub panel: Option<ObservationUIPanel>,
}

#[derive(Clone, Oneof)]
pub enum ObservationUIPanel {
  #[prost(message, tag = "2")]
  Single(SinglePanel),
  #[prost(message, tag = "3")]
  Multi(MultiPanel),
  #[prost(message, tag = "4")]
  Cargo(CargoPanel),
  #[prost(message, tag = "5")]
  Production(ProductionPanel),
}

#[derive(Clone, Copy, Message)]
pub struct ControlGroup {
  #[prost(uint32, tag = "1")]
  pub control_group_index: u32,
  #[prost(uint32, tag = "2")]
  pub leader_unit_type: u32,
  #[prost(uint32, tag = "3")]
  pub count: u32,
}

#[derive(Clone, Message)]
pub struct UnitInfo {
  #[prost(uint32, tag = "1")]
  pub unit_type: u32,
  #[prost(uint32, tag = "2")]
  pub player_relative: u32,
  #[prost(int32, tag = "3")]
  pub health: i32,
  #[prost(int32, tag = "4")]
  pub shields: i32,
  #[prost(int32, tag = "5")]
  pub energy: i32,
  #[prost(int32, tag = "6")]
  pub transport_slots_taken: i32,
  #[prost(float, tag = "7")]
  pub build_progress: f32,
  #[prost(message, tag = "8")]
  pub add_on: Option<Box<UnitInfo>>,
  #[prost(int32, tag = "9")]
  pub max_health: i32,
  #[prost(int32, tag = "10")]
  pub max_shields: i32,
  #[prost(int32, tag = "11")]
  pub max_energy: i32,
}

#[derive(Clone, Message)]
pub struct SinglePanel {
  #[prost(message, tag = "1")]
  pub unit: Option<UnitInfo>,
  #[prost(int32, tag = "2")]
  pub attack_upgrade_level: i32,
  #[prost(int32, tag = "3")]
  pub armor_upgrade_level: i32,
  #[prost(int32, tag = "4")]
  pub shield_upgrade_level: i32,
  #[prost(repeated, int32, tag = "5")]
  pub buffs: Vec<i32>,
}

#[derive(Clone, Message)]
pub struct MultiPanel {
  #[prost(repeated, message, tag = "1")]
  pub units: Vec<UnitInfo>,
}

#[derive(Clone, Message)]
pub struct CargoPanel {
  #[prost(message, tag = "1")]
  pub unit: Option<UnitInfo>,
  #[prost(repeated, message, tag = "2")]
  pub passengers: Vec<UnitInfo>,
  #[prost(int32, tag = "3")]
  pub build_progress: i32,
}

#[derive(Clone, Copy, Message)]
pub struct BuildItem {
  #[prost(uint32, tag = "1")]
  pub ability_id: u32,
  #[prost(float, tag = "2")]
  pub build_progress: f32,
}

#[derive(Clone, Message)]
pub struct ProductionPanel {
  #[prost(message, tag = "1")]
  pub unit: Option<UnitInfo>,
  #[prost(repeated, message, tag = "2")]
  pub build_queue: Vec<UnitInfo>,
  #[prost(repeated, message, tag = "3")]
  pub production_queue: Vec<BuildItem>,
}

// -----------------------------------------------------------------------------
// Action
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Message)]
pub struct ActionUI {
  #[prost(oneof = "ActionUIVariant", tags = "1,2,3,4,5,6,7,8,9")]
  pub action: Option<ActionUIVariant>,
}

#[derive(Clone, Copy, Oneof)]
pub enum ActionUIVariant {
  #[prost(message, tag = "1")]
  ControlGroup(ActionControlGroup),
  #[prost(message, tag = "2")]
  SelectArmy(ActionSelectArmy),
  #[prost(message, tag = "3")]
  SelectWarpGates(ActionSelectWarpGates),
  #[prost(message, tag = "4")]
  SelectLarva(ActionSelectLarva),
  #[prost(message, tag = "5")]
  SelectIdleWorker(ActionSelectIdleWorker),
  #[prost(message, tag = "6")]
  MultiPanel(ActionMultiPanel),
  #[prost(message, tag = "7")]
  CargoPanel(ActionCargoPanelUnload),
  #[prost(message, tag = "8")]
  ProductionPanel(ActionProductionPanelRemoveFromQueue),
  #[prost(message, tag = "9")]
  ToggleAutocast(ActionToggleAutocast),
}

#[derive(Clone, Copy, Message)]
pub struct ActionControlGroup {
  #[prost(enumeration = "ControlGroupAction", tag = "1")]
  pub action: i32,
  #[prost(uint32, tag = "2")]
  pub control_group_index: u32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ControlGroupAction {
  NULL = 0,
  Recall = 1,
  Set = 2,
  Append = 3,
  SetAndSteal = 4,
  AppendAndSteal = 5,
}

#[derive(Clone, Copy, Message)]
pub struct ActionSelectArmy {
  #[prost(bool, tag = "1")]
  pub selection_add: bool,
}

#[derive(Clone, Copy, Message)]
pub struct ActionSelectWarpGates {
  #[prost(bool, tag = "1")]
  pub selection_add: bool,
}

#[derive(Clone, Copy, Message)]
pub struct ActionSelectLarva {}

#[derive(Clone, Copy, Message)]
pub struct ActionSelectIdleWorker {
  #[prost(enumeration = "ActionSelectIdleWorkerType", tag = "1")]
  pub kind: i32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ActionSelectIdleWorkerType {
  NULL = 0,
  Set = 1,
  Add = 2,
  All = 3,
  AddAll = 4,
}

#[derive(Clone, Copy, Message)]
pub struct ActionMultiPanel {
  #[prost(enumeration = "ActionMultiPanelType", tag = "1")]
  pub kind: i32,
  #[prost(int32, tag = "2")]
  pub unit_index: i32,
}

#[derive(Clone, Copy, Debug, Enumeration)]
#[repr(i32)]
pub enum ActionMultiPanelType {
  NULL = 0,
  SingleSelect = 1,
  DeselectUnit = 2,
  SelectAllOfType = 3,
  DeselectAllOfType = 4,
}

#[derive(Clone, Copy, Message)]
pub struct ActionCargoPanelUnload {
  #[prost(int32, tag = "1")]
  pub unit_index: i32,
}

#[derive(Clone, Copy, Message)]
pub struct ActionProductionPanelRemoveFromQueue {
  #[prost(int32, tag = "1")]
  pub unit_index: i32,
}

#[derive(Clone, Copy, Message)]
pub struct ActionToggleAutocast {
  #[prost(int32, tag = "1")]
  pub ability_id: i32,
}
