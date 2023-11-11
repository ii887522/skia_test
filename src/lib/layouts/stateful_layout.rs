use crate::{common::Sharable, models::Box2D, Context, View};
use sdl2::event::Event;
use skia_safe::Canvas;
use std::{cell::RefCell, rc::Rc};

pub trait StatefulLayout {
  fn get_key(&self) -> &str;
  fn make_state(&mut self) -> Rc<RefCell<dyn State>>;
}

pub trait State {
  fn on_event(&mut self, _context: &mut Context, _event: &Event) {}
  fn tick(&mut self, _context: &mut Context, _dt: f32) {}

  fn get_size(&self) -> (f32, f32) {
    (f32::MAX, f32::MAX)
  }

  fn pre_draw(&self, _canvas: &Canvas, _constraint: Box2D) {}

  fn make(&self, _constraint: Box2D) -> Option<Sharable<View>> {
    None
  }

  fn post_draw(&self, _canvas: &Canvas, _constraint: Box2D) {}
}
