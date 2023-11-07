use crate::views::SnakeGrid;
use sdl2::event::Event;
use skia_safe::Canvas;
use skia_test::{
  common::Ticker,
  models::Box2D,
  views::{Shake, View},
  Context,
};
use std::{cell::Cell, rc::Rc};

#[derive(Debug)]
pub(crate) struct GamePage {
  ticker: Ticker,
  shake: Rc<Cell<bool>>,
  child: Shake<SnakeGrid>,
}

impl GamePage {
  pub fn new() -> Self {
    let shake = Rc::new(Cell::new(false));

    Self {
      shake: Rc::clone(&shake),
      ticker: Ticker::new(0.25f32),
      child: Shake::new(Rc::clone(&shake), SnakeGrid::new(move || shake.set(true))),
    }
  }
}

impl View for GamePage {
  fn on_event(&mut self, context: &mut Context, event: &Event) {
    self.child.on_event(context, event);
  }

  fn tick(&mut self, context: &mut Context, dt: f32) {
    if self.shake.get() {
      self.ticker.advance(dt, |_| self.shake.set(false));
    }

    self.child.tick(context, dt);
  }

  fn draw(&self, context: &Context, canvas: &Canvas, constraint: Box2D) {
    self.child.draw(context, canvas, constraint);
  }
}
