use super::{Unit, View};
use crate::{common::Ticker, models::Box2D, Context};
use sdl2::event::Event;
use skia_safe::Canvas;
use tinyrand::Rand;
use tinyrand_std::thread_rand;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Shake<Child = Unit> {
  strength: f32,
  angle: f32,
  ticker: Ticker,
  is_enabled: bool,
  child: Child,
}

impl<Child> Shake<Child> {
  pub fn new(is_enabled: bool, child: Child) -> Self {
    Self {
      strength: 16f32,
      angle: 0f32,
      ticker: Ticker::new(0.02f32),
      is_enabled,
      child,
    }
  }
}

impl<Child: View> View for Shake<Child> {
  fn on_event(&mut self, context: &mut Context, event: &Event) {
    self.child.on_event(context, event);
  }

  fn tick(&mut self, context: &mut Context, dt: f32) {
    if self.is_enabled {
      self.ticker.advance(dt, |_| {
        self.angle = (thread_rand().next_u32() as f32 / u32::MAX as f32) * 2f32 * std::f32::consts::PI;
      });
    }

    self.child.tick(context, dt);
  }

  fn draw(&self, context: &mut Context, canvas: &Canvas, constraint: Box2D) {
    if self.is_enabled {
      canvas.save();
      canvas.translate((self.strength * self.angle.cos(), self.strength * self.angle.sin()));
    }

    self.child.draw(context, canvas, constraint);

    if self.is_enabled {
      canvas.restore();
    }
  }
}
