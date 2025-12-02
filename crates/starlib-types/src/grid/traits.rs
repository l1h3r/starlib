use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::slice;

// -----------------------------------------------------------------------------
// Scalar
// -----------------------------------------------------------------------------

pub trait Scalar: Clone + Copy + Default + Display {
  fn cast(value: u8) -> Self;

  #[inline]
  fn cast_slice(value: &[u8]) -> &[Self] {
    let ptr: *const Self = value.as_ptr().cast();
    let len: usize = value.len();

    unsafe { slice::from_raw_parts(ptr, len) }
  }
}

impl Scalar for i8 {
  #[inline]
  fn cast(scalar: u8) -> Self {
    scalar as Self
  }
}

impl Scalar for u8 {
  #[inline]
  fn cast(scalar: u8) -> Self {
    scalar
  }
}

// -----------------------------------------------------------------------------
// Pixel
// -----------------------------------------------------------------------------

pub trait Pixel {
  type Inner: Scalar;

  fn cast_ref(pixel: &Self::Inner) -> &Self;

  fn cast_mut(pixel: &mut Self::Inner) -> &mut Self;

  #[inline]
  fn fmt(this: &Self::Inner, f: &mut Formatter<'_>) -> Result {
    Display::fmt(this, f)
  }
}

// -----------------------------------------------------------------------------
// Implementations
// -----------------------------------------------------------------------------

impl Pixel for i8 {
  type Inner = Self;

  #[inline]
  fn cast_ref(pixel: &Self::Inner) -> &Self {
    pixel
  }

  #[inline]
  fn cast_mut(pixel: &mut Self::Inner) -> &mut Self {
    pixel
  }
}

impl Pixel for u8 {
  type Inner = Self;

  #[inline]
  fn cast_ref(pixel: &Self::Inner) -> &Self {
    pixel
  }

  #[inline]
  fn cast_mut(pixel: &mut Self::Inner) -> &mut Self {
    pixel
  }
}
