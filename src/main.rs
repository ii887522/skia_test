#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod pages;
mod views;

use pages::GamePage;
use skia_test::views::{app, App};

fn main() {
  app::run(App {
    title: "Snake",
    size: (1600, 900),
    color: None,
    play_audio: true,
    child: GamePage::new(),
  });
}
