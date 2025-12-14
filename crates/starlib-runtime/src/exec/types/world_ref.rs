use crate::exec::Argument;
use crate::exec::Mut;
use crate::exec::Ref;
use crate::exec::Resource;
use crate::exec::World;

// -----------------------------------------------------------------------------
// World Ref
// -----------------------------------------------------------------------------

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct WorldRef<'world>(&'world World);

impl<'world> WorldRef<'world> {
  #[inline]
  pub(crate) const fn new(world: &'world World) -> Self {
    Self(world)
  }

  #[inline]
  pub(crate) const fn into_inner(self) -> &'world World {
    self.0
  }

  #[inline]
  pub fn fetch_ref<R>(self) -> Ref<'world, R>
  where
    R: Resource + ?Sized,
  {
    self.0.fetch_ref::<R>()
  }

  #[inline]
  pub fn fetch_mut<R>(self) -> Mut<'world, R>
  where
    R: Resource + ?Sized,
  {
    self.0.fetch_mut::<R>()
  }

  pub fn try_fetch_ref<R>(self) -> Option<Ref<'world, R>>
  where
    R: Resource + ?Sized,
  {
    self.0.try_fetch_ref::<R>()
  }

  pub fn try_fetch_mut<R>(self) -> Option<Mut<'world, R>>
  where
    R: Resource + ?Sized,
  {
    self.0.try_fetch_mut::<R>()
  }
}

impl Argument for WorldRef<'_> {
  type Item<'world> = WorldRef<'world>;

  #[inline]
  fn setup(_world: &mut World) -> Result<(), String> {
    Ok(()) // do nothing
  }

  #[inline]
  fn fetch<'world>(world: WorldRef<'world>) -> Self::Item<'world> {
    WorldRef(world.0)
  }
}
