use serde::Deserialize;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub mod template {
  pub const ROOT: &'static str = include_str!("template/mod.rs");
  pub const ENUM: &'static str = include_str!("template/enum.rs");
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StableIdMap {
  pub abilities: Vec<Ability>,
  pub buffs: Vec<Buff>,
  pub effects: Vec<Effect>,
  pub units: Vec<Unit>,
  pub upgrades: Vec<Upgrade>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Ability {
  pub id: u32,
  pub remapid: Option<u32>,
  pub name: Variant,
  pub buttonname: Variant,
  pub friendlyname: Option<Variant>,
  pub index: u32,
}

impl Ability {
  pub fn name(self) -> Variant {
    if self.id == 10 {
      // HoldFire -> StopSpecial
      return self.buttonname;
    }

    if self.id == 3665 {
      // Stop -> Stop2
      return Variant("Stop2".to_owned());
    }

    if self.id == 325 {
      // TerranBuildRefinery -> TerranBuildRefinery2
      return Variant("TerranBuildRefinery2".to_owned());
    }

    let force: bool = self.buttonname.0.contains(self.name.0.as_str());

    let name: Variant = match self.friendlyname {
      Some(name) if name.0 == "UnloadUnit" => self.name,
      Some(name) => name,
      None => self.name,
    };

    if self.index == 0 {
      name
    } else if self.buttonname.0.is_empty() {
      Variant(format!("{}{}", name, self.index + 1))
    } else if force {
      self.buttonname
    } else {
      Variant(format!("{}{}", name, self.buttonname))
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Buff {
  pub id: u32,
  pub name: Variant,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Effect {
  pub id: u32,
  pub name: Variant,
  pub radius: (),
  pub friendly_name: Option<Variant>,
}

impl Effect {
  pub fn name(self) -> Variant {
    self.friendly_name.unwrap_or(self.name)
  }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Unit {
  pub id: u32,
  pub name: Variant,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Upgrade {
  pub id: u32,
  pub name: Variant,
}

// -----------------------------------------------------------------------------
// Variant
// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
pub struct Variant(pub String);

impl Variant {
  fn sanitize(&self) -> Cow<'_, str> {
    let mut name: Cow<'_, str> = Cow::Borrowed(self.0.as_str());

    if name.is_empty() {
      return name;
    }

    if name.contains('@') {
      name = Cow::Owned(name.replace('@', ""));
    }

    if name.contains(' ') {
      name = Cow::Owned(name.replace(' ', ""));
    }

    let Some(ch) = name.chars().next() else {
      unreachable!()
    };

    if ch.is_lowercase() {
      name = Cow::Owned(format!("{}{}", ch.to_uppercase(), &name[1..]));
    }

    match name.as_ref() {
      "250mmStrikeCannons" => Cow::Borrowed("StrikeCannons250mm"),
      "250mmStrikeCannonsCancel" => Cow::Borrowed("StrikeCannons250mmCancel"),
      "330mmBarrageCannons" => Cow::Borrowed("BarrageCannons330mm"),
      "330mmBarrageCannonsCancel" => Cow::Borrowed("BarrageCannons330mmCancel"),

      "4SlotBag" => Cow::Borrowed("SlotBag4"),
      "6SlotBag" => Cow::Borrowed("SlotBag6"),
      "8SlotBag" => Cow::Borrowed("SlotBag8"),
      "10SlotBag" => Cow::Borrowed("SlotBag10"),
      "12SlotBag" => Cow::Borrowed("SlotBag12"),
      "14SlotBag" => Cow::Borrowed("SlotBag14"),
      "16SlotBag" => Cow::Borrowed("SlotBag16"),
      "18SlotBag" => Cow::Borrowed("SlotBag18"),
      "20SlotBag" => Cow::Borrowed("SlotBag20"),
      "22SlotBag" => Cow::Borrowed("SlotBag22"),
      "24SlotBag" => Cow::Borrowed("SlotBag24"),

      "Overlordspeed" => Cow::Borrowed("OverlordSpeed"),
      "Overlordtransport" => Cow::Borrowed("OverlordTransport"),
      "Zerglingattackspeed" => Cow::Borrowed("ZerglingAttackSpeed"),
      "Zerglingmovementspeed" => Cow::Borrowed("ZerglingMovementSpeed"),
      "Hydraliskspeed" => Cow::Borrowed("HydraliskSpeed"),
      "Haltech" => Cow::Borrowed("HalTech"),

      _ => name,
    }
  }
}

impl Debug for Variant {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(&self.sanitize(), f)
  }
}

impl Display for Variant {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Display::fmt(&self.sanitize(), f)
  }
}
