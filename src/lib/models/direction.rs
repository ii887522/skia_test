#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  #[default]
  Up,
  Right,
  Down,
  Left,
}

pub const DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Right, Direction::Down, Direction::Left];
