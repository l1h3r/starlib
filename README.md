# StarLib

A Rust library for writing StarCraft II bots.

StarLib starts the game, connects to it through Blizzard's [SC2 API][s2client-proto] and drives a match one step at a time. Your bot implements a small trait and gets a typed view of the game: units, abilities, upgrades and the map. It sends commands back through the same types.

> **Status:** early and unpublished. The client, launcher and game types work; `examples/rpc.rs` uses them.
> The runtime (`starlib-runtime`) is a work in progress, and the API will change. There are no tests yet.

## Example

```rust
use starlib::command::Agent;
use starlib::command::AgentConfig;
use starlib::command::AgentState;
use starlib::command::Computer;
use starlib::command::Map;
use starlib::command::Sc2Api;
use starlib::types::Race;
use std::net::Ipv4Addr;

struct Bot;

impl Agent for Bot {
  type Error = std::io::Error;

  fn config(&self) -> AgentConfig {
    AgentConfig::new(Race::Terran).name("Bot").auto_distribution(true)
  }

  fn on_step(&mut self, state: &AgentState<Self>, step: u32) -> Result<(), Self::Error> {
    if step % 100 == 0 {
      println!("step {step}: {} workers", state.units().owned().workers().count());
    }
    Ok(())
  }
}

fn main() {
  let api = Sc2Api::connect(Ipv4Addr::LOCALHOST, 5000).expect("game is running");
  let map = Map::path("Maps/AcropolisLE.SC2Map");
  let opponent = Computer::new("CPU", Race::Zerg);

  let result = Bot.run_singleplayer(&api, map, &[opponent]);
  println!("{result:?}");
}
```

[`examples/rpc.rs`](examples/rpc.rs) shows the lower level: it starts the game, pings it, lists the installed maps and quits.

## Requirements

- StarCraft II installed. The free Starter Edition works. Tested on macOS; Windows should work but hasn't been tested.
- Rust 1.91 or later (edition 2024).
- To regenerate the game ID types, the `stableid.json` file the game writes after its first launch.

## How it's built

The workspace has seven crates. `starlib` re-exports all of them.

| Crate | What it does |
|---|---|
| `starlib-launcher` | Finds the newest installed game build, starts it with the right ports and options, and stops it when dropped. |
| `starlib-network` | A WebSocket connection to the game, with pluggable retry strategies while the game starts up. |
| `starlib-proto` | The SC2 API protocol as hand-written [prost] types, with no build step or `protoc`. |
| `starlib-command` | The typed client (`Sc2Rpc`, `Sc2Api`), game setup (maps, players, computer opponents), the `Agent` trait and the single-player runner. |
| `starlib-types` | Game state: units, positions, geometry, pixel grids, and the generated IDs for abilities, units, upgrades, buffs and effects. Unit queries are iterator adapters, for example `state.units().owned().workers().closest(point)`. |
| `starlib-runtime` | Work in progress: a small system-and-resource scheduler for bots, with composable operations (train, build, supply checks) and trackers for units and costs. |
| `starlib-utils` | Logging, path discovery and game version parsing. |

`generate-ids` is a separate tool that reads the game's `stableid.json` and writes the ID enums in `starlib-types`.
Run it with `cargo generate`. The generated files are about half of the line count.

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.

[s2client-proto]: https://github.com/Blizzard/s2client-proto
[prost]: https://github.com/tokio-rs/prost
