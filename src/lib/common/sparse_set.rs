use std::fmt::Debug;
use tinyrand::RandRange;
use tinyrand_std::thread_rand;

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SparseSet<T> {
  dense: Vec<T>,
  sparse: Vec<Option<T>>,
}

impl<T: TryFrom<usize, Error = impl Debug> + Into<usize> + Copy> SparseSet<T> {
  pub fn add(&mut self, key: T) {
    if let Some(&Some(_)) = self.sparse.get(key.into()) {
      return;
    }

    self.dense.push(key);

    let key = key.into();

    // Allocate necessary memory for sparse array to avoid panic later due to index out of bounds
    if key >= self.sparse.len() {
      self.sparse.resize_with(key + 1, || None);
    }

    self.sparse[key] = Some((self.dense.len() - 1).try_into().unwrap());
  }

  pub fn remove_random_key(&mut self) -> T {
    let key = self.dense[thread_rand().next_range(0..self.dense.len())];
    self.remove(key);
    key
  }

  pub fn remove(&mut self, key: T) {
    let key = key.into();

    if let Some(&Some(index)) = self.sparse.get(key) {
      let index = index.into();
      self.sparse[key] = None;
      self.dense.swap_remove(index);

      if let Some(&key) = self.dense.get(index) {
        self.sparse[key.into()] = Some(index.try_into().unwrap());
      }
    }
  }
}

impl<T: TryFrom<usize, Error = impl Debug> + Into<usize> + Copy> FromIterator<T> for SparseSet<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let dense = iter.into_iter().collect::<Vec<_>>();
    let mut sparse = vec![];

    // Initialize sparse array
    for (i, &key) in dense.iter().enumerate() {
      let key = key.into();

      // Allocate necessary memory for sparse array to avoid panic later due to index out of bounds
      if key >= sparse.len() {
        sparse.resize_with(key + 1, || None);
      }

      sparse[key] = Some(i.try_into().unwrap());
    }

    Self { dense, sparse }
  }
}
