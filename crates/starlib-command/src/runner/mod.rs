pub mod singleplayer;

mod auto {
  use std::collections::HashSet;

  use starlib_types::UnitIter;
  use starlib_types::UnitRef;
  use starlib_types::UnitTag;
  use starlib_types::UnitTypeId;

  use crate::agent::Agent;
  use crate::agent::AgentState;
  use crate::error::CommandError;

  pub fn do_auto_construction<A>(state: &AgentState<A>) -> Result<(), CommandError>
  where
    A: Agent + ?Sized,
  {
    println!("[auto]: Townhall (Searching)");

    let Some(townhall) = state.units().owned().townhalls().next() else {
      return Err(CommandError::msg("cannot find townhall"));
    };

    println!("[auto]: Townhall (Found)");

    let (trainee, trainer): (UnitTypeId, UnitRef<'_>) = match townhall.kind() {
      UnitTypeId::CommandCenter => (UnitTypeId::SCV, townhall),
      UnitTypeId::Nexus => (UnitTypeId::Probe, townhall),
      UnitTypeId::Hatchery => {
        println!("[auto]: Larva (Searching)");

        let Some(larva) = state.units().owned().larva().next() else {
          return Err(CommandError::msg("cannot find larva"));
        };

        println!("[auto]: Larva (Found)");

        (UnitTypeId::Drone, larva)
      }
      _ => {
        unreachable!()
      }
    };

    println!("[auto]: {trainee} (Training)");

    if trainer.train(trainee, false) {
      println!("[auto]: {trainee} (Trained)");

      Ok(())
    } else {
      Err(CommandError::msg("failed to train worker"))
    }
  }

  pub fn do_auto_distribution<A>(state: &AgentState<A>) -> Result<(), CommandError>
  where
    A: Agent + ?Sized,
  {
    const CAP_MINERALS: usize = 8;
    const CAP_WORKERS: usize = 12;

    println!("[auto]: Townhall (Searching)");

    let Some(townhall) = state.units().owned().townhalls().next() else {
      return Err(CommandError::msg("cannot find townhall"));
    };

    println!("[auto]: Townhall (Found)");

    let mut mineral: Vec<UnitRef<'_>> = Vec::with_capacity(CAP_MINERALS);
    let mut workers: Vec<UnitRef<'_>> = Vec::with_capacity(CAP_WORKERS);

    let mut assigned: HashSet<UnitTag> = HashSet::with_capacity(CAP_MINERALS);

    mineral.extend(state.units().minerals().by_range(townhall.point_2d(), 10.));
    workers.extend(state.units().owned().workers());

    println!("[auto]: Total Minerals = {}", mineral.len());
    println!("[auto]: Total Workers  = {}", workers.len());

    if mineral.is_empty() {
      return Err(CommandError::msg("cannot find minerals"));
    }

    if workers.is_empty() {
      return Err(CommandError::msg("cannot find workers"));
    }

    println!("[auto]: Priority Distribution ({})", mineral.len());

    for field in mineral.iter() {
      let worker: Option<UnitRef<'_>> = workers
        .iter()
        .filter(|unit| !assigned.contains(&unit.tag()))
        .copied()
        .closest(field.point_2d());

      if let Some(worker) = worker {
        println!("[auto]: Gather({} -> {})", worker.tag(), field.tag());

        worker.harvest_gather(field.tag(), false);
        assigned.insert(worker.tag());
      } else {
        break;
      }
    }

    let leftover: usize = workers.len() - assigned.len();

    println!("[auto]: Leftover Distribution ({})", leftover);

    for worker in workers {
      if assigned.contains(&worker.tag()) {
        continue;
      }

      let Some(field) = mineral.iter().copied().closest(worker.point_2d()) else {
        unreachable!()
      };

      println!("[auto]: Gather({} -> {})", worker.tag(), field.tag());

      worker.harvest_gather(field.tag(), false);
    }

    println!("[auto]: Workers Distributed");

    Ok(())
  }
}
