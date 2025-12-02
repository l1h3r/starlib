use serde_json::from_reader;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

use generate_ids::StableIdMap;
use generate_ids::Variant;
use generate_ids::template;

#[cfg(not(any(target_os = "macos", windows)))]
compile_error!("macos or windows");

#[cfg(target_os = "macos")]
const DATA: &str = "Library/Application Support/Blizzard/StarCraft II/stableid.json";

#[cfg(windows)]
const DATA: &str = "Documents/StarCraft II/stableid.json";

fn main() -> Result<(), Error> {
  let data: StableIdMap = parse_stable_ids()?;
  let path: PathBuf = output_path();

  fs::create_dir_all(path.as_path())?;

  let mut abilities: BTreeSet<(u32, Variant)> = BTreeSet::new();
  let mut buffs: BTreeSet<(u32, Variant)> = BTreeSet::new();
  let mut effects: BTreeSet<(u32, Variant)> = BTreeSet::new();
  let mut units: BTreeSet<(u32, Variant)> = BTreeSet::new();
  let mut upgrades: BTreeSet<(u32, Variant)> = BTreeSet::new();

  for item in data.abilities {
    if item.remapid.is_some() {
      continue;
    }

    assert!(abilities.insert((item.id, item.name())), "Duplicate Ability");
  }

  for item in data.buffs {
    assert!(buffs.insert((item.id, item.name)), "Duplicate Buff");
  }

  for item in data.effects {
    assert!(effects.insert((item.id, item.name())), "Duplicate Effect");
  }

  for item in data.units {
    assert!(units.insert((item.id, item.name)), "Duplicate Unit");
  }

  for item in data.upgrades {
    assert!(upgrades.insert((item.id, item.name)), "Duplicate Upgrade");
  }

  let gen_ability_id: String = generate_enum("AbilityId", "Null", abilities);
  let gen_buff_id: String = generate_enum("BuffId", "Null", buffs);
  let gen_effect_id: String = generate_enum("EffectId", "Null", effects);
  let gen_unit_type_id: String = generate_enum("UnitTypeId", "NotAUnit", units);
  let gen_upgrade_id: String = generate_enum("UpgradeId", "Null", upgrades);

  fs::write(path.join("ability_id.rs"), gen_ability_id.trim_start())?;
  fs::write(path.join("buff_id.rs"), gen_buff_id.trim_start())?;
  fs::write(path.join("effect_id.rs"), gen_effect_id.trim_start())?;
  fs::write(path.join("unit_type_id.rs"), gen_unit_type_id.trim_start())?;
  fs::write(path.join("upgrade_id.rs"), gen_upgrade_id.trim_start())?;

  fs::write(path.join("mod.rs"), template::ROOT.trim_start())?;

  Ok(())
}

fn output_path() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("crates")
    .join("starlib-types")
    .join("src")
    .join("stable")
    .canonicalize()
    .unwrap()
}

fn parse_stable_ids() -> Result<StableIdMap, Error> {
  let home: PathBuf = env::home_dir().expect("home_dir");
  let path: PathBuf = home.join(DATA);
  let file: File = File::open(path)?;
  let data: StableIdMap = from_reader(file)?;

  Ok(data)
}

fn generate_enum<I>(name: &'static str, null: &'static str, source: I) -> String
where
  I: IntoIterator<Item = (u32, Variant)>,
{
  let mut data: Vec<(u32, Variant)> = source
    .into_iter()
    .collect();

  data.sort_by_key(|(id, _)| *id);

  let variants: Vec<String> = data
    .iter()
    .map(|(id, name)| format!("{name} = {id}"))
    .collect();

  let from_u32: Vec<String> = data
    .iter()
    .map(|(id, name)| format!("{id} => Self::{name}"))
    .collect();

  let into_str: Vec<String> = data
    .iter()
    .map(|(_, name)| format!("Self::{name} => \"{name}\""))
    .collect();

  template::ENUM
    .replace("{{name}}", name)
    .replace("{{null}}", null)
    .replace("{{variants}}", variants.join(",\n  ").as_str())
    .replace("{{from_u32}}", from_u32.join(",\n      ").as_str())
    .replace("{{into_str}}", into_str.join(",\n      ").as_str())
}
