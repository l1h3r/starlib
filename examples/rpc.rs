use starlib::command::CommandError as Error;
use starlib::command::Sc2Rpc;
use starlib::launcher::DisplayMode;
use starlib::launcher::Process;
use starlib::proto::ResponseAvailableMaps;
use starlib::proto::ResponsePing;
use starlib::utils::Logger;
use std::net::Ipv4Addr;

const HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
const PORT: u16 = 5000;

fn main() {
  assert!(Logger::init_debug(), "failed to initialize logger");

  if let Err(error) = run(HOST, PORT) {
    eprintln!("{error}");
  }
}

fn run(host: Ipv4Addr, port: u16) -> Result<(), Error> {
  // Launch StarCraft II

  let process: Process = Process::builder()
    .host(host)
    .port(port)
    .display(DisplayMode::Windowed)
    .spawn()
    .expect("FIXME");

  // Connect to StarCraft II

  let rpc: Sc2Rpc = Sc2Rpc::connect(host, port)?;

  // Display Basic Information

  let ping: ResponsePing = rpc.ping()?;

  println!("> rpc.ping(..)");
  println!();
  println!("game_version = {}", ping.game_version);
  println!("data_version = {}", ping.data_version);
  println!("data_build   = {}", ping.data_build);
  println!("base_build   = {}", ping.base_build);
  println!();

  let maps: ResponseAvailableMaps = rpc.maps()?;

  println!("> rpc.maps(..)");
  println!();

  for (index, path) in maps.local_map_paths.iter().enumerate() {
    println!("internal map #{index} = {path}", index = index + 1);
  }

  for (index, name) in maps.battlenet_map_names.iter().enumerate() {
    println!("external map #{index} = {name}", index = index + 1);
  }

  println!();

  // Quit StarCraft II

  rpc.quit()?;

  drop(process);

  Ok(())
}
