use std::fmt::Formatter;
use std::fmt::Result;
use std::ptr;

use crate::grid::Pixel;

// -----------------------------------------------------------------------------
// Pixel - Block
// -----------------------------------------------------------------------------

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Block(u8);

impl Block {
  #[inline]
  pub const fn get(self) -> u8 {
    self.0
  }
}

impl Pixel for Block {
  type Inner = u8;

  #[inline]
  fn cast_ref(pixel: &Self::Inner) -> &Self {
    unsafe { &*ptr::from_ref(pixel).cast::<Self>() }
  }

  #[inline]
  fn cast_mut(pixel: &mut Self::Inner) -> &mut Self {
    unsafe { &mut *ptr::from_mut(pixel).cast::<Self>() }
  }

  #[inline]
  fn fmt(this: &Self::Inner, f: &mut Formatter<'_>) -> Result {
    match this {
      0 => f.write_str(" "),
      1 => f.write_str("█"),
      _ => unreachable!(),
    }
  }
}

// -----------------------------------------------------------------------------
// Pixel - Height
// -----------------------------------------------------------------------------

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Height(i8);

impl Height {
  #[inline]
  pub const fn get(self) -> i8 {
    self.0
  }
}

impl Pixel for Height {
  type Inner = i8;

  #[inline]
  fn cast_ref(pixel: &Self::Inner) -> &Self {
    unsafe { &*ptr::from_ref(pixel).cast::<Self>() }
  }

  #[inline]
  fn cast_mut(pixel: &mut Self::Inner) -> &mut Self {
    unsafe { &mut *ptr::from_mut(pixel).cast::<Self>() }
  }

  #[inline]
  fn fmt(this: &Self::Inner, f: &mut Formatter<'_>) -> Result {
    // █ ▇ ▆ ▅ ▄ ▃ ▂ ▁
    match this {
      -128..-64 => f.write_str("▁"),
      -64..-0 => f.write_str("▄"),
      0..64 => f.write_str("▆"),
      64..=127 => f.write_str("█"),
    }
  }
}
