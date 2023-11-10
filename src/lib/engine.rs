use crate::{
  common::Sharable,
  layouts::{stateful_layout::State, MultiChildLayout, StatefulLayout, StatelessLayout},
  models::Box2D,
  Context, View,
};
use sdl2::event::Event;
use skia_safe::Canvas;
use std::{
  cell::{RefCell, RefMut},
  collections::HashMap,
  rc::Rc,
};

const NO_CONSTRAINT: Box2D = Box2D {
  position: (0f32, 0f32),
  size: (f32::MAX, f32::MAX),
};

#[derive(Default)]
pub(super) struct Engine {
  state_map: HashMap<String, Rc<RefCell<dyn State>>>,
}

impl Engine {
  pub(super) fn get_state(&mut self, layout: &mut (impl StatefulLayout + ?Sized)) -> Rc<RefCell<dyn State>> {
    Rc::clone(
      self
        .state_map
        .entry(layout.get_key().to_owned())
        .or_insert_with(|| layout.make_state()),
    )
  }

  pub(super) fn draw_view(&mut self, view: &mut View, canvas: &Canvas, constraint: Box2D) {
    match view {
      View::StatelessLayout(layout) => self.draw_stateless_layout(&mut **layout, canvas, constraint),
      View::StatefulLayout(layout) => self.draw_stateful_layout(&mut **layout, canvas, constraint),
      View::MultiChildLayout(layout) => self.draw_multi_child_layout(&mut **layout, canvas, constraint),
      View::Node(node) => node.draw(canvas, constraint),
    }
  }

  fn draw_stateless_layout(&mut self, layout: &mut dyn StatelessLayout, canvas: &Canvas, constraint: Box2D) {
    match layout.make(constraint) {
      Some(Sharable::Owned(mut child)) => {
        layout.pre_draw(canvas, constraint);
        self.draw_view(&mut child, canvas, constraint);
        layout.post_draw(canvas, constraint);
      },
      Some(Sharable::Shared(child)) => {
        layout.pre_draw(canvas, constraint);
        self.draw_view(&mut child.borrow_mut(), canvas, constraint);
        layout.post_draw(canvas, constraint);
      },
      None => {},
    }
  }

  fn draw_stateful_layout(&mut self, layout: &mut dyn StatefulLayout, canvas: &Canvas, constraint: Box2D) {
    let state = self
      .state_map
      .entry(layout.get_key().to_owned())
      .or_insert_with(|| layout.make_state())
      .borrow();

    match state.make(constraint) {
      Some(Sharable::Owned(mut child)) => {
        state.pre_draw(canvas, constraint);
        drop(state);
        self.draw_view(&mut child, canvas, constraint);

        self
          .state_map
          .get(&layout.get_key().to_owned())
          .unwrap()
          .borrow()
          .post_draw(canvas, constraint);
      },
      Some(Sharable::Shared(child)) => {
        state.pre_draw(canvas, constraint);
        drop(state);
        self.draw_view(&mut child.borrow_mut(), canvas, constraint);

        self
          .state_map
          .get(&layout.get_key().to_owned())
          .unwrap()
          .borrow()
          .post_draw(canvas, constraint);
      },
      None => {},
    }
  }

  fn draw_multi_child_layout(&mut self, layout: &mut dyn MultiChildLayout, canvas: &Canvas, constraint: Box2D) {
    layout.pre_draw(canvas, constraint);

    let mut child_constraint = constraint;

    for child in layout.make(constraint) {
      match child {
        Sharable::Owned(mut child) => {
          self.draw_view(&mut child, canvas, child_constraint);

          // Tell the layout to reduce the constraint for the next child
          child_constraint = layout.calc_rect_left(child_constraint, &child);
        },
        Sharable::Shared(child) => {
          let mut child = child.borrow_mut();
          self.draw_view(&mut child, canvas, child_constraint);

          // Tell the layout to reduce the constraint for the next child
          child_constraint = layout.calc_rect_left(child_constraint, &child);
        },
      }
    }

    layout.post_draw(canvas, constraint);
  }

  pub(super) fn on_event(this: RefMut<Self>, view: &mut View, context: &Context, event: &Event) {
    match view {
      View::StatelessLayout(layout) => Engine::on_event_in_stateless_layout(this, &mut **layout, context, event),
      View::StatefulLayout(layout) => Engine::on_event_in_stateful_layout(this, &mut **layout, context, event),
      View::MultiChildLayout(layout) => Engine::on_event_in_multi_child_layout(this, &mut **layout, context, event),
      View::Node(node) => node.on_event(context, event),
    }
  }

