use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
  //
  // Create
  //
  pub disable_fog: bool,
  pub random_seed: u32,
  pub realtime: bool,
  pub raw_affects_selection: bool,
  //
  // Misc.
  //
  pub match_timeout: Option<Duration>,
  pub step_count: u32,
}

impl Config {
  #[inline]
  pub const fn new() -> Self {
    Self {
      disable_fog: false,
      random_seed: 0,
      realtime: false,
      raw_affects_selection: true,
      match_timeout: None,
      step_count: 4,
    }
  }

  #[inline]
  pub const fn disable_fog(mut self, value: bool) -> Self {
    self.disable_fog = value;
    self
  }

  #[inline]
  pub const fn random_seed(mut self, value: u32) -> Self {
    self.random_seed = value;
    self
  }

  #[inline]
  pub const fn realtime(mut self, value: bool) -> Self {
    self.realtime = value;
    self
  }

  #[inline]
  pub const fn raw_affects_selection(mut self, value: bool) -> Self {
    self.raw_affects_selection = value;
    self
  }

  #[inline]
  pub const fn match_timeout(mut self, value: Duration) -> Self {
    self.match_timeout = Some(value);
    self
  }
}

impl Default for Config {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
