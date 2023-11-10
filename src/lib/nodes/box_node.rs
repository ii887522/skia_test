use super::Node;
use crate::models::Box2D;
use skia_safe::{Canvas, Color, Paint, Rect};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BoxNode {
  pub color: Color,
}

impl Node for BoxNode {
  fn draw(&self, canvas: &Canvas, constraint: Box2D) {
    canvas.draw_rect(
      Rect::from_xywh(
        constraint.position.0,
        constraint.position.1,
        constraint.size.0,
        constraint.size.1,
      ),
      Paint::default().set_color(self.color),
    );
  }
}
