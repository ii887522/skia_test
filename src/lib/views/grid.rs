use super::{Unit, View};
use crate::models::Box2D;
use skia_safe::Canvas;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Grid<Maker: FnMut(usize) -> Child, Child: View = Unit> {
  pub dim: (usize, usize),
  pub gap: (Option<f32>, Option<f32>),
  pub size: (Option<f32>, Option<f32>),
  pub maker: Maker,
}

impl<Maker: FnMut(usize) -> Child, Child: View> View for Grid<Maker, Child> {
  fn draw(&mut self, canvas: &Canvas, constraint: Box2D) {
    // Preconditions
    assert_ne!(self.dim.0, 0, "dim.0 must be a positive integer");
    assert_ne!(self.dim.1, 0, "dim.1 must be a positive integer");

    if let Some(value) = self.gap.0 {
      assert!(value > 0f32, "gap.0 must be a positive value");
    }

    if let Some(value) = self.gap.1 {
      assert!(value > 0f32, "gap.1 must be a positive value");
    }

    if let Some(value) = self.size.0 {
      assert!(value > 0f32, "size.0 must be a positive value");
    }

    if let Some(value) = self.size.1 {
      assert!(value > 0f32, "size.1 must be a positive value");
    }

    let size = (
      self.size.0.unwrap_or(constraint.size.0).min(constraint.size.0),
      self.size.1.unwrap_or(constraint.size.1).min(constraint.size.1),
    );

    let gap = (self.gap.0.unwrap_or(4f32), self.gap.1.unwrap_or(4f32));

    let cell_size = (
      (size.0 - gap.0 * (self.dim.0 - 1) as f32) / self.dim.0 as f32,
      (size.1 - gap.1 * (self.dim.1 - 1) as f32) / self.dim.1 as f32,
    );

    // TODO: Let the parent determines this grid position
    let position = (
      (constraint.size.0 - size.0) * 0.5f32,
      (constraint.size.1 - size.1) * 0.5f32,
    );

    for i in 0..self.dim.0 * self.dim.1 {
      (self.maker)(i).draw(
        canvas,
        Box2D {
          position: (
            position.0 + (i % self.dim.0) as f32 * (cell_size.0 + gap.0),
            position.1 + (i / self.dim.0) as f32 * (cell_size.1 + gap.1),
          ),
          size: cell_size,
        },
      );
    }
  }
}
