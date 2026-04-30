use std::cell::{RefCell, RefMut};

/// A basic arena type that safely stores items
/// and provides stable references to them.
pub struct Arena<T> {
  chunks: RefCell<Vec<Chunk<T>>>,
}

/// A fixed-capacity chunk of values.
struct Chunk<T> {
  values: Vec<T>,
}

const CHUNK_SIZE: usize = 512;

impl<T> Default for Chunk<T> {
  fn default() -> Self {
    Self {
      values: Vec::with_capacity(CHUNK_SIZE),
    }
  }
}

impl<T> Chunk<T> {
  fn full(&self) -> bool {
    self.values.len() == self.values.capacity()
  }

  fn push(&mut self, value: T) -> &T {
    debug_assert!(!self.full());
    self.values.push(value);
    self.values.last().expect("just pushed")
  }
}

impl<T> Default for Arena<T> {
  fn default() -> Self {
    Self {
      chunks: RefCell::new(vec![Chunk::default()]),
    }
  }
}

impl<T> Arena<T> {
  pub fn alloc(&self, value: T) -> &T {
    let ptr: *const T = self.chunk_with_capacity().push(value);

    // We expand the lifetime of the reference to `T`.
    // This is safe because:
    // - The lifetime of the returned reference is bound to this Arena,
    //   so it cannot outlive the Vec holding its data.
    // - We never push more than a chunk's capacity, as initialized
    //   by Vec::with_capacity, so a chunk never reallocates.
    unsafe { &*ptr }
  }

  fn chunk_with_capacity(&self) -> RefMut<'_, Chunk<T>> {
    RefMut::map(self.chunks.borrow_mut(), |chunks| {
      let last_chunk = chunks.last().expect("Arena always has at least one chunk");
      if last_chunk.full() {
        chunks.push(Chunk::default());
      }
      chunks.last_mut().expect("Chunks is non-empty")
    })
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::adt::arena::{Arena, CHUNK_SIZE};

  #[gtest]
  fn test_alloc_new_chunk() {
    let arena = Arena::default();
    let mut refs = Vec::new();
    for i in 0..2 * CHUNK_SIZE {
      refs.push(arena.alloc(i));
    }
    for (i, r) in refs.into_iter().enumerate() {
      expect_eq!(r, &i)
    }
  }
}
