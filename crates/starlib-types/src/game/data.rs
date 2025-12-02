use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::collections::btree_map;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::Filter;

use starlib_proto::ResponseData;

use crate::bridge::AbilityData;
use crate::bridge::BuffData;
use crate::bridge::EffectData;
use crate::bridge::UnitTypeData;
use crate::bridge::UpgradeData;
use crate::core::Cost;
use crate::stable::AbilityId;
use crate::stable::BuffId;
use crate::stable::EffectId;
use crate::stable::UnitTypeId;
use crate::stable::UpgradeId;
use crate::traits::FromProto;
use crate::traits::IntoSc2;

type Key<T> = <T as DataType>::Key;
type Iter<'a, T> = Filter<btree_map::Iter<'a, Key<T>, T>, fn(&(&Key<T>, &T)) -> bool>;
type Values<'a, T> = Filter<btree_map::Values<'a, Key<T>, T>, fn(&&T) -> bool>;

// -----------------------------------------------------------------------------
// Game Data
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct GameData {
  abilities: DataMap<AbilityData>,
  buffs: DataMap<BuffData>,
  effects: DataMap<EffectData>,
  units: DataMap<UnitTypeData>,
  upgrades: DataMap<UpgradeData>,
}

impl GameData {
  // ---------------------------------------------------------------------------
  // Getters - Cost
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn ability_cost(&self, id: AbilityId) -> Option<Cost> {
    if let Some(item) = self.units().find(|item| item.ability_id() == id) {
      return Some(item.cost());
    }

    if let Some(item) = self.upgrades().find(|item| item.ability_id() == id) {
      return Some(item.cost());
    }

    None
  }

  #[inline]
  pub fn unit_cost(&self, id: UnitTypeId) -> Option<Cost> {
    self.try_unit(id).map(UnitTypeData::cost)
  }

  #[inline]
  pub fn upgrade_cost(&self, id: UpgradeId) -> Option<Cost> {
    self.try_upgrade(id).map(UpgradeData::cost)
  }

  // ---------------------------------------------------------------------------
  // Getters - Panic
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn ability(&self, id: AbilityId) -> &AbilityData {
    self
      .try_ability(id)
      .unwrap_or_else(|| panic!("missing ability data for {id}"))
  }

  #[inline]
  pub fn buff(&self, id: BuffId) -> &BuffData {
    self
      .try_buff(id)
      .unwrap_or_else(|| panic!("missing buff data for {id}"))
  }

  #[inline]
  pub fn effect(&self, id: EffectId) -> &EffectData {
    self
      .try_effect(id)
      .unwrap_or_else(|| panic!("missing effect data for {id}"))
  }

  #[inline]
  pub fn unit(&self, id: UnitTypeId) -> &UnitTypeData {
    self
      .try_unit(id)
      .unwrap_or_else(|| panic!("missing unit data for {id}"))
  }

  #[inline]
  pub fn upgrade(&self, id: UpgradeId) -> &UpgradeData {
    self
      .try_upgrade(id)
      .unwrap_or_else(|| panic!("missing upgrade data for {id}"))
  }

  // ---------------------------------------------------------------------------
  // Getters - Option
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn try_ability(&self, id: AbilityId) -> Option<&AbilityData> {
    self.abilities.get(&id)
  }

  #[inline]
  pub fn try_buff(&self, id: BuffId) -> Option<&BuffData> {
    self.buffs.get(&id)
  }

  #[inline]
  pub fn try_effect(&self, id: EffectId) -> Option<&EffectData> {
    self.effects.get(&id)
  }

  #[inline]
  pub fn try_unit(&self, id: UnitTypeId) -> Option<&UnitTypeData> {
    self.units.get(&id)
  }

  #[inline]
  pub fn try_upgrade(&self, id: UpgradeId) -> Option<&UpgradeData> {
    self.upgrades.get(&id)
  }

  // ---------------------------------------------------------------------------
  // Iterators
  // ---------------------------------------------------------------------------

