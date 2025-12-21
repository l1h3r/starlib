use crate::core::PlayerTag;
use crate::stable::AbilityId;
use crate::stable::BuffId;
use crate::stable::UnitTypeId;

// -----------------------------------------------------------------------------
// Observation
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(ObservationUI)]
  pub struct ObservationUI {
    groups: ControlGroup vec = proto::groups,
    panel: ObservationUIPanel ref = proto::panel,
  }
}

define_bridge_oneof! {
  #[bridge(ObservationUIPanel)]
  pub enum ObservationUIPanel {
    None,
    Single(SinglePanel) = proto::Single,
    Multi(MultiPanel) = proto::Multi,
    Cargo(CargoPanel) = proto::Cargo,
    Production(ProductionPanel) = proto::Production,
  }
}

define_bridge_message! {
  #[bridge(ControlGroup)]
  pub struct ControlGroup {
    control_group_index: u32 = proto::control_group_index,
    leader_unit_type: UnitTypeId = proto::leader_unit_type,
    count: u32 = proto::count,
  }
}

define_bridge_message! {
  #[bridge(UnitInfo)]
  pub struct UnitInfo {
    unit_type: UnitTypeId = proto::unit_type,
    player_relative: PlayerTag = proto::player_relative,
    health: i32 = proto::health,
    shields: i32 = proto::shields,
    energy: i32 = proto::energy,
    transport_slots_taken: i32 = proto::transport_slots_taken,
    build_progress: f32 = proto::build_progress,
    add_on: Self = proto::add_on,
    max_health: i32 = proto::max_health,
    max_shields: i32 = proto::max_shields,
    max_energy: i32 = proto::max_energy,
  }
}

define_bridge_message! {
  #[bridge(SinglePanel)]
  pub struct SinglePanel {
    unit: UnitInfo ref = proto::unit,
    attack_upgrade_level: i32 = proto::attack_upgrade_level,
    armor_upgrade_level: i32 = proto::armor_upgrade_level,
    shield_upgrade_level: i32 = proto::shield_upgrade_level,
    buffs: BuffId vec = proto::buffs,
  }
}

define_bridge_message! {
  #[bridge(MultiPanel)]
  pub struct MultiPanel {
    units: UnitInfo vec = proto::units,
  }
}

define_bridge_message! {
  #[bridge(CargoPanel)]
  pub struct CargoPanel {
    unit: UnitInfo ref = proto::unit,
    passengers: UnitInfo vec = proto::passengers,
    build_progress: i32 = proto::build_progress,
  }
}

define_bridge_message! {
  #[bridge(BuildItem)]
  pub struct BuildItem {
    ability_id: AbilityId = proto::ability_id,
    build_progress: f32 = proto::build_progress,
  }
}

define_bridge_message! {
  #[bridge(ProductionPanel)]
  pub struct ProductionPanel {
    unit: UnitInfo ref = proto::unit,
    build_queue: UnitInfo vec = proto::build_queue,
    production_queue: BuildItem vec = proto::production_queue,
  }
}

// -----------------------------------------------------------------------------
// Action
// -----------------------------------------------------------------------------

define_bridge_message! {
  #[bridge(ActionUI)]
  pub struct ActionUI {
    action: ActionUIVariant ref = proto::action,
  }
}

define_bridge_oneof! {
  #[bridge(ActionUIVariant)]
  pub enum ActionUIVariant {
    None,
    ControlGroup(ActionControlGroup) = proto::ControlGroup,
    SelectArmy(ActionSelectArmy) = proto::SelectArmy,
    SelectWarpGates(ActionSelectWarpGates) = proto::SelectWarpGates,
    SelectLarva(ActionSelectLarva) = proto::SelectLarva,
    SelectIdleWorker(ActionSelectIdleWorker) = proto::SelectIdleWorker,
    MultiPanel(ActionMultiPanel) = proto::MultiPanel,
    CargoPanel(ActionCargoPanelUnload) = proto::CargoPanel,
    ProductionPanel(ActionProductionPanelRemoveFromQueue) = proto::ProductionPanel,
    ToggleAutocast(ActionToggleAutocast) = proto::ToggleAutocast,
  }
}

define_bridge_enum! {
  #[bridge(ControlGroupAction, NULL = Recall)]
  pub enum ControlGroupAction {
    Recall,
    Set,
    Append,
    SetAndSteal,
    AppendAndSteal,
  }
}

define_bridge_message! {
  #[bridge(ActionControlGroup)]
  pub struct ActionControlGroup {
    action: ControlGroupAction = proto::action,
    index: u32 = proto::control_group_index,
  }
}

define_bridge_message! {
  #[bridge(ActionSelectArmy)]
  pub struct ActionSelectArmy {
    selection_add: bool = proto::selection_add,
  }
}

define_bridge_message! {
  #[bridge(ActionSelectWarpGates)]
  pub struct ActionSelectWarpGates {
    selection_add: bool = proto::selection_add,
  }
}

define_bridge_message! {
  #[bridge(ActionSelectLarva)]
  pub struct ActionSelectLarva {}
}

define_bridge_enum! {
  #[bridge(ActionSelectIdleWorkerType, NULL = Set)]
  pub enum ActionSelectIdleWorkerType {
    Set,
    Add,
    All,
    AddAll,
  }
}

define_bridge_message! {
  #[bridge(ActionSelectIdleWorker)]
  pub struct ActionSelectIdleWorker {
    kind: ActionSelectIdleWorkerType = proto::kind,
  }
}

define_bridge_enum! {
  #[bridge(ActionMultiPanelType, NULL = SingleSelect)]
  pub enum ActionMultiPanelType {
    SingleSelect,
    DeselectUnit,
    SelectAllOfType,
    DeselectAllOfType,
  }
}

define_bridge_message! {
  #[bridge(ActionMultiPanel)]
  pub struct ActionMultiPanel {
    kind: ActionMultiPanelType = proto::kind,
    unit_index: i32 = proto::unit_index,
  }
}

define_bridge_message! {
  #[bridge(ActionCargoPanelUnload)]
  pub struct ActionCargoPanelUnload {
    unit_index: i32 = proto::unit_index,
  }
}

define_bridge_message! {
  #[bridge(ActionProductionPanelRemoveFromQueue)]
  pub struct ActionProductionPanelRemoveFromQueue {
    unit_index: i32 = proto::unit_index,
  }
}

define_bridge_message! {
  #[bridge(ActionToggleAutocast)]
  pub struct ActionToggleAutocast {
    ability_id: AbilityId = proto::ability_id,
  }
}
