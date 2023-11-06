use super::SnakePart;
use std::collections::VecDeque;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Snake {
  pub head: SnakePart,
  pub joint_queue: VecDeque<SnakePart>,
  pub last: SnakePart,
}
