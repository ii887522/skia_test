use crate::models::Box2D;
use skia_safe::Canvas;

pub trait View {
  fn tick(&mut self, _dt: f32) {}
  fn draw(&mut self, _canvas: &Canvas, _constraint: Box2D) {}
}
