#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod layouts;
mod models;
mod pages;

use pages::GamePage;
use skia_test::{
  layouts::{app, App},
  view::IntoViewFromStatefulLayout,
};

fn main() {
  app::run(App {
    title: "Snake",
    size: (830, 900),
    play_audio: true,
    child: GamePage.into_view(),
    ..Default::default()
  });
}

// TODO: Enhancement
// - Generate a key for each instance of view instead of manually specify the key
// - The data type of key should be usize instead of String
// - Engine::state_map should be Vec instead of HashMap
// - make() trait methods should only be called when state changes
//   - lazily call make() trait method
//   - caching if state not changed
//   - invalidate the cache if state changes
