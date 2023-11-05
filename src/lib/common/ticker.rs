#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Ticker {
  interval: f32,
  elasped: f32,
}

impl Ticker {
  pub fn new(interval: impl Into<Option<f32>> + Copy) -> Self {
    // Preconditions
    if let Some(value) = interval.into() {
      assert!(value > 0f32, "interval must be a positive value");
    }

    Self {
      interval: interval.into().unwrap_or(1f32),
      elasped: 0f32,
    }
  }

  pub fn advance(&mut self, dt: f32, on_tick: impl FnOnce()) {
    self.elasped += dt;

    if self.elasped >= self.interval {
      on_tick();
      self.elasped -= self.interval;
    }
  }
}
