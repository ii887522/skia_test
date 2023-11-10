#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Clock {
  interval: f32,
  elasped: f32,
  is_running: bool,
}

impl Default for Clock {
  fn default() -> Self {
    Self::new(None)
  }
}

impl Clock {
  pub fn new(interval: impl Into<Option<f32>> + Copy) -> Self {
    // Preconditions
    if let Some(value) = interval.into() {
      assert!(value > 0f32, "interval must be a positive value");
    }

    Self {
      interval: interval.into().unwrap_or(1f32),
      elasped: 0f32,
      is_running: true,
    }
  }

  pub const fn is_running(&self) -> bool {
    self.is_running
  }

  pub fn advance(&mut self, dt: f32, on_tick: impl FnOnce(&mut Self)) {
    if !self.is_running {
      return;
    }

    self.elasped += dt;

    if self.elasped >= self.interval {
      on_tick(self);
      self.elasped -= self.interval;
    }
  }

  pub fn pause(&mut self) {
    self.is_running = false;
  }
}
