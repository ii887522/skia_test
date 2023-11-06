use crate::models::{Snake, SnakePart};
use sdl2::{event::Event, keyboard::Keycode};
use skia_safe::{Canvas, Color};
use skia_test::{
  common::{SparseSet, Ticker},
  models::{direction::DIRECTIONS, Box2D, Direction},
  views::{BoxView, Grid, View},
  Context,
};
use std::{
  collections::VecDeque,
  fmt::{self, Debug, Formatter},
};
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

const DIM: u16 = 31; // Follow the dimension of the below data grid

const DIE_SOUND: &str = "die";
const EAT_SOUND: &str = "eat";
const TURN_SOUND: &str = "turn";

pub(crate) struct SnakeGrid {
  snake: Snake,
  change_snake_direction: bool,
  ticker: Ticker,
  data: Vec<u8>,
  air_indices: SparseSet<u16>,
  on_die: Box<dyn FnMut()>,
}

impl Debug for SnakeGrid {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    fmt
      .debug_struct("SnakeGrid")
      .field("snake", &self.snake)
      .field("change_snake_direction", &self.change_snake_direction)
      .field("ticker", &self.ticker)
      .field("data", &self.data)
      .field("air_indices", &self.air_indices)
      .finish_non_exhaustive()
  }
}

impl SnakeGrid {
  pub(crate) fn new(on_die: impl FnMut() + 'static) -> Self {
    // Initialize snake moving direction
    let snake_direction = DIRECTIONS[thread_rand().next_range(0..DIRECTIONS.len())];

    #[rustfmt::skip]
    let mut this = Self {
      snake: Snake {
        head: SnakePart { position: (DIM >> 1) * DIM + (DIM >> 1), direction: snake_direction },
        joint_queue: VecDeque::new(),
        last: SnakePart { position: (DIM >> 1) * DIM + (DIM >> 1), direction: snake_direction }
      },
      change_snake_direction: true,
      ticker: Ticker::new(0.05f32),
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
      air_indices: (1u16..DIM - 1u16)
        .flat_map(move |x| (1u16..DIM - 1u16).map(move |y| (x, y)))
        .map(|(x, y)| x * DIM + y)
        .collect::<_>(),
      on_die: Box::new(on_die),
    };

    // The center of the data grid is already allocated by the snake, so remove this data grid index from the free list.
    this.air_indices.remove((DIM >> 1) * DIM + (DIM >> 1));

    this.spawn_food();
    this
  }

  fn spawn_food(&mut self) {
    // Spawn a food at a random free location
    self.data[self.air_indices.remove_random_key() as usize] = FOOD;
  }
}

