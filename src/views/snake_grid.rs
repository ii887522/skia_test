use crate::models::SnakeJoint;
use sdl2::{event::Event, keyboard::Keycode};
use skia_safe::{Canvas, Color};
use skia_test::{
  common::{SparseSet, Ticker},
  models::{direction::DIRECTIONS, Box2D, Direction},
  views::{BoxView, Grid, View},
};
use std::collections::VecDeque;
use tinyrand::RandRange;
use tinyrand_std::thread_rand;

// data[i] = 0 means air
// data[i] = 1 means wall
// data[i] = 2 means snake (the player)
// data[i] = 3 means food
const AIR: u8 = 0;
const WALL: u8 = 1;
const SNAKE: u8 = 2;
const FOOD: u8 = 3;

// The indices of COLORS constant below represents the data[i] described above
const COLORS: &[Color] = &[Color::DARK_GRAY, Color::RED, Color::CYAN, Color::GREEN];

const DIM: usize = 31; // Follow the dimension of the below data grid

#[derive(Debug, PartialEq)]
pub(crate) struct SnakeGrid {
  snake_head_position: usize,
  snake_last_position: usize,
  snake_head_direction: Direction,
  snake_last_direction: Direction,
  change_snake_direction: bool,
  ticker: Ticker,
  data: Vec<u8>,
  snake_joint_queue: VecDeque<SnakeJoint>,
  air_indices: SparseSet<u16>,
}

impl SnakeGrid {
  pub(crate) fn new() -> Self {
    // Initialize snake moving direction
    let snake_direction = DIRECTIONS[thread_rand().next_range(0..DIRECTIONS.len())];

    #[rustfmt::skip]
    let mut this = Self {
      snake_head_position: (DIM >> 1) * DIM + (DIM >> 1),
      snake_last_position: (DIM >> 1) * DIM + (DIM >> 1),
      snake_head_direction: snake_direction,
      snake_last_direction: snake_direction,
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
      snake_joint_queue: VecDeque::new(),
      air_indices: (1u16..DIM as u16 - 1u16)
        .flat_map(move |x| (1u16..DIM as u16 - 1u16).map(move |y| (x, y)))
        .map(|(x, y)| x * DIM as u16 + y)
        .collect::<_>(),
    };

    // The center of the data grid is already allocated by the snake, so remove this data grid index from the free list.
    this.air_indices.remove(((DIM >> 1) * DIM + (DIM >> 1)) as u16);

    this.spawn_food();
    this
  }

  fn spawn_food(&mut self) {
    // Spawn a food at a random free location
    self.data[self.air_indices.remove_random_key() as usize] = FOOD;
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

      match keycode {
        Keycode::W | Keycode::Up
          if self.snake_head_direction != Direction::Up && self.snake_head_direction != Direction::Down =>
        {
          self.snake_head_direction = Direction::Up;

          self.snake_joint_queue.push_back(SnakeJoint {
            direction: Direction::Up,
            index: self.snake_head_position as _,
          });
        },
        Keycode::D | Keycode::Right
          if self.snake_head_direction != Direction::Right && self.snake_head_direction != Direction::Left =>
        {
          self.snake_head_direction = Direction::Right;

          self.snake_joint_queue.push_back(SnakeJoint {
            direction: Direction::Right,
            index: self.snake_head_position as _,
          });
        },
        Keycode::S | Keycode::Down
          if self.snake_head_direction != Direction::Down && self.snake_head_direction != Direction::Up =>
        {
          self.snake_head_direction = Direction::Down;

          self.snake_joint_queue.push_back(SnakeJoint {
            direction: Direction::Down,
            index: self.snake_head_position as _,
          });
        },
        Keycode::A | Keycode::Left
          if self.snake_head_direction != Direction::Left && self.snake_head_direction != Direction::Right =>
        {
          self.snake_head_direction = Direction::Left;

          self.snake_joint_queue.push_back(SnakeJoint {
            direction: Direction::Left,
            index: self.snake_head_position as _,
          });
        },
        _ => {
          // Assumption above failed. Rollback this variable
          self.change_snake_direction = true;
        },
      }
    }
  }

  fn tick(&mut self, dt: f32) {
    let mut is_food_eaten = false;

    self.ticker.advance(dt, |ticker| {
      // Reset this variable so that can change the snake direction in the next frame
      self.change_snake_direction = true;

      // Snake will hit an obstacle ?
      if self.snake_head_direction == Direction::Up && (self.data[self.snake_head_position - DIM] == WALL ||
        self.data[self.snake_head_position - DIM] == SNAKE) // Snake will hit the obstacle above ?
        || self.snake_head_direction == Direction::Right && (self.data[self.snake_head_position + 1] == WALL ||
          self.data[self.snake_head_position + 1] == SNAKE) // Snake will hit the right obstacle ?
        || self.snake_head_direction == Direction::Down && (self.data[self.snake_head_position + DIM] == WALL ||
          self.data[self.snake_head_position + DIM] == SNAKE) // Snake will hit the obstacle below ?
        || self.snake_head_direction == Direction::Left && (self.data[self.snake_head_position - 1] == WALL ||
          self.data[self.snake_head_position - 1] == SNAKE)
      // Snake will hit the left obstacle ?
      {
        // Game over
        ticker.pause();
        return;
      }

      // Snake will eat the food ?
      if self.snake_head_direction == Direction::Up && self.data[self.snake_head_position - DIM] == FOOD
        || self.snake_head_direction == Direction::Right && self.data[self.snake_head_position + 1] == FOOD
        || self.snake_head_direction == Direction::Down && self.data[self.snake_head_position + DIM] == FOOD
        || self.snake_head_direction == Direction::Left && self.data[self.snake_head_position - 1] == FOOD
      {
        // Grow the snake tail
        is_food_eaten = true;
      } else {
        // Move the snake last in the pre-determined direction
        self.data[self.snake_last_position] = AIR;
        self.air_indices.add(self.snake_last_position as _);

        if let Some(joint) = self.snake_joint_queue.front() {
          if self.snake_last_position == joint.index as _ {
            self.snake_last_direction = joint.direction;
            self.snake_joint_queue.pop_front();
          }
        }

        self.snake_last_position = match self.snake_last_direction {
          Direction::Up => self.snake_last_position - DIM, // Set the upper cell to snake last
          Direction::Right => self.snake_last_position + 1, // Set the right cell to snake last
          Direction::Down => self.snake_last_position + DIM, // Set the lower cell to snake last
          Direction::Left => self.snake_last_position - 1, // Set the left cell to snake last
        }
      }

      // Move the snake head in the pre-determined direction
      self.snake_head_position = match self.snake_head_direction {
        Direction::Up => self.snake_head_position - DIM, // Set the upper cell to snake head
        Direction::Right => self.snake_head_position + 1, // Set the right cell to snake head
        Direction::Down => self.snake_head_position + DIM, // Set the lower cell to snake head
        Direction::Left => self.snake_head_position - 1, // Set the left cell to snake head
      };

      self.data[self.snake_head_position] = SNAKE;
      self.air_indices.remove(self.snake_head_position as _);
    });

    if is_food_eaten {
      self.spawn_food();
    }
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
