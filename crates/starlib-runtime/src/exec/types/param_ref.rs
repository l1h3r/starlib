use std::any::type_name;
use std::cell;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::exec::Argument;
use crate::exec::Resource;
use crate::exec::World;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// Ref
// -----------------------------------------------------------------------------

#[repr(transparent)]
pub struct Ref<'world, R>
where
  R: Resource + ?Sized,
{
  pub(crate) borrow: cell::Ref<'world, dyn Resource>,
  pub(crate) marker: PhantomData<&'world R>,
}

impl<'world, R> Clone for Ref<'world, R>
where
  R: Resource + ?Sized,
{
  #[inline]
  fn clone(&self) -> Self {
    Self {
      borrow: cell::Ref::clone(&self.borrow),
      marker: PhantomData,
    }
  }
}

impl<R> Debug for Ref<'_, R>
where
  R: Debug + Resource,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&**self, f)
  }
}

impl<R> Display for Ref<'_, R>
where
  R: Display + Resource,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&**self, f)
  }
}

impl<R> Deref for Ref<'_, R>
where
  R: Resource,
{
  type Target = R;

  #[inline]
  fn deref(&self) -> &Self::Target {
    // SAFETY: We asserted the type of `R` during construction
    unsafe { self.borrow.downcast_ref_unchecked() }
  }
}

impl<R> AsRef<R> for Ref<'_, R>
where
  R: Resource,
{
  #[inline]
  fn as_ref(&self) -> &R {
    self.deref()
  }
}

impl<R> Argument for Ref<'_, R>
where
  R: Resource + ?Sized,
{
  type Item<'world> = Ref<'world, R>;

  #[inline]
  fn setup(world: &mut World) -> Result<(), String> {
    if world.contains::<R>() {
      Ok(())
    } else {
      Err(format!("dangling resource: {}", type_name::<R>()))
    }
  }

  #[inline]
  fn fetch<'world>(world: WorldRef<'world>) -> Self::Item<'world> {
    world.fetch_ref::<R>()
  }
}
