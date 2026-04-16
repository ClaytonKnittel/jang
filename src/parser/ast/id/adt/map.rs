// Intentionally allow IdMap to depend on the private trait AstIdImpl,
// so ID types remain entirely opaque to code outside the ast module.
#![allow(private_bounds)]
use std::{marker::PhantomData, ops::Index};

use crate::parser::ast::id::AstIdImpl;

#[derive(Clone, Debug)]
pub struct IdMap<ID: AstIdImpl, T> {
  data: Vec<Option<T>>,
  phantom_data: PhantomData<ID>,
}

impl<ID: AstIdImpl, T: std::clone::Clone> IdMap<ID, T> {
  pub fn get(&self, id: ID) -> Option<&T> {
    self.data[id.as_index()].as_ref()
  }

  pub fn get_mut(&mut self, id: ID) -> Option<&mut T> {
    self.data[id.as_index()].as_mut()
  }

  pub fn insert(&mut self, id: ID, value: T) {
    self.data[id.as_index()] = Some(value);
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      data: vec![None; capacity],
      phantom_data: PhantomData,
    }
  }
}

impl<ID: AstIdImpl, T> Index<ID> for IdMap<ID, T> {
  type Output = T;

  fn index(&self, id: ID) -> &Self::Output {
    self.data[id.as_index()]
      .as_ref()
      .expect("Index called on unpopulated ID map")
  }
}

pub struct IdMapIterator<ID: AstIdImpl, T> {
  iter: std::iter::Enumerate<std::vec::IntoIter<Option<T>>>,
  phantom_data: PhantomData<ID>,
}

impl<ID: AstIdImpl, T> Iterator for IdMapIterator<ID, T> {
  type Item = (ID, T);

  fn next(&mut self) -> Option<Self::Item> {
    for (index, value) in &mut self.iter {
      if let Some(value) = value {
        return Some((ID::from_index(index), value));
      }
    }
    None
  }
}

impl<ID: AstIdImpl, T> IntoIterator for IdMap<ID, T> {
  type Item = (ID, T);
  type IntoIter = IdMapIterator<ID, T>;

  fn into_iter(self) -> Self::IntoIter {
    IdMapIterator {
      iter: self.data.into_iter().enumerate(),
      phantom_data: PhantomData,
    }
  }
}

#[cfg(test)]
mod test {
  use googletest::prelude::*;

  use crate::parser::ast::id::{AstIdImpl, adt::map::IdMap, def::AstExpressionId};

  #[gtest]
  fn test_get_set() {
    let mut m: IdMap<AstExpressionId, u32> = IdMap::with_capacity(3);
    let id0 = AstExpressionId::default();
    let id1 = id0.next_id();
    m.insert(id0, 0);
    m.insert(id1, 1);
    expect_that!(m.get(id0), pat!(Some(eq(&0))));
    expect_that!(m.get(id1), pat!(Some(eq(&1))));
  }

  #[gtest]
  fn test_get_empty_returns_none() {
    let m: IdMap<AstExpressionId, u32> = IdMap::with_capacity(3);
    expect_true!(m.get(AstExpressionId::default()).is_none());
  }

  #[gtest]
  fn test_into_iter() {
    let mut m: IdMap<AstExpressionId, u32> = IdMap::with_capacity(2);
    let id0 = AstExpressionId::default();
    let id1 = id0.next_id();
    m.insert(id0, 0);
    m.insert(id1, 1);
    let v: Vec<(AstExpressionId, u32)> = m.into_iter().collect();

    expect_that!(v, elements_are![eq(&(id0, 0)), eq(&(id1, 1))])
  }
}
