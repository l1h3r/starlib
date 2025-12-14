use std::any::type_name;
use std::cell::RefMut;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::exec::Argument;
use crate::exec::Resource;
use crate::exec::World;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// Mut
// -----------------------------------------------------------------------------

#[repr(transparent)]
pub struct Mut<'world, R>
where
  R: Resource + ?Sized,
{
  pub(crate) borrow: RefMut<'world, dyn Resource>,
  pub(crate) marker: PhantomData<&'world R>,
}

impl<R> Debug for Mut<'_, R>
where
  R: Debug + Resource,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&**self, f)
  }
}

impl<R> Display for Mut<'_, R>
where
  R: Display + Resource,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&**self, f)
  }
}

impl<R> Deref for Mut<'_, R>
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

impl<R> DerefMut for Mut<'_, R>
where
  R: Resource,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    // SAFETY: We asserted the type of `R` during construction
    unsafe { self.borrow.downcast_mut_unchecked() }
  }
}

impl<R> AsRef<R> for Mut<'_, R>
where
  R: Resource,
{
  #[inline]
  fn as_ref(&self) -> &R {
    self.deref()
  }
}

impl<R> AsMut<R> for Mut<'_, R>
where
  R: Resource,
{
  #[inline]
  fn as_mut(&mut self) -> &mut R {
    self.deref_mut()
  }
}

impl<R> Argument for Mut<'_, R>
where
  R: Resource + ?Sized,
{
  type Item<'world> = Mut<'world, R>;

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
    world.fetch_mut::<R>()
  }
}
