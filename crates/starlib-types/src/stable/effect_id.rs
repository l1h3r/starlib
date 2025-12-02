#![allow(non_camel_case_types)]

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EffectId {
  Null = 0,
  PsiStorm = 1,
  GuardianShield = 2,
  TemporalFieldGrowing = 3,
  TemporalField = 4,
  ThermalLance = 5,
  ScannerSweep = 6,
  NukeDot = 7,
  LiberatorDefenderZoneSetup = 8,
  LiberatorDefenderZone = 9,
  BlindingCloud = 10,
  CorrosiveBile = 11,
  LurkerSpines = 12,
}

impl EffectId {
  #[rustfmt::skip]
  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Null => "Null",
      Self::PsiStorm => "PsiStorm",
      Self::GuardianShield => "GuardianShield",
      Self::TemporalFieldGrowing => "TemporalFieldGrowing",
      Self::TemporalField => "TemporalField",
      Self::ThermalLance => "ThermalLance",
      Self::ScannerSweep => "ScannerSweep",
      Self::NukeDot => "NukeDot",
      Self::LiberatorDefenderZoneSetup => "LiberatorDefenderZoneSetup",
      Self::LiberatorDefenderZone => "LiberatorDefenderZone",
      Self::BlindingCloud => "BlindingCloud",
      Self::CorrosiveBile => "CorrosiveBile",
      Self::LurkerSpines => "LurkerSpines",
    }
  }

  #[inline]
  pub const fn from_u32(this: u32) -> Self {
    match this {
      0 => Self::Null,
      1 => Self::PsiStorm,
      2 => Self::GuardianShield,
      3 => Self::TemporalFieldGrowing,
      4 => Self::TemporalField,
      5 => Self::ThermalLance,
      6 => Self::ScannerSweep,
      7 => Self::NukeDot,
      8 => Self::LiberatorDefenderZoneSetup,
      9 => Self::LiberatorDefenderZone,
      10 => Self::BlindingCloud,
      11 => Self::CorrosiveBile,
      12 => Self::LurkerSpines,
      _ => Self::Null,
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
    matches!(self, Self::Null)
  }
}

impl ::core::default::Default for EffectId {
  #[inline]
  fn default() -> Self {
    Self::Null
  }
}

impl ::core::fmt::Debug for EffectId {
  fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl ::core::fmt::Display for EffectId {
  fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl crate::traits::FromProto<u32> for EffectId {
  #[inline]
  fn from_proto(this: u32) -> Self {
    Self::from_u32(this)
  }
}

impl crate::traits::FromSc2<EffectId> for u32 {
  #[inline]
  fn from_sc2(this: EffectId) -> Self {
    this.into_u32()
  }
}

impl crate::traits::FromProto<i32> for EffectId {
  #[inline]
  fn from_proto(this: i32) -> Self {
    Self::from_u32(this as u32)
  }
}

impl crate::traits::FromSc2<EffectId> for i32 {
  #[inline]
  fn from_sc2(this: EffectId) -> Self {
    this.into_u32() as i32
  }
}
