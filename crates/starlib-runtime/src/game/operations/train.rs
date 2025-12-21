use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use starlib_types::UnitIter;
use starlib_types::UnitTypeId;

use crate::core::ControlFlow;
use crate::core::Operation;
use crate::exec::Argument;
use crate::exec::Mut;
use crate::exec::Ref;
use crate::exec::StateRef;
use crate::game::CostTracker;
use crate::game::UnitTracker;

#[derive(Clone, Copy)]
pub struct Train(pub UnitTypeId, pub usize);

impl Train {
  pub const PROTOSS: &'static [UnitTypeId] = &[];

  pub const TERRAN: &'static [UnitTypeId] = &[];

  pub const ZERG: &'static [UnitTypeId] = &[
    // -------------------------------------------------------------------------
    // Hatchery Units
    // -------------------------------------------------------------------------
    UnitTypeId::Queen,
    // -------------------------------------------------------------------------
    // Larva Units
    // -------------------------------------------------------------------------
    UnitTypeId::Corruptor,
    UnitTypeId::Drone,
    UnitTypeId::Hydralisk,
    UnitTypeId::Infestor,
    UnitTypeId::Mutalisk,
    UnitTypeId::Overlord,
    UnitTypeId::Roach,
    UnitTypeId::SwarmHostMP,
    UnitTypeId::Ultralisk,
    UnitTypeId::Viper,
    UnitTypeId::Zergling,
    // -------------------------------------------------------------------------
    // Morph Units
    // -------------------------------------------------------------------------
    UnitTypeId::Baneling,
    UnitTypeId::BroodLord,
    UnitTypeId::LurkerMP,
    UnitTypeId::OverlordTransport,
    UnitTypeId::Overseer,
    UnitTypeId::Ravager,
  ];
}

impl Debug for Train {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "Train({}, {})", self.0, self.1)
  }
}

impl Operation for Train {
  type Argv = (
    StateRef<'static>,
    Mut<'static, CostTracker>,
    Ref<'static, UnitTracker>,
  );

  fn setup(&self) -> Result<(), String> {
    let mut valid: bool = false;

    valid |= Self::PROTOSS.contains(&self.0);
    valid |= Self::TERRAN.contains(&self.0);
    valid |= Self::ZERG.contains(&self.0);

    if valid {
      Ok(())
    } else {
      Err(format!("invalid unit: {}", self.0))
    }
  }

  #[inline]
  fn run(
    &mut self,
    (state, mut cost_tracker, unit_tracker): <Self::Argv as Argument>::Item<'_>,
  ) -> ControlFlow {
    let Some(udata) = state.try_unit(self.0) else {
      unreachable!()
    };

    let mut count: usize = 0;

    count += unit_tracker.count_current(self.0);
    count += unit_tracker.count_pending(self.0);

    if count >= self.1 {
      return ControlFlow::Next;
    }

    if !cost_tracker.can_afford(&udata) {
      return ControlFlow::Stop;
    }

    'train: for builder in state.units().owned().builder_of(self.0) {
      if builder.is_flying() || builder.build_pending() {
        continue;
      }

      if builder.is_busy() || builder.has_action() {
        continue;
      }

      if builder.train(self.0, false) {
        cost_tracker.reserve(&udata);

        println!(
          "[Train]: {trainee} from {trainer} at {point}",
          trainee = self.0,
          trainer = builder.kind(),
          point = builder.point_2d(),
        );

        break 'train;
      }
    }

    ControlFlow::Stop
  }
}

#[macro_export]
macro_rules! Train {
  ($utype:expr $(,)?) => {
    $crate::game::Train($utype, usize::MAX)
  };
  ($utype:expr, $count:expr $(,)?) => {
    $crate::game::Train($utype, $count)
  };
}
