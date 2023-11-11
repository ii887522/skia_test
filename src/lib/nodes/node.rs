use crate::{models::Box2D, Context};
use sdl2::event::Event;
use skia_safe::Canvas;

pub trait Node {
  fn on_event(&mut self, _context: &mut Context, _event: &Event) {}
  fn tick(&mut self, _context: &mut Context, _dt: f32) {}

  fn get_size(&self) -> (f32, f32) {
    (f32::MAX, f32::MAX)
  }

  fn draw(&self, _canvas: &Canvas, _constraint: Box2D) {}
}
