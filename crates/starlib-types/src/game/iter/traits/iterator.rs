use crate::bridge::Alliance;
use crate::core::UnitTag;
use crate::game::UnitRef;
use crate::game::iter::FilterAlliance;
use crate::game::iter::FilterBuildComplete;
use crate::game::iter::FilterBuildPending;
use crate::game::iter::FilterBuilder;
use crate::game::iter::FilterDistance;
use crate::game::iter::FilterEggs;
use crate::game::iter::FilterHalfSupply;
use crate::game::iter::FilterHarvesters;
use crate::game::iter::FilterLarva;
use crate::game::iter::FilterMineral;
use crate::game::iter::FilterReactors;
use crate::game::iter::FilterSuppliers;
use crate::game::iter::FilterTechlabs;
use crate::game::iter::FilterTownhalls;
use crate::game::iter::FilterType;
use crate::game::iter::FilterVespene;
use crate::game::iter::FilterWatchtowers;
use crate::game::iter::FilterWorkers;
use crate::geom::PointF;
use crate::stable::UnitTypeId;

macro_rules! define_filter_fn {
  ($name:ident::$func:ident) => {
    #[inline]
    fn $func(self) -> $name<Self>
    where
      Self: Sized,
    {
      $name::new(self)
    }
  };
}

// Blanket implementation for all compatible iterators
impl<'a, T> UnitIter<'a> for T where T: Iterator<Item = UnitRef<'a>> {}

pub trait UnitIter<'a>: Iterator<Item = UnitRef<'a>> {
  // ---------------------------------------------------------------------------
  // Finders
  // ---------------------------------------------------------------------------

  #[inline]
  fn find_by_tag(mut self, tag: UnitTag) -> Option<UnitRef<'a>>
  where
    Self: Sized,
  {
    self.find(move |unit| unit.tag() == tag)
  }

  #[inline]
  fn first_complete(self) -> Option<UnitRef<'a>>
  where
    Self: Sized,
  {
    self.max_by_key(UnitRef::build_progress)
  }

  #[inline]
  fn last_complete(self) -> Option<UnitRef<'a>>
  where
    Self: Sized,
  {
    self.min_by_key(UnitRef::build_progress)
  }

  // ---------------------------------------------------------------------------
  // Filters - Alliance
  // ---------------------------------------------------------------------------

  #[inline]
  fn by_alliance(self, value: Alliance) -> FilterAlliance<Self>
  where
    Self: Sized,
  {
    FilterAlliance::new(value, self)
  }

  #[inline]
  fn owned(self) -> FilterAlliance<Self>
  where
    Self: Sized,
  {
    self.by_alliance(Alliance::User)
  }

  #[inline]
  fn allies(self) -> FilterAlliance<Self>
  where
    Self: Sized,
  {
    self.by_alliance(Alliance::Ally)
  }

  #[inline]
  fn neutral(self) -> FilterAlliance<Self>
  where
    Self: Sized,
  {
    self.by_alliance(Alliance::Neutral)
  }

  #[inline]
  fn enemies(self) -> FilterAlliance<Self>
  where
    Self: Sized,
  {
    self.by_alliance(Alliance::Enemy)
  }

  // ---------------------------------------------------------------------------
  // Filters - Distance
  // ---------------------------------------------------------------------------

  #[inline]
  fn by_range(self, point: PointF, range: f32) -> FilterDistance<Self>
  where
    Self: Sized,
  {
    FilterDistance::new(point, range, self)
  }

  #[inline]
  fn closest(self, point: PointF) -> Option<UnitRef<'a>>
  where
    Self: Sized,
  {
    self.min_by_key(|unit| unit.point_2d().distance(point))
  }

  #[inline]
  fn furthest(self, point: PointF) -> Option<UnitRef<'a>>
  where
    Self: Sized,
  {
    self.max_by_key(|unit| unit.point_2d().distance(point))
  }

  // ---------------------------------------------------------------------------
  // Filters - Progress
  // ---------------------------------------------------------------------------

  define_filter_fn!(FilterBuildComplete::complete);
  define_filter_fn!(FilterBuildPending::pending);

  // ---------------------------------------------------------------------------
  // Filters - Resource
  // ---------------------------------------------------------------------------

  define_filter_fn!(FilterMineral::minerals);
  define_filter_fn!(FilterVespene::vespene);

  // ---------------------------------------------------------------------------
  // Filters - Unit Type
  // ---------------------------------------------------------------------------

  #[inline]
  fn of(self, kind: UnitTypeId) -> FilterType<Self>
  where
    Self: Sized,
  {
    FilterType::new(kind, self)
  }

  #[inline]
  fn builder_of(self, kind: UnitTypeId) -> FilterBuilder<Self>
  where
    Self: Sized,
  {
    FilterBuilder::new(kind, self)
  }

  define_filter_fn!(FilterTownhalls::townhalls);
  define_filter_fn!(FilterSuppliers::suppliers);
  define_filter_fn!(FilterHarvesters::harvesters);
  define_filter_fn!(FilterTechlabs::techlabs);
  define_filter_fn!(FilterReactors::reactors);
  define_filter_fn!(FilterWorkers::workers);

  define_filter_fn!(FilterWatchtowers::watchtowers);
  define_filter_fn!(FilterHalfSupply::half_supply);
  define_filter_fn!(FilterLarva::larva);
  define_filter_fn!(FilterEggs::eggs);
}
