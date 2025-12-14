use starlib_types::Cost;
use starlib_types::UnitTypeData;

pub trait TryCost {
  fn try_cost(&self) -> Option<Cost>;
}

impl<T> TryCost for &'_ T
where
  T: TryCost + ?Sized,
{
  #[inline]
  fn try_cost(&self) -> Option<Cost> {
    (**self).try_cost()
  }
}

impl TryCost for Cost {
  #[inline]
  fn try_cost(&self) -> Option<Cost> {
    Some(*self)
  }
}

impl TryCost for UpgradeData {
  #[inline]
  fn try_cost(&self) -> Option<Cost> {
    Some(self.cost())
  }
}

impl TryCost for UnitTypeData {
  #[inline]
  fn try_cost(&self) -> Option<Cost> {
    Some(self.cost())
  }
}
