use super::View;
use crate::{models::Box2D, Context};
use sdl2::event::Event;
use skia_safe::Canvas;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Grid<Maker> {
  pub dim: (usize, usize),
  pub gap: Option<(Option<f32>, Option<f32>)>,
  pub size: Option<(Option<f32>, Option<f32>)>,
  pub maker: Maker,
}

impl<Maker: Fn(usize) -> Child, Child: View> View for Grid<Maker> {
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

  fn draw(&self, context: &Context, canvas: &Canvas, constraint: Box2D) {
    // Preconditions
    debug_assert_ne!(self.dim.0, 0, "dim.0 must be a positive integer");
    debug_assert_ne!(self.dim.1, 0, "dim.1 must be a positive integer");

    #[cfg(debug_assertions)]
    if let Some((width, height)) = self.gap {
      if let Some(value) = width {
        debug_assert!(value > 0f32, "gap.0 must be a positive value");
      }

      if let Some(value) = height {
        debug_assert!(value > 0f32, "gap.1 must be a positive value");
      }
    }

    #[cfg(debug_assertions)]
    if let Some((width, height)) = self.size {
      if let Some(value) = width {
        debug_assert!(value > 0f32, "size.0 must be a positive value");
      }

      if let Some(value) = height {
        debug_assert!(value > 0f32, "size.1 must be a positive value");
      }
    }

    let size = self.size.map_or(constraint.size, |(width, height)| {
      (
        width.unwrap_or(constraint.size.0).min(constraint.size.0),
        height.unwrap_or(constraint.size.1).min(constraint.size.1),
      )
    });

    const DEFAULT_GAP_SIZE: f32 = 4f32;

    let gap = self
      .gap
      .map_or((DEFAULT_GAP_SIZE, DEFAULT_GAP_SIZE), |(width, height)| {
        (width.unwrap_or(DEFAULT_GAP_SIZE), height.unwrap_or(DEFAULT_GAP_SIZE))
      });

    let cell_size = (
      (size.0 - gap.0 * (self.dim.0 - 1) as f32) / self.dim.0 as f32,
      (size.1 - gap.1 * (self.dim.1 - 1) as f32) / self.dim.1 as f32,
    );

    let position = (
      constraint.position.0 + (constraint.size.0 - size.0) * 0.5f32,
      constraint.position.1 + (constraint.size.1 - size.1) * 0.5f32,
    );

    for i in 0..self.dim.0 * self.dim.1 {
      (self.maker)(i).draw(
        context,
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
