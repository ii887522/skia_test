use crate::models::{Snake, SnakePart};
use sdl2::{event::Event, keyboard::Keycode};
use skia_safe::Color;
use skia_test::{
  common::{Clock, Sharable, SparseSet},
  layouts::{stateful_layout::State, StatefulLayout},
  models::{direction::DIRECTIONS, Box2D, Direction},
  nodes::{BoxNode, GridNode},
  view::IntoViewFromNode,
  Context, View,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
use tinyrand::RandRange;
use tinyrand_std::thread_rand;

const DIM: u16 = 31; // Follow the dimension of the below data grid

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

// Sound assets
const DIE_SOUND: &str = "die";
const EAT_SOUND: &str = "eat";
const TURN_SOUND: &str = "turn";

#[derive(Default)]
pub(crate) struct SnakeGrid {
  pub on_die: Option<Box<dyn FnMut()>>,
}

impl StatefulLayout for SnakeGrid {
  fn get_key(&self) -> &str {
    "layouts/snake_grid"
  }

  fn make_state(&mut self) -> Rc<RefCell<dyn State>> {
    Rc::new(RefCell::new(SnakeGridState::new(self.on_die.take())))
  }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct SnakeGridState<OnDie> {
  snake: Snake,
  change_snake_direction: bool,
  clock: Clock,
  data: Rc<RefCell<Vec<u8>>>,
  air_indices: SparseSet<u16>,
  on_die: Option<OnDie>,
}

impl<OnDie> Default for SnakeGridState<OnDie> {
  fn default() -> Self {
    Self::new(None)
  }
}

impl<OnDie> SnakeGridState<OnDie> {
  fn new(on_die: Option<OnDie>) -> Self {
    // Initialize snake moving direction
    let snake_direction = DIRECTIONS[thread_rand().next_range(0..DIRECTIONS.len())];

    #[rustfmt::skip]
    let data = Rc::new(RefCell::new(vec![ // Make sure DIM constant above follows the dimension of this data grid
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
    ]));

    let mut this = Self {
      snake: Snake {
        head: SnakePart {
          position: (DIM >> 1) * DIM + (DIM >> 1),
          direction: snake_direction,
        },
        joint_queue: VecDeque::new(),
        last: SnakePart {
          position: (DIM >> 1) * DIM + (DIM >> 1),
          direction: snake_direction,
        },
      },
      change_snake_direction: true,
      clock: Clock::new(0.05f32),
      data,
      air_indices: (1u16..DIM - 1u16)
        .flat_map(move |x| (1u16..DIM - 1u16).map(move |y| (x, y)))
        .map(|(x, y)| x * DIM + y)
        .collect::<_>(),
      on_die,
    };

    // The center of the data grid is already allocated by the snake, so remove this data grid index from the free list.
    this.air_indices.remove((DIM >> 1) * DIM + (DIM >> 1));

    this.spawn_food();
    this
  }

  fn spawn_food(&mut self) {
    // Spawn a food at a random free location
    self.data.borrow_mut()[self.air_indices.remove_random_key() as usize] = FOOD;
  }
}

impl<OnDie: FnMut()> State for SnakeGridState<OnDie> {
  fn on_event(&mut self, context: &mut Context, event: &Event) {
    if !self.clock.is_running() || !self.change_snake_direction {
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

    self.clock.advance(dt, |clock| {
      // Reset this variable so that can change the snake direction in the next frame
      self.change_snake_direction = true;

      let mut data = self.data.borrow_mut();

      // Snake will hit an obstacle ?
      if self.snake.head.direction == Direction::Up && (data[(self.snake.head.position - DIM) as usize] == WALL ||
        data[(self.snake.head.position - DIM) as usize] == SNAKE) // Snake will hit the obstacle above ?
        || self.snake.head.direction == Direction::Right && (data[(self.snake.head.position + 1) as usize] == WALL ||
          data[(self.snake.head.position + 1) as usize] == SNAKE) // Snake will hit the right obstacle ?
        || self.snake.head.direction == Direction::Down && (data[(self.snake.head.position + DIM) as usize] == WALL ||
          data[(self.snake.head.position + DIM) as usize] == SNAKE) // Snake will hit the obstacle below ?
        || self.snake.head.direction == Direction::Left && (data[(self.snake.head.position - 1) as usize] == WALL ||
          data[(self.snake.head.position - 1) as usize] == SNAKE)
      // Snake will hit the left obstacle ?
      {
        // Game over
        clock.pause();
        context.play_sound(DIE_SOUND);
        if let Some(on_die) = &mut self.on_die {
          on_die();
        }

        return;
      }

      // Snake will eat the food ?
      if self.snake.head.direction == Direction::Up && data[(self.snake.head.position - DIM) as usize] == FOOD
        || self.snake.head.direction == Direction::Right && data[(self.snake.head.position + 1) as usize] == FOOD
        || self.snake.head.direction == Direction::Down && data[(self.snake.head.position + DIM) as usize] == FOOD
        || self.snake.head.direction == Direction::Left && data[(self.snake.head.position - 1) as usize] == FOOD
      {
        // Grow the snake tail
        is_food_eaten = true;
      } else {
        // Move the snake last in the pre-determined direction
        data[self.snake.last.position as usize] = AIR;
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

      data[self.snake.head.position as usize] = SNAKE;
      self.air_indices.remove(self.snake.head.position);
    });

    if is_food_eaten {
      self.spawn_food();
      context.play_sound(EAT_SOUND);
    }
  }

  fn make(&self, constraint: Box2D) -> Option<Sharable<View>> {
    GridNode {
      dim: (DIM as _, DIM as _),
      gap: (8f32, 8f32),
      size: (f32::MAX, constraint.size.0),
      maker: {
        let data = Rc::clone(&self.data);

        Box::new(move |index| {
          Box::new(BoxNode {
            color: COLORS[data.borrow()[index] as usize],
          })
        })
      },
    }
    .into_view()
  }
}
