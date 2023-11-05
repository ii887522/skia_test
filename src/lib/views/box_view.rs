use super::View;
use crate::models::Box2D;
use skia_safe::{Canvas, Color, Paint, Rect};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BoxView {
  pub color: Color,
}

impl View for BoxView {
  fn draw(&mut self, canvas: &Canvas, constraint: Box2D) {
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
