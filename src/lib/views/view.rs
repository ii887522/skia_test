use crate::models::Box2D;
use sdl2::event::Event;
use skia_safe::Canvas;

pub trait View {
  fn on_event(&mut self, _event: &Event) {}
  fn tick(&mut self, _dt: f32) {}
  fn draw(&mut self, _canvas: &Canvas, _constraint: Box2D) {}
}
