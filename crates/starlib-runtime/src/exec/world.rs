use std::any::Any;
use std::any::TypeId;
use std::any::type_name;
use std::cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use starlib_types::GameState;

use crate::exec::Mut;
use crate::exec::Ref;
use crate::exec::Resource;
use crate::exec::WorldRef;

// -----------------------------------------------------------------------------
// World
// -----------------------------------------------------------------------------

pub struct World {
  inner: HashMap<TypeId, RefCell<Box<dyn Resource>>>,
  state: Option<Arc<GameState>>,
}

impl World {
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: HashMap::new(),
      state: None,
    }
  }

  #[inline]
  pub(crate) fn world_ref(&self) -> WorldRef<'_> {
    WorldRef::new(self)
  }

  #[inline]
  pub(crate) fn state(&self) -> Option<&GameState> {
    self.state.as_deref()
  }

  #[inline]
  pub fn with<F>(&mut self, state: Arc<GameState>, f: F)
  where
    F: FnOnce(&mut Self),
  {
    self.state = Some(state);

    f(self);

    self.state = None;
  }

  pub fn for_each<F>(&self, f: F)
  where
    F: Fn(&mut dyn Resource),
  {
    for resource in self.inner.values() {
      f(&mut *cell::RefMut::map(resource.borrow_mut(), Box::as_mut));
    }
  }

  pub fn insert<R>(&mut self, resource: R)
  where
    R: Resource,
  {
    if self.contains::<R>() {
      panic!("resource already exists: {}", type_name::<R>());
    } else {
      self
        .inner
        .insert(resource.type_id(), RefCell::new(Box::new(resource)));
    }
  }

  pub fn remove<R>(&mut self) -> Option<R>
  where
    R: Resource,
  {
    self
      .inner
      .remove(&TypeId::of::<R>())
      .and_then(|boxed| boxed.into_inner().downcast().ok())
      .map(|boxed| *boxed)
  }

  pub fn contains<R>(&self) -> bool
  where
    R: Resource + ?Sized,
  {
    self.inner.contains_key(&TypeId::of::<R>())
  }

  pub(crate) fn fetch_ref<R>(&self) -> Ref<'_, R>
  where
    R: Resource + ?Sized,
  {
    self
      .try_fetch_ref()
      .unwrap_or_else(|| panic!("resource does not exist: {}", type_name::<R>()))
  }

  pub(crate) fn fetch_mut<R>(&self) -> Mut<'_, R>
  where
    R: Resource + ?Sized,
  {
    self
      .try_fetch_mut()
      .unwrap_or_else(|| panic!("resource does not exist: {}", type_name::<R>()))
  }

  pub(crate) fn try_fetch_ref<R>(&self) -> Option<Ref<'_, R>>
  where
    R: Resource + ?Sized,
  {
    Some(Ref {
      borrow: self.fetch_ref_raw::<R>()?,
      marker: PhantomData,
    })
  }

  pub(crate) fn try_fetch_mut<R>(&self) -> Option<Mut<'_, R>>
  where
    R: Resource + ?Sized,
  {
    Some(Mut {
      borrow: self.fetch_mut_raw::<R>()?,
      marker: PhantomData,
    })
  }

  fn fetch_ref_raw<R>(&self) -> Option<cell::Ref<'_, dyn Resource>>
  where
    R: Resource + ?Sized,
  {
    let Some(item) = self.inner.get(&TypeId::of::<R>()) else {
      return None;
    };

    let Ok(borrow) = item.try_borrow() else {
      return None;
    };

    Some(cell::Ref::map(borrow, Box::as_ref))
  }

  fn fetch_mut_raw<R>(&self) -> Option<cell::RefMut<'_, dyn Resource>>
  where
    R: Resource + ?Sized,
  {
    let Some(item) = self.inner.get(&TypeId::of::<R>()) else {
      return None;
    };

    let Ok(borrow) = item.try_borrow_mut() else {
      return None;
    };

    Some(cell::RefMut::map(borrow, Box::as_mut))
  }
}
