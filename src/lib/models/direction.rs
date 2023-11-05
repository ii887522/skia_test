#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

pub const DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Right, Direction::Down, Direction::Left];
