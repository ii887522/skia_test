use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sharable<T> {
  Owned(T),
  Shared(Rc<RefCell<T>>),
}

impl<T> From<T> for Sharable<T> {
  fn from(value: T) -> Self {
    Sharable::Owned(value)
  }
}

impl<T> From<&Rc<RefCell<T>>> for Sharable<T> {
  fn from(value: &Rc<RefCell<T>>) -> Self {
    Sharable::Shared(Rc::clone(value))
  }
}
