#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

fn main() {
  app::run("Snake", 1600, 900);
}
