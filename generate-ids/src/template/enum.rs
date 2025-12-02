#![allow(non_camel_case_types)]

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum {{name}} {
  {{variants}},
}

impl {{name}} {
  #[rustfmt::skip]
  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match self {
      {{into_str}},
    }
  }

  #[inline]
  pub const fn from_u32(this: u32) -> Self {
    match this {
      {{from_u32}},
      _ => Self::{{null}},
    }
  }

  #[inline]
  pub const fn into_u32(self) -> u32 {
    self as u32
  }

  #[inline]
  pub const fn into_option(self) -> Option<Self> {
    if self.is_null() { None } else { Some(self) }
  }

  #[inline]
  pub const fn is_null(&self) -> bool {
    matches!(self, Self::{{null}})
  }
}

impl ::core::default::Default for {{name}} {
  #[inline]
  fn default() -> Self {
    Self::{{null}}
  }
}

impl ::core::fmt::Debug for {{name}} {
  fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl ::core::fmt::Display for {{name}} {
  fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl crate::traits::FromProto<u32> for {{name}} {
  #[inline]
  fn from_proto(this: u32) -> Self {
    Self::from_u32(this)
  }
}

impl crate::traits::FromSc2<{{name}}> for u32 {
  #[inline]
  fn from_sc2(this: {{name}}) -> Self {
    this.into_u32()
  }
}

impl crate::traits::FromProto<i32> for {{name}} {
  #[inline]
  fn from_proto(this: i32) -> Self {
    Self::from_u32(this as u32)
  }
}

impl crate::traits::FromSc2<{{name}}> for i32 {
  #[inline]
  fn from_sc2(this: {{name}}) -> Self {
    this.into_u32() as i32
  }
}
