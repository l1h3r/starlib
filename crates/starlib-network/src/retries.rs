use dyn_clone::DynClone;
use std::time::Duration;

// -----------------------------------------------------------------------------
// Retry Strategy
// -----------------------------------------------------------------------------

pub trait RetryStrategy: DynClone {
  fn tick(&mut self) -> Duration;
}

// -----------------------------------------------------------------------------
// Retry Strategy - Interval
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Interval {
  delay: Duration,
}

impl Interval {
  #[inline]
  pub const fn new(delay: Duration) -> Self {
    Self { delay }
  }

  #[inline]
  pub const fn from_millis(millis: u64) -> Self {
    Self::new(Duration::from_millis(millis))
  }
}

impl RetryStrategy for Interval {
  #[inline]
  fn tick(&mut self) -> Duration {
    self.delay
  }
}

// -----------------------------------------------------------------------------
// Retry Strategy - Exponential
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Exponential {
  this: u64,
  base: u64,
  stop: Option<Duration>,
}

impl Exponential {
  #[inline]
  pub const fn from_millis(millis: u64) -> Self {
    Self {
      this: millis,
      base: millis,
      stop: None,
    }
  }

  #[inline]
  pub fn stop(mut self, value: Option<Duration>) -> Self {
    self.stop = value;
    self
  }
}

impl RetryStrategy for Exponential {
  fn tick(&mut self) -> Duration {
    let this: Duration = Duration::from_millis(self.this);

    if let Some(stop) = self.stop {
      if this > stop {
        return stop;
      }
    }

    self.this = self.this.checked_mul(self.base).unwrap_or(u64::MAX);

    this
  }
}

// -----------------------------------------------------------------------------
// Retry Strategy - Fibonacci
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Fibonacci {
  this: u64,
  next: u64,
  stop: Option<Duration>,
}

impl Fibonacci {
  #[inline]
  pub const fn from_millis(millis: u64) -> Self {
    Self {
      this: millis,
      next: millis,
      stop: None,
    }
  }

  #[inline]
  pub fn stop(mut self, value: Option<Duration>) -> Self {
    self.stop = value;
    self
  }
}

impl RetryStrategy for Fibonacci {
  fn tick(&mut self) -> Duration {
    let this: Duration = Duration::from_millis(self.this);

    if let Some(stop) = self.stop {
      if this > stop {
        return stop;
      }
    }

    let next: u64 = self.this.checked_add(self.next).unwrap_or(u64::MAX);

    self.this = self.next;
    self.next = next;

    this
  }
}
