use crate::{
  common::Sharable,
  layouts::{MultiChildLayout, StatefulLayout, StatelessLayout},
  nodes::{BoxNode, Node},
};

pub enum View {
  StatelessLayout(Box<dyn StatelessLayout>),
  StatefulLayout(Box<dyn StatefulLayout>),
  MultiChildLayout(Box<dyn MultiChildLayout>),
  Node(Box<dyn Node>),
}

impl Default for View {
  fn default() -> Self {
    View::Node(Box::<BoxNode>::default())
  }
}

pub trait FromStatelessLayout<T> {
  fn from(value: T) -> Self;
}

pub trait IntoViewFromStatelessLayout {
  fn into_view(self) -> Option<Sharable<View>>;
}

impl<T: StatelessLayout + 'static> FromStatelessLayout<T> for Option<Sharable<View>> {
  fn from(value: T) -> Self {
    Some(Sharable::Owned(View::StatelessLayout(Box::new(value))))
  }
}

impl<T: StatelessLayout + 'static> IntoViewFromStatelessLayout for T {
  fn into_view(self) -> Option<Sharable<View>> {
    <Option<Sharable<View>> as FromStatelessLayout<T>>::from(self)
  }
}

pub trait FromStatefulLayout<T> {
  fn from(value: T) -> Self;
}

pub trait IntoViewFromStatefulLayout {
  fn into_view(self) -> Option<Sharable<View>>;
}

impl<T: StatefulLayout + 'static> FromStatefulLayout<T> for Option<Sharable<View>> {
  fn from(value: T) -> Self {
    Some(Sharable::Owned(View::StatefulLayout(Box::new(value))))
  }
}

impl<T: StatefulLayout + 'static> IntoViewFromStatefulLayout for T {
  fn into_view(self) -> Option<Sharable<View>> {
    <Option<Sharable<View>> as FromStatefulLayout<T>>::from(self)
  }
}

pub trait FromMultiChildLayout<T> {
  fn from(value: T) -> Self;
}

pub trait IntoViewFromMultiChildLayout {
  fn into_view(self) -> Option<Sharable<View>>;
}

impl<T: MultiChildLayout + 'static> FromMultiChildLayout<T> for Option<Sharable<View>> {
  fn from(value: T) -> Self {
    Some(Sharable::Owned(View::MultiChildLayout(Box::new(value))))
  }
}

impl<T: MultiChildLayout + 'static> IntoViewFromMultiChildLayout for T {
  fn into_view(self) -> Option<Sharable<View>> {
    <Option<Sharable<View>> as FromMultiChildLayout<T>>::from(self)
  }
}

pub trait FromNode<T> {
  fn from(value: T) -> Self;
}

pub trait IntoViewFromNode {
  fn into_view(self) -> Option<Sharable<View>>;
}

impl<T: Node + 'static> FromNode<T> for Option<Sharable<View>> {
  fn from(value: T) -> Self {
    Some(Sharable::Owned(View::Node(Box::new(value))))
  }
}

impl<T: Node + 'static> IntoViewFromNode for T {
  fn into_view(self) -> Option<Sharable<View>> {
    <Option<Sharable<View>> as FromNode<T>>::from(self)
  }
}
