#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Box2D {
  pub position: (f32, f32),
  pub size: (f32, f32),
}