impl View for SnakeGrid {
  fn on_event(&mut self, context: &mut Context, event: &Event) {
    if !self.ticker.is_running() || !self.change_snake_direction {
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
          if self.snake.head.direction != Direction::Up && self.snake.head.direction != Direction::Down =>
        {
          self.snake.head.direction = Direction::Up;

          self.snake.joint_queue.push_back(SnakePart {
            position: self.snake.head.position,
            direction: Direction::Up,
          });

          context.play_sound(TURN_SOUND);
        },
        Keycode::D | Keycode::Right
          if self.snake.head.direction != Direction::Right && self.snake.head.direction != Direction::Left =>
        {
          self.snake.head.direction = Direction::Right;

          self.snake.joint_queue.push_back(SnakePart {
            position: self.snake.head.position,
            direction: Direction::Right,
          });

          context.play_sound(TURN_SOUND);
        },
        Keycode::S | Keycode::Down
          if self.snake.head.direction != Direction::Down && self.snake.head.direction != Direction::Up =>
        {
          self.snake.head.direction = Direction::Down;

          self.snake.joint_queue.push_back(SnakePart {
            position: self.snake.head.position,
            direction: Direction::Down,
          });

          context.play_sound(TURN_SOUND);
        },
        Keycode::A | Keycode::Left
          if self.snake.head.direction != Direction::Left && self.snake.head.direction != Direction::Right =>
        {
          self.snake.head.direction = Direction::Left;

          self.snake.joint_queue.push_back(SnakePart {
            position: self.snake.head.position,
            direction: Direction::Left,
          });

          context.play_sound(TURN_SOUND);
        },
        _ => {
          // Assumption above failed. Rollback this variable
          self.change_snake_direction = true;
        },
      }
    }
  }

  fn tick(&mut self, context: &mut Context, dt: f32) {
    let mut is_food_eaten = false;

    self.ticker.advance(dt, |ticker| {
      // Reset this variable so that can change the snake direction in the next frame
      self.change_snake_direction = true;

      // Snake will hit an obstacle ?
      if self.snake.head.direction == Direction::Up && (self.data[(self.snake.head.position - DIM) as usize] == WALL ||
        self.data[(self.snake.head.position - DIM) as usize] == SNAKE) // Snake will hit the obstacle above ?
        || self.snake.head.direction == Direction::Right && (self.data[(self.snake.head.position + 1) as usize] == WALL ||
          self.data[(self.snake.head.position + 1) as usize] == SNAKE) // Snake will hit the right obstacle ?
        || self.snake.head.direction == Direction::Down && (self.data[(self.snake.head.position + DIM) as usize] == WALL ||
          self.data[(self.snake.head.position + DIM) as usize] == SNAKE) // Snake will hit the obstacle below ?
        || self.snake.head.direction == Direction::Left && (self.data[(self.snake.head.position - 1) as usize] == WALL ||
          self.data[(self.snake.head.position - 1) as usize] == SNAKE)
      // Snake will hit the left obstacle ?
      {
        // Game over
        ticker.pause();
        context.play_sound(DIE_SOUND);
        (self.on_die)();
        return;
      }

      // Snake will eat the food ?
      if self.snake.head.direction == Direction::Up && self.data[(self.snake.head.position - DIM) as usize] == FOOD
        || self.snake.head.direction == Direction::Right && self.data[(self.snake.head.position + 1) as usize] == FOOD
        || self.snake.head.direction == Direction::Down && self.data[(self.snake.head.position + DIM) as usize] == FOOD
        || self.snake.head.direction == Direction::Left && self.data[(self.snake.head.position - 1) as usize] == FOOD
      {
        // Grow the snake tail
        is_food_eaten = true;
      } else {
        // Move the snake last in the pre-determined direction
        self.data[self.snake.last.position as usize] = AIR;
        self.air_indices.add(self.snake.last.position);

        if let Some(joint) = self.snake.joint_queue.front() {
          if self.snake.last.position == joint.position {
            self.snake.last.direction = joint.direction;
            self.snake.joint_queue.pop_front();
          }
        }

        self.snake.last.position = match self.snake.last.direction {
          Direction::Up => self.snake.last.position - DIM, // Set the upper cell to snake last
          Direction::Right => self.snake.last.position + 1, // Set the right cell to snake last
          Direction::Down => self.snake.last.position + DIM, // Set the lower cell to snake last
          Direction::Left => self.snake.last.position - 1, // Set the left cell to snake last
        }
      }

      // Move the snake head in the pre-determined direction
      self.snake.head.position = match self.snake.head.direction {
        Direction::Up => self.snake.head.position - DIM, // Set the upper cell to snake head
        Direction::Right => self.snake.head.position + 1, // Set the right cell to snake head
        Direction::Down => self.snake.head.position + DIM, // Set the lower cell to snake head
        Direction::Left => self.snake.head.position - 1, // Set the left cell to snake head
      };

      self.data[self.snake.head.position as usize] = SNAKE;
      self.air_indices.remove(self.snake.head.position);
    });

    if is_food_eaten {
      self.spawn_food();
      context.play_sound(EAT_SOUND);
    }
  }

  fn draw(&mut self, context: &mut Context, canvas: &Canvas, constraint: Box2D) {
    Grid {
      dim: (DIM as _, DIM as _),
      gap: Some((Some(8f32), Some(8f32))),
      size: Some((Some(constraint.size.1), None)),
      maker: |index| BoxView {
        color: COLORS[self.data[index] as usize],
      },
    }
    .draw(context, canvas, constraint);
  }
}
