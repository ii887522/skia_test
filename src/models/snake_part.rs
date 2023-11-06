use skia_test::models::Direction;

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SnakePart {
  pub position: u16,
  pub direction: Direction,
}
