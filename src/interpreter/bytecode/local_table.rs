use crate::interpreter::error::{InterpreterError, InterpreterResult};
use std::default::Default;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Default)]
pub struct LocalId(u32);

impl LocalId {
  pub fn next(self) -> Self {
    Self(self.0 + 1)
  }
}

pub enum LocalSlot<T> {
  Uninitialized,
  Val(T),
}

pub struct LocalTable<T> {
  slots: Vec<LocalSlot<T>>,
}

impl<T> LocalTable<T> {
  pub fn new() -> Self {
    Self { slots: Vec::new() }
  }

  pub fn read(&self, local_id: LocalId) -> InterpreterResult<&T> {
    match self.slots.get(local_id.0 as usize) {
      Some(LocalSlot::Val(value)) => Ok(value),
      Some(LocalSlot::Uninitialized) => {
        Err(InterpreterError::generic_err("uninitialized local").into())
      }
      None => Err(InterpreterError::generic_err("bad local read").into()),
    }
  }

  pub fn write(&mut self, local_id: LocalId, value: T) {
    let index = local_id.0 as usize;
    if self.slots.len() <= index {
      self
        .slots
        .resize_with(index + 1, || LocalSlot::Uninitialized);
    }
    self.slots[index] = LocalSlot::Val(value);
  }
}
