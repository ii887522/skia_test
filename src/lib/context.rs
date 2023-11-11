use crate::{common::asset_loader, Engine};
use sdl2::mixer::{Channel, Chunk};
use std::{cell::RefCell, collections::HashMap};

pub struct Context {
  engine: Engine,
  sounds: HashMap<String, Chunk>,
}

impl Context {
  pub(super) fn init_audio(&mut self) {
    self.sounds = asset_loader::load_sounds("assets/sounds/");
  }

  pub(super) fn get_engine(&mut self) -> &mut Engine {
    &mut self.engine
  }

  pub fn play_sound(&self, name: &str) {
    Channel::all().play(&self.sounds[name], 0).unwrap();
  }
}

thread_local! {
  pub static CONTEXT: RefCell<Context> = RefCell::new(Context {
    engine: Engine::default(),
    sounds: HashMap::new(),
  })
}
