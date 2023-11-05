use crate::models::Box2D;
use skia_safe::Canvas;

pub trait View {
  fn draw(&mut self, _canvas: &Canvas, _constraint: Box2D) {}
}
