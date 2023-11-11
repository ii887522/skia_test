use crate::layouts::SnakeGrid;
use skia_test::{
  common::{Clock, Sharable},
  layouts::{stateful_layout::State, Shake, StatefulLayout},
  models::Box2D,
  view::IntoViewFromStatefulLayout,
  Context, View,
};
use std::{
  cell::{Cell, RefCell},
  rc::Rc,
};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GamePage;

impl StatefulLayout for GamePage {
  fn get_key(&self) -> &str {
    "pages/game_page"
  }

  fn make_state(&mut self) -> Rc<RefCell<dyn State>> {
    Rc::new(RefCell::new(GamePageState::new()))
  }
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
struct GamePageState {
  shake: Rc<Cell<bool>>,
  shake_clock: Clock,
}

impl GamePageState {
  fn new() -> Self {
    Self {
      shake: Rc::new(Cell::new(false)),
      shake_clock: Clock::new(0.25f32),
    }
  }
}

impl State for GamePageState {
  fn tick(&mut self, _context: &mut Context, dt: f32) {
    if self.shake.get() {
      self.shake_clock.advance(dt, |clock| {
        self.shake.set(false);
        clock.pause();
      });
    }
  }

  fn make(&self, _constraint: Box2D) -> Option<Sharable<View>> {
    Shake {
      is_enabled: Rc::clone(&self.shake),
      child: SnakeGrid {
        on_die: {
          let shake = Rc::clone(&self.shake);
          Some(Box::new(move || shake.set(true)))
        },
      }
      .into_view(),
    }
    .into_view()
  }
}
