use crate::common::asset_loader;
use sdl2::mixer::{Channel, Chunk};
use std::collections::HashMap;

#[derive(Default, PartialEq)]
pub struct Context {
  sounds: Option<HashMap<String, Chunk>>,
}

impl Context {
  pub(super) const fn new() -> Self {
    Self { sounds: None }
  }

  pub(super) fn init_audio(&mut self) {
    self.sounds = Some(asset_loader::load_sounds("assets/sounds/"));
  }

  pub fn play_sound(&self, name: &str) {
    Channel::all().play(&self.sounds.as_ref().unwrap()[name], 0).unwrap();
  }
}
