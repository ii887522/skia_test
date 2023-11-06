use crate::{models::Box2D, Context};
use sdl2::event::Event;
use skia_safe::Canvas;

pub trait View {
  fn on_event(&mut self, _context: &mut Context, _event: &Event) {}
  fn tick(&mut self, _context: &mut Context, _dt: f32) {}
  fn draw(&mut self, _context: &mut Context, _canvas: &Canvas, _constraint: Box2D) {}
}
