#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod views;

use skia_test::views::{app, App};
use views::SnakeGrid;

fn main() {
  app::run(App {
    title: "Snake",
    size: (1600, 900),
    color: None,
    child: SnakeGrid::new(),
  });
}