  #[inline]
  pub fn abilities(&self) -> Values<'_, AbilityData> {
    self.abilities.values()
  }

  #[inline]
  pub fn buffs(&self) -> Values<'_, BuffData> {
    self.buffs.values()
  }

  #[inline]
  pub fn effects(&self) -> Values<'_, EffectData> {
    self.effects.values()
  }

  #[inline]
  pub fn units(&self) -> Values<'_, UnitTypeData> {
    self.units.values()
  }

  #[inline]
  pub fn upgrades(&self) -> Values<'_, UpgradeData> {
    self.upgrades.values()
  }
}

impl FromProto<ResponseData> for GameData {
  #[inline]
  fn from_proto(other: ResponseData) -> Self {
    let mut abilities: DataMap<AbilityData> = DataMap::new();
    let mut buffs: DataMap<BuffData> = DataMap::new();
    let mut effects: DataMap<EffectData> = DataMap::new();
    let mut units: DataMap<UnitTypeData> = DataMap::new();
    let mut upgrades: DataMap<UpgradeData> = DataMap::new();

    for item in other.abilities {
      if item.remaps_to_ability_id != 0 {
        continue;
      }

      abilities.insert(item.into_sc2());
    }

    for item in other.buffs {
      buffs.insert(item.into_sc2());
    }

    for item in other.effects {
      effects.insert(item.into_sc2());
    }

    for item in other.units {
      units.insert(item.into_sc2());
    }

    for item in other.upgrades {
      upgrades.insert(item.into_sc2());
    }

    Self {
      abilities,
      buffs,
      effects,
      units,
      upgrades,
    }
  }
}

// -----------------------------------------------------------------------------
// Data Map
// -----------------------------------------------------------------------------

struct DataMap<T>
where
  T: DataType,
{
  inner: BTreeMap<T::Key, T>,
}

impl<T> DataMap<T>
where
  T: DataType,
{
  #[inline]
  fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }

  #[inline]
  fn insert(&mut self, item: T) {
    if self.inner.contains_key(&item.resource()) {
      println!("Duplicate Resource: {item:?}");
    }

    self.inner.insert(item.resource(), item);
  }

  #[inline]
  fn get<Q>(&self, name: &Q) -> Option<&T>
  where
    T::Key: Borrow<Q>,
    Q: Ord + ?Sized,
  {
    self.inner.get(name).filter(|item| item.include())
  }

  #[inline]
  fn iter(&self) -> Iter<'_, T> {
    self.inner.iter().filter(|(_, item)| item.include())
  }

  #[inline]
  fn values(&self) -> Values<'_, T> {
    self.inner.values().filter(|item| item.include())
  }
}

impl<T> Debug for DataMap<T>
where
  T: DataType,
  T::Key: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_map().entries(self.iter()).finish()
  }
}

// -----------------------------------------------------------------------------
// Data Type
// -----------------------------------------------------------------------------

#[doc(hidden)]
pub trait DataType: Debug {
  type Key: Ord;

  fn resource(&self) -> Self::Key;

  #[inline]
  fn include(&self) -> bool {
    true
  }
}

impl DataType for AbilityData {
  type Key = AbilityId;

  #[inline]
  fn resource(&self) -> Self::Key {
    self.id_raw()
  }

  #[inline]
  fn include(&self) -> bool {
    self.available()
  }
}

impl DataType for BuffData {
  type Key = BuffId;

  #[inline]
  fn resource(&self) -> Self::Key {
    self.id()
  }
}

impl DataType for EffectData {
  type Key = EffectId;

  #[inline]
  fn resource(&self) -> Self::Key {
    self.id()
  }
}

impl DataType for UnitTypeData {
  type Key = UnitTypeId;

  #[inline]
  fn resource(&self) -> Self::Key {
    self.id()
  }

  #[inline]
  fn include(&self) -> bool {
    self.available()
  }
}

impl DataType for UpgradeData {
  type Key = UpgradeId;

  #[inline]
  fn resource(&self) -> Self::Key {
    self.id()
  }

  #[inline]
  fn include(&self) -> bool {
    self.ability_id().into_option().is_some()
  }
}
