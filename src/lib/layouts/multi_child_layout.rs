use crate::{common::Sharable, models::Box2D, Context, View};
use sdl2::event::Event;
use skia_safe::Canvas;

pub trait MultiChildLayout {
  fn on_event(&mut self, _context: &Context, _event: &Event) {}
  fn tick(&mut self, _context: &Context, _dt: f32) {}

  fn calc_rect_left(&self, constraint: Box2D, _child: &View) -> Box2D {
    constraint
  }

  fn pre_draw(&self, _canvas: &Canvas, _constraint: Box2D) {}

  fn make(&self, _constraint: Box2D) -> Vec<Sharable<View>> {
    vec![]
  }

  fn post_draw(&self, _canvas: &Canvas, _constraint: Box2D) {}
}
