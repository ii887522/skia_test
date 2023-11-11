use super::{stateful_layout::State, StatefulLayout};
use crate::{
  common::{Clock, Sharable},
  models::Box2D,
  Context, View,
};
use skia_safe::Canvas;
use std::{
  cell::{Cell, RefCell},
  fmt::{self, Debug, Formatter},
  rc::Rc,
};
use tinyrand::Rand;
use tinyrand_std::thread_rand;

#[derive(Default)]
pub struct Shake {
  pub is_enabled: Rc<Cell<bool>>,
  pub child: Option<Sharable<View>>,
}

impl Debug for Shake {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("Shake")
      .field("is_enabled", &self.is_enabled)
      .finish_non_exhaustive()
  }
}

impl StatefulLayout for Shake {
  fn get_key(&self) -> &str {
    "lib/layouts/shake"
  }

  fn make_state(&mut self) -> Rc<RefCell<dyn State>> {
    Rc::new(RefCell::new(ShakeState::new(
      Rc::clone(&self.is_enabled),
      self.child.take(),
    )))
  }
}

#[derive(Default)]
struct ShakeState {
  strength: f32,
  angle: f32,
  clock: Clock,
  is_enabled: Rc<Cell<bool>>,
  child: Option<Rc<RefCell<View>>>,
}

impl Debug for ShakeState {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("ShakeState")
      .field("strength", &self.strength)
      .field("angle", &self.angle)
      .field("clock", &self.clock)
      .finish_non_exhaustive()
  }
}

impl ShakeState {
  fn new(is_enabled: Rc<Cell<bool>>, child: Option<Sharable<View>>) -> Self {
    Self {
      strength: 16f32,
      angle: 0f32,
      clock: Clock::new(0.02f32),
      is_enabled,
      child: match child {
        Some(Sharable::Owned(child)) => Some(Rc::new(RefCell::new(child))),
        Some(Sharable::Shared(child)) => Some(child),
        None => None,
      },
    }
  }
}

impl State for ShakeState {
  fn tick(&mut self, _context: &mut Context, dt: f32) {
    if self.is_enabled.get() {
      self.clock.advance(dt, |_| {
        self.angle = (thread_rand().next_u32() as f32 / u32::MAX as f32) * 2f32 * std::f32::consts::PI;
      });
    }
  }

  fn pre_draw(&self, canvas: &Canvas, _constraint: Box2D) {
    if self.is_enabled.get() {
      canvas.save();
      canvas.translate((self.strength * self.angle.cos(), self.strength * self.angle.sin()));
    }
  }

  fn make(&self, _constraint: Box2D) -> Option<Sharable<View>> {
    self.child.as_ref().map(|child| child.into())
  }

  fn post_draw(&self, canvas: &Canvas, _constraint: Box2D) {
    if self.is_enabled.get() {
      canvas.restore();
    }
  }
}
