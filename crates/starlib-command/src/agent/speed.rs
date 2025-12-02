// -----------------------------------------------------------------------------
// Agent Speed
//
// More Info: https://liquipedia.net/starcraft2/Game_Speed
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Speed {
  Slower = 0,
  Slow = 1,
  Normal = 2,
  Fast = 3,
  #[default]
  Faster = 4,
}

impl Speed {
  const FPS: f64 = 16.0;

  #[inline]
  pub const fn fps(self) -> f64 {
    Self::FPS * self.factor()
  }

  #[inline]
  pub const fn factor(self) -> f64 {
    match self {
      Self::Slower => 0.6,
      Self::Slow => 0.8,
      Self::Normal => 1.0,
      Self::Fast => 1.2,
      Self::Faster => 1.4,
    }
  }

  #[inline]
  pub const fn seconds(self) -> f64 {
    match self {
      Self::Slower => 100.0,
      Self::Slow => 75.0,
      Self::Normal => 60.0,
      Self::Fast => 50.0,
      Self::Faster => 42.86,
    }
  }
}
