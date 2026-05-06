// Intentionally allow IdMap to depend on the private trait AstIdImpl,
// so ID types remain entirely opaque to code outside the ast module.
#![allow(private_bounds)]
use std::{marker::PhantomData, ops::Index};

use crate::parser::ast::id::AstIdImpl;

#[derive(Clone, Debug)]
pub struct IdMap<ID: AstIdImpl, T> {
  offset: ID,
  data: Vec<Option<T>>,
  phantom_data: PhantomData<ID>,
}

impl<ID: AstIdImpl, T> IdMap<ID, T> {
  fn idx(&self, id: ID) -> usize {
    debug_assert!(id.as_index() >= self.offset.as_index());
    id.as_index() - self.offset.as_index()
  }

  pub fn get(&self, id: ID) -> Option<&T> {
    self.data[self.idx(id)].as_ref()
  }

  pub fn get_mut(&mut self, id: ID) -> Option<&mut T> {
    let index = self.idx(id);
    self.data[index].as_mut()
  }

  pub fn insert(&mut self, id: ID, value: T) {
    let index = self.idx(id);
    self.data[index] = Some(value);
  }

  pub(in crate::parser::ast::id) fn from_range(start: ID, end: ID) -> Self {
    Self {
      offset: start,
      data: (start.as_index()..end.as_index()).map(|_| None).collect(),
      phantom_data: PhantomData,
    }
  }
}

impl<ID: AstIdImpl, T> Index<ID> for IdMap<ID, T> {
  type Output = T;

  fn index(&self, id: ID) -> &Self::Output {
    self.data[self.idx(id)]
      .as_ref()
      .expect("Index called on unpopulated ID map")
  }
}

pub struct IdMapIterator<ID: AstIdImpl, T> {
  offset: ID,
  iter: std::iter::Enumerate<std::vec::IntoIter<Option<T>>>,
  phantom_data: PhantomData<ID>,
}

impl<ID: AstIdImpl, T> Iterator for IdMapIterator<ID, T> {
  type Item = (ID, T);

  fn next(&mut self) -> Option<Self::Item> {
    for (index, value) in &mut self.iter {
      if let Some(value) = value {
        return Some((ID::from_index(index + self.offset.as_index()), value));
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
      offset: self.offset,
      iter: self.data.into_iter().enumerate(),
      phantom_data: PhantomData,
    }
  }
}

#[cfg(test)]
mod test {
  use googletest::prelude::*;

  use crate::parser::ast::id::{adt::map::IdMap, builder::IdBuilder, def::AstExpressionId};

  #[gtest]
  fn test_get_set() {
    let mut ids = IdBuilder::default();
    let start = ids.snapshot();
    let id0 = ids.new_expr_id();
    let id1 = ids.new_expr_id();
    let mut m: IdMap<AstExpressionId, u32> = start.to(ids.snapshot()).new_expression_id_map();
    m.insert(id0, 0);
    m.insert(id1, 1);
    expect_that!(m.get(id0), pat!(Some(eq(&0))));
    expect_that!(m.get(id1), pat!(Some(eq(&1))));
  }

  #[gtest]
  fn test_get_set_positive_offset() {
    let mut ids = IdBuilder::default();
    let _ = ids.new_expr_id();
    let _ = ids.new_expr_id();
    let start = ids.snapshot();
    let id1 = ids.new_expr_id();
    let id2 = ids.new_expr_id();
    let end = ids.snapshot();
    let mut m: IdMap<AstExpressionId, u32> = start.to(end).new_expression_id_map();
    m.insert(id1, 0);
    m.insert(id2, 1);
    expect_that!(m.get(id1), pat!(Some(eq(&0))));
    expect_that!(m.get(id2), pat!(Some(eq(&1))));
  }

  #[gtest]
  fn test_get_empty_returns_none() {
    let mut ids = IdBuilder::default();
    let start = ids.snapshot();
    let id0 = ids.new_expr_id();
    let end = ids.snapshot();
    let m: IdMap<AstExpressionId, u32> = start.to(end).new_expression_id_map();
    expect_true!(m.get(id0).is_none());
  }

  #[gtest]
  fn test_into_iter() {
    let mut ids = IdBuilder::default();
    let start = ids.snapshot();
    let id0 = ids.new_expr_id();
    let id1 = ids.new_expr_id();
    let end = ids.snapshot();
    let mut m: IdMap<AstExpressionId, u32> = start.to(end).new_expression_id_map();
    m.insert(id0, 0);
    m.insert(id1, 1);
    let v: Vec<(AstExpressionId, u32)> = m.into_iter().collect();

    expect_that!(v, elements_are![eq(&(id0, 0)), eq(&(id1, 1))])
  }
}
