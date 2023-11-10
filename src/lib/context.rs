use crate::{common::asset_loader, Engine};
use sdl2::mixer::{Channel, Chunk};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Context {
  engine: Rc<RefCell<Engine>>,
  sounds: RefCell<HashMap<String, Chunk>>,
}

impl Context {
  pub(super) fn init_audio(&self) {
    self.sounds.replace(asset_loader::load_sounds("assets/sounds/"));
  }

  pub(super) fn get_engine(&self) -> Rc<RefCell<Engine>> {
    Rc::clone(&self.engine)
  }

  pub fn play_sound(&self, name: &str) {
    Channel::all().play(&self.sounds.borrow()[name], 0).unwrap();
  }
}

thread_local! {
  pub static CONTEXT: Context = Context {
    engine: Rc::new(RefCell::new(Engine::default())),
    sounds: RefCell::new(HashMap::new()),
  }
}
