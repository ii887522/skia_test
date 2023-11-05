#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  UP,
  RIGHT,
  DOWN,
  LEFT,
}

pub const DIRECTIONS: &[Direction] = &[Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT];
