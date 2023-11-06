use crate::views::SnakeGrid;
use sdl2::event::Event;
use skia_safe::Canvas;
use skia_test::{
  models::Box2D,
  views::{Shake, View},
  Context,
};

#[derive(Debug)]
pub(crate) struct GamePage {
  child: Shake<SnakeGrid>,
}

impl GamePage {
  pub fn new() -> Self {
    Self {
      child: Shake::new(false, SnakeGrid::new(|| {})),
    }
  }
}

impl View for GamePage {
  fn on_event(&mut self, context: &mut Context, event: &Event) {
    self.child.on_event(context, event);
  }

  fn tick(&mut self, context: &mut Context, dt: f32) {
    self.child.tick(context, dt);
  }

  fn draw(&mut self, context: &mut Context, canvas: &Canvas, constraint: Box2D) {
    self.child.draw(context, canvas, constraint);
  }
}
