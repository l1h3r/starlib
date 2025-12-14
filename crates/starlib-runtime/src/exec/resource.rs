use std::any::Any;
use std::any::TypeId;
use std::ptr;

use starlib_types::GameState;

// -----------------------------------------------------------------------------
// Resource
// -----------------------------------------------------------------------------

pub trait Resource: Send + Sync + 'static {
  fn tick(&mut self, state: &GameState);
}

// -----------------------------------------------------------------------------
// Any
// -----------------------------------------------------------------------------

impl dyn Resource {
  #[inline]
  pub fn is<R>(&self) -> bool
  where
    R: Resource,
  {
    TypeId::of::<R>() == self.type_id()
  }

  #[inline]
  pub fn downcast<R>(self: Box<Self>) -> Result<Box<R>, Box<Self>>
  where
    R: Resource,
  {
    if self.is::<R>() {
      // SAFETY: We just ensured the type is `R`.
      Ok(unsafe { self.downcast_unchecked() })
    } else {
      Err(self)
    }
  }

  #[inline]
  pub fn downcast_ref<R>(&self) -> Option<&R>
  where
    R: Resource,
  {
    if self.is::<R>() {
      // SAFETY: We just ensured the type is `R`.
      Some(unsafe { self.downcast_ref_unchecked() })
    } else {
      None
    }
  }

  #[inline]
  pub fn downcast_mut<R>(&mut self) -> Option<&mut R>
  where
    R: Resource,
  {
    if self.is::<R>() {
      // SAFETY: We just ensured the type is `R`.
      Some(unsafe { self.downcast_mut_unchecked() })
    } else {
      None
    }
  }

  #[inline]
  pub unsafe fn downcast_unchecked<R>(self: Box<Self>) -> Box<R>
  where
    R: Resource,
  {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { Box::from_raw(Box::into_raw(self).cast::<R>()) }
  }

  #[inline]
  pub unsafe fn downcast_ref_unchecked<R>(&self) -> &R
  where
    R: Resource,
  {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { &*ptr::from_ref(self).cast::<R>() }
  }

  #[inline]
  pub unsafe fn downcast_mut_unchecked<R>(&mut self) -> &mut R
  where
    R: Resource,
  {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { &mut *ptr::from_mut(self).cast::<R>() }
  }
}