  fn on_event_in_stateless_layout(
    this: RefMut<Self>,
    layout: &mut dyn StatelessLayout,
    context: &Context,
    event: &Event,
  ) {
    match layout.make(NO_CONSTRAINT) {
      Some(Sharable::Owned(mut child)) => Engine::on_event(this, &mut child, context, event),
      Some(Sharable::Shared(child)) => Engine::on_event(this, &mut child.borrow_mut(), context, event),
      None => {},
    }

    layout.on_event(context, event);
  }

  fn on_event_in_stateful_layout(
    mut this: RefMut<Self>,
    layout: &mut dyn StatefulLayout,
    context: &Context,
    event: &Event,
  ) {
    let state = this
      .state_map
      .entry(layout.get_key().to_owned())
      .or_insert_with(|| layout.make_state())
      .borrow();

    match state.make(NO_CONSTRAINT) {
      Some(Sharable::Owned(mut child)) => {
        drop(state);
        Engine::on_event(this, &mut child, context, event);
      },
      Some(Sharable::Shared(child)) => {
        drop(state);
        Engine::on_event(this, &mut child.borrow_mut(), context, event);
      },
      None => drop(state),
    }

    let state = Rc::clone(
      context
        .get_engine()
        .borrow_mut()
        .state_map
        .get(&layout.get_key().to_owned())
        .unwrap(),
    );

    // Drop the mutable borrow of engine from the given context here, because later on_event(context, event) call might
    // might mutably borrow this engine

    state.borrow_mut().on_event(context, event);
  }

  fn on_event_in_multi_child_layout(
    this: RefMut<Self>,
    layout: &mut dyn MultiChildLayout,
    context: &Context,
    event: &Event,
  ) {
    // Later we will mutably borrow the engine from the given context, so drop "this" which is a mutable borrow of this
    // engine to avoid multiple mutable borrows that is prohibited.
    drop(this);
    let engine = context.get_engine();

    for child in layout.make(NO_CONSTRAINT) {
      let engine = engine.borrow_mut();

      match child {
        Sharable::Owned(mut child) => Engine::on_event(engine, &mut child, context, event),
        Sharable::Shared(child) => Engine::on_event(engine, &mut child.borrow_mut(), context, event),
      }
    }

    layout.on_event(context, event);
  }

  pub(super) fn tick(this: RefMut<Self>, view: &mut View, context: &Context, dt: f32) {
    match view {
      View::StatelessLayout(layout) => Engine::tick_in_stateless_layout(this, &mut **layout, context, dt),
      View::StatefulLayout(layout) => Engine::tick_in_stateful_layout(this, &mut **layout, context, dt),
      View::MultiChildLayout(layout) => Engine::tick_in_multi_child_layout(this, &mut **layout, context, dt),
      View::Node(node) => node.tick(context, dt),
    }
  }

  fn tick_in_stateless_layout(this: RefMut<Self>, layout: &mut dyn StatelessLayout, context: &Context, dt: f32) {
    match layout.make(NO_CONSTRAINT) {
      Some(Sharable::Owned(mut child)) => Engine::tick(this, &mut child, context, dt),
      Some(Sharable::Shared(child)) => Engine::tick(this, &mut child.borrow_mut(), context, dt),
      None => {},
    }

    layout.tick(context, dt);
  }

  fn tick_in_stateful_layout(mut this: RefMut<Self>, layout: &mut dyn StatefulLayout, context: &Context, dt: f32) {
    let state = this
      .state_map
      .entry(layout.get_key().to_owned())
      .or_insert_with(|| layout.make_state())
      .borrow();

    match state.make(NO_CONSTRAINT) {
      Some(Sharable::Owned(mut child)) => {
        drop(state);
        Engine::tick(this, &mut child, context, dt);
      },
      Some(Sharable::Shared(child)) => {
        drop(state);
        Engine::tick(this, &mut child.borrow_mut(), context, dt);
      },
      None => drop(state),
    }

    let state = Rc::clone(
      context
        .get_engine()
        .borrow()
        .state_map
        .get(&layout.get_key().to_owned())
        .unwrap(),
    );

    // Drop the mutable borrow of engine from the given context here, because later on_event(context, event) call might
    // might mutably borrow this engine

    state.borrow_mut().tick(context, dt);
  }

  fn tick_in_multi_child_layout(this: RefMut<Self>, layout: &mut dyn MultiChildLayout, context: &Context, dt: f32) {
    // Later we will mutably borrow the engine from the given context, so drop "this" which is a mutable borrow of this
    // engine to avoid multiple mutable borrows that is prohibited.
    drop(this);
    let engine = context.get_engine();

    for child in layout.make(NO_CONSTRAINT) {
      let engine = engine.borrow_mut();

      match child {
        Sharable::Owned(mut child) => Engine::tick(engine, &mut child, context, dt),
        Sharable::Shared(child) => Engine::tick(engine, &mut child.borrow_mut(), context, dt),
      }
    }

    layout.tick(context, dt);
  }
}
