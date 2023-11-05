use skia_test::models::Direction;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SnakeJoint {
  pub direction: Direction,
  pub index: u16,
}
