use sdl2::{event::Event, keyboard::Keycode};
use skia_safe::{Canvas, Color};
use skia_test::{
  common::Ticker,
  models::{direction::DIRECTIONS, Box2D, Direction},
  views::{BoxView, Grid, View},
};
use tinyrand::RandRange;
use tinyrand_std::thread_rand;

// data[i] = 0 means air
// data[i] = 1 means wall
// data[i] = 2 means snake (the player)
// data[i] = 3 means food
//
// The indices of COLORS constant below represents the data[i] described above
const COLORS: &[Color] = &[Color::DARK_GRAY, Color::RED, Color::CYAN, Color::GREEN];

const DIM: usize = 31; // Follow the dimension of the below data grid

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct SnakeGrid {
  snake_position: usize,
  snake_direction: Direction,
  change_snake_direction: bool,
  ticker: Ticker,
  data: Vec<u8>,
}

impl SnakeGrid {
  #[rustfmt::skip]
  pub(crate) fn new() -> Self {
    Self {
      snake_position: (DIM >> 1) * DIM + (DIM >> 1),
      snake_direction: DIRECTIONS[thread_rand().next_range(0..DIRECTIONS.len())],
      change_snake_direction: true,
      ticker: Ticker::new(0.1f32),
      data: vec![ // Make sure DIM constant follows the dimension of this data grid
        1, 1, 1, 1, 1, 1, 1, 1,   1, 1, 1, 1, 1, 1, 1, 1,   1, 1, 1, 1, 1, 1, 1, 1,   1, 1, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,

        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 2,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,

        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,

        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 0, 0,   0, 0, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1,   1, 1, 1, 1, 1, 1, 1, 1,   1, 1, 1, 1, 1, 1, 1, 1,   1, 1, 1, 1, 1, 1, 1,
      ],
    }
  }
}

impl View for SnakeGrid {
  fn on_event(&mut self, event: &Event) {
    if !self.change_snake_direction {
      return;
    }

    if let Event::KeyDown {
      keycode: Some(keycode), ..
    } = event
    {
      // Assume the snake has already changed its moving direction
      self.change_snake_direction = false;

      self.snake_direction = match keycode {
        Keycode::W | Keycode::Up if self.snake_direction != Direction::Down => Direction::Up,
        Keycode::D | Keycode::Right if self.snake_direction != Direction::Left => Direction::Right,
        Keycode::S | Keycode::Down if self.snake_direction != Direction::Up => Direction::Down,
        Keycode::A | Keycode::Left if self.snake_direction != Direction::Right => Direction::Left,
        _ => {
          // Assumption above failed. Rollback this variable
          self.change_snake_direction = true;

          self.snake_direction
        },
      }
    }
  }

  fn tick(&mut self, dt: f32) {
    self.ticker.advance(dt, |ticker| {
      // Reset this variable so that can change the snake direction in the next frame
      self.change_snake_direction = true;

      if self.snake_direction == Direction::Up && self.data[self.snake_position - DIM] == 1 // Snake will hit the top wall ?
        || self.snake_direction == Direction::Right && self.data[self.snake_position + 1] == 1 // Snake will hit the right wall ?
        || self.snake_direction == Direction::Down && self.data[self.snake_position + DIM] == 1 // Snake will hit the bottom wall ?
        || self.snake_direction == Direction::Left && self.data[self.snake_position - 1] == 1
      // Snake will hit the left wall ?
      {
        // Game over
        ticker.pause();
        return;
      }

      // Move the snake in the pre-determined direction
      self.data[self.snake_position] = 0; // Set the current snake position to air
      self.snake_position = match self.snake_direction {
        Direction::Up => self.snake_position - DIM,   // Set the upper cell to snake
        Direction::Right => self.snake_position + 1,  // Set the right cell to snake
        Direction::Down => self.snake_position + DIM, // Set the lower cell to snake
        Direction::Left => self.snake_position - 1,   // Set the left cell to snake
      };
      self.data[self.snake_position] = 2;
    });
  }

  fn draw(&mut self, canvas: &Canvas, constraint: Box2D) {
    Grid {
      dim: (DIM, DIM),
      gap: Some((Some(8f32), Some(8f32))),
      size: Some((Some(constraint.size.1), None)),
      maker: |index| BoxView {
        color: COLORS[self.data[index] as usize],
      },
    }
    .draw(canvas, constraint);
  }
}
