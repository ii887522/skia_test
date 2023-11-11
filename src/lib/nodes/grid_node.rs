use super::{BoxNode, Node};
use crate::{models::Box2D, Context};
use sdl2::event::Event;
use skia_safe::Canvas;
use std::fmt::{self, Debug, Formatter};

pub struct GridNode {
  pub dim: (usize, usize),
  pub gap: (f32, f32),
  pub size: (f32, f32),
  pub maker: Box<dyn Fn(usize) -> Box<dyn Node>>,
}

impl Debug for GridNode {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("GridNode")
      .field("dim", &self.dim)
      .field("gap", &self.gap)
      .field("size", &self.size)
      .finish_non_exhaustive()
  }
}

impl Default for GridNode {
  fn default() -> Self {
    Self {
      dim: (4, 4),
      gap: (4f32, 4f32),
      size: (f32::MAX, f32::MAX),
      maker: Box::new(|_| Box::<BoxNode>::default()),
    }
  }
}

impl Node for GridNode {
  fn on_event(&mut self, context: &mut Context, event: &Event) {
    // Preconditions
    debug_assert_ne!(self.dim.0, 0, "dim.0 must be a positive integer");
    debug_assert_ne!(self.dim.1, 0, "dim.1 must be a positive integer");

    for i in 0..self.dim.0 * self.dim.1 {
      (self.maker)(i).on_event(context, event);
    }
  }

  fn tick(&mut self, context: &mut Context, dt: f32) {
    // Preconditions
    debug_assert_ne!(self.dim.0, 0, "dim.0 must be a positive integer");
    debug_assert_ne!(self.dim.1, 0, "dim.1 must be a positive integer");

    for i in 0..self.dim.0 * self.dim.1 {
      (self.maker)(i).tick(context, dt);
    }
  }

  fn get_size(&self) -> (f32, f32) {
    self.size
  }

  fn draw(&self, canvas: &Canvas, constraint: Box2D) {
    // Preconditions
    debug_assert_ne!(self.dim.0, 0, "dim.0 must be a positive integer");
    debug_assert_ne!(self.dim.1, 0, "dim.1 must be a positive integer");
    debug_assert!(self.gap.0 > 0f32, "gap.0 must be a positive value");
    debug_assert!(self.gap.1 > 0f32, "gap.1 must be a positive value");
    debug_assert!(self.size.0 > 0f32, "size.0 must be a positive value");
    debug_assert!(self.size.1 > 0f32, "size.1 must be a positive value");

    let size = (self.size.0.min(constraint.size.0), self.size.1.min(constraint.size.1));

    let cell_size = (
      (size.0 - self.gap.0 * (self.dim.0 - 1) as f32) / self.dim.0 as f32,
      (size.1 - self.gap.1 * (self.dim.1 - 1) as f32) / self.dim.1 as f32,
    );

    let position = (
      constraint.position.0 + (constraint.size.0 - size.0) * 0.5f32,
      constraint.position.1 + (constraint.size.1 - size.1) * 1.0f32,
    );

    for i in 0..self.dim.0 * self.dim.1 {
      (self.maker)(i).draw(
        canvas,
        Box2D {
          position: (
            position.0 + (i % self.dim.0) as f32 * (cell_size.0 + self.gap.0),
            position.1 + (i / self.dim.0) as f32 * (cell_size.1 + self.gap.1),
          ),
          size: cell_size,
        },
      );
    }
  }
}
